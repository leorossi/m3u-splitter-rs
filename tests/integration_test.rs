use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn get_binary_path() -> String {
    // Try release first, then debug
    if Path::new("target/release/m3u-splitter").exists() {
        "target/release/m3u-splitter".to_string()
    } else if Path::new("target/release/m3u-splitter.exe").exists() {
        "target/release/m3u-splitter.exe".to_string()
    } else if Path::new("target/debug/m3u-splitter").exists() {
        "target/debug/m3u-splitter".to_string()
    } else if Path::new("target/debug/m3u-splitter.exe").exists() {
        "target/debug/m3u-splitter.exe".to_string()
    } else {
        panic!("Binary not found. Run 'cargo build' first.");
    }
}

#[test]
fn test_basic_splitting() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.m3u");
    let output_dir = temp_dir.path().join("output");

    // Create test M3U file
    let m3u_content = r#"#EXTM3U
#EXTINF:-1 group-title="Sports" tvg-id="sports1",Sports Channel 1
http://example.com/sports1.m3u8
#EXTINF:-1 group-title="News" tvg-id="news1",News Channel 1
http://example.com/news1.m3u8
#EXTINF:-1 group-title="Sports" tvg-id="sports2",Sports Channel 2
http://example.com/sports2.m3u8
#EXTINF:-1 group-title="Movies" tvg-id="movie1",Movie Channel 1
http://example.com/movie1.m3u8
"#;

    fs::write(&input_file, m3u_content).unwrap();

    // Run the binary
    let binary = get_binary_path();
    let output = Command::new(binary)
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify output files were created
    let sports_file = output_dir.join("Sports.m3u");
    let news_file = output_dir.join("News.m3u");
    let movies_file = output_dir.join("Movies.m3u");

    assert!(sports_file.exists(), "Sports.m3u should exist");
    assert!(news_file.exists(), "News.m3u should exist");
    assert!(movies_file.exists(), "Movies.m3u should exist");

    // Verify Sports.m3u content
    let sports_content = fs::read_to_string(&sports_file).unwrap();
    assert!(sports_content.starts_with("#EXTM3U"));
    assert_eq!(
        sports_content.matches("http://example.com/sports").count(),
        2
    );

    // Verify News.m3u content
    let news_content = fs::read_to_string(&news_file).unwrap();
    assert!(news_content.starts_with("#EXTM3U"));
    assert_eq!(news_content.matches("http://example.com/news").count(), 1);

    // Verify Movies.m3u content
    let movies_content = fs::read_to_string(&movies_file).unwrap();
    assert!(movies_content.starts_with("#EXTM3U"));
    assert_eq!(
        movies_content.matches("http://example.com/movie").count(),
        1
    );
}

#[test]
fn test_dry_run_mode() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.m3u");
    let output_dir = temp_dir.path().join("output");

    // Create test M3U file
    let m3u_content = r#"#EXTM3U
#EXTINF:-1 group-title="Sports" tvg-id="sports1",Sports Channel 1
http://example.com/sports1.m3u8
#EXTINF:-1 group-title="News" tvg-id="news1",News Channel 1
http://example.com/news1.m3u8
"#;

    fs::write(&input_file, m3u_content).unwrap();

    // Run the binary with --dry-run
    let binary = get_binary_path();
    let output = Command::new(binary)
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_dir)
        .arg("--dry-run")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    // Verify no files were created
    assert!(
        !output_dir.exists(),
        "Output directory should not exist in dry-run mode"
    );

    // Verify output contains statistics
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let all_output = format!("{}{}", stdout, stderr);

    assert!(
        all_output.contains("Found"),
        "Output should contain group statistics. stdout: {:?}, stderr: {:?}",
        stdout,
        stderr
    );
    assert!(
        all_output.contains("Sports"),
        "Output should mention Sports group"
    );
    assert!(
        all_output.contains("News"),
        "Output should mention News group"
    );
    assert!(
        all_output.contains("Dry-run")
            || all_output.contains("dry-run")
            || all_output.contains("Dry-run mode"),
        "Output should mention dry-run mode. stdout: {:?}, stderr: {:?}",
        stdout,
        stderr
    );
}

#[test]
fn test_missing_group_title() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.m3u");
    let output_dir = temp_dir.path().join("output");

    // Create test M3U file with missing group-title
    let m3u_content = r#"#EXTM3U
#EXTINF:-1 tvg-id="channel1",Channel Without Group
http://example.com/channel1.m3u8
#EXTINF:-1 group-title="Sports" tvg-id="sports1",Sports Channel
http://example.com/sports1.m3u8
"#;

    fs::write(&input_file, m3u_content).unwrap();

    // Run the binary
    let binary = get_binary_path();
    let output = Command::new(binary)
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    // Verify Unknown.m3u was created
    let unknown_file = output_dir.join("Unknown.m3u");
    assert!(
        unknown_file.exists(),
        "Unknown.m3u should exist for channels without group-title"
    );

    // Verify Sports.m3u was created
    let sports_file = output_dir.join("Sports.m3u");
    assert!(sports_file.exists(), "Sports.m3u should exist");
}

#[test]
fn test_special_characters_in_group_name() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.m3u");
    let output_dir = temp_dir.path().join("output");

    // Create test M3U file with special characters
    let m3u_content = r#"#EXTM3U
#EXTINF:-1 group-title="Kids & Family" tvg-id="kids1",Kids Channel
http://example.com/kids1.m3u8
#EXTINF:-1 group-title="Café" tvg-id="cafe1",Cafe Channel
http://example.com/cafe1.m3u8
"#;

    fs::write(&input_file, m3u_content).unwrap();

    // Run the binary
    let binary = get_binary_path();
    let output = Command::new(binary)
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    // Verify sanitized filenames
    let kids_file = output_dir.join("Kids__Family.m3u");
    let cafe_file = output_dir.join("Caf.m3u");

    assert!(kids_file.exists(), "Kids__Family.m3u should exist");
    assert!(
        cafe_file.exists(),
        "Caf.m3u should exist (non-ASCII removed)"
    );
}

#[test]
fn test_nonexistent_input_file() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("nonexistent.m3u");
    let output_dir = temp_dir.path().join("output");

    // Run the binary with non-existent file
    let binary = get_binary_path();
    let output = Command::new(binary)
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("Failed to execute command");

    assert!(
        !output.status.success(),
        "Command should fail for non-existent file"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("does not exist") || stderr.contains("Error"),
        "Error message should indicate file doesn't exist"
    );
}

#[test]
fn test_empty_m3u_file() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.m3u");
    let output_dir = temp_dir.path().join("output");

    // Create empty M3U file (just header)
    fs::write(&input_file, "#EXTM3U\n").unwrap();

    // Run the binary
    let binary = get_binary_path();
    let output = Command::new(binary)
        .arg("--input")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("Failed to execute command");

    // Should succeed but warn about no channels
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Either success with warning or exit with warning
    assert!(
        stdout.contains("Warning")
            || stderr.contains("Warning")
            || stdout.contains("No channels")
            || stderr.contains("No channels"),
        "Should warn about empty file"
    );
}
