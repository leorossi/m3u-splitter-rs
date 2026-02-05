use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "m3u-splitter")]
#[command(about = "Splits M3U playlist files by group-name")]
struct Args {
    /// Input M3U file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for split M3U files
    #[arg(short, long)]
    output: PathBuf,

    /// Dry run: only show statistics without writing files
    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug)]
struct Channel {
    extinf_line: String,
    url: String,
    group_name: String,
}

fn parse_group_name(extinf_line: &str) -> Option<String> {
    // Look for group-title="..." or group-title='...'
    // Try double quotes first
    if let Some(start) = extinf_line.find("group-title=\"") {
        let start = start + "group-title=\"".len();
        if let Some(end) = extinf_line[start..].find('"') {
            return Some(extinf_line[start..start + end].to_string());
        }
    }

    // Try single quotes
    if let Some(start) = extinf_line.find("group-title='") {
        let start = start + "group-title='".len();
        if let Some(end) = extinf_line[start..].find('\'') {
            return Some(extinf_line[start..start + end].to_string());
        }
    }

    None
}

fn parse_m3u_file(input_path: &Path) -> io::Result<Vec<Channel>> {
    let file = fs::File::open(input_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut channels = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.starts_with("#EXTINF:") {
            if i + 1 < lines.len() {
                let extinf_line = line.to_string();
                let url = lines[i + 1].trim().to_string();

                let group_name =
                    parse_group_name(&extinf_line).unwrap_or_else(|| "Unknown".to_string());

                channels.push(Channel {
                    extinf_line,
                    url,
                    group_name,
                });

                i += 2;
            } else {
                // EXTINF line without URL, skip it
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    Ok(channels)
}

fn sanitize_filename(group_name: &str) -> String {
    // Remove non-ASCII characters and keep only safe filesystem characters
    group_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || *c == ' ')
        .collect::<String>()
        .trim()
        .replace(' ', "_")
}

fn write_group_file(output_dir: &Path, group_name: &str, channels: &[Channel]) -> io::Result<()> {
    let sanitized_name = sanitize_filename(group_name);
    let filename = format!("{}.m3u", sanitized_name);
    let filepath = output_dir.join(&filename);

    let mut file = fs::File::create(&filepath)?;

    // Write M3U header
    writeln!(file, "#EXTM3U")?;

    // Write each channel
    for channel in channels {
        writeln!(file, "{}", channel.extinf_line)?;
        writeln!(file, "{}", channel.url)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Validate input file exists
    if !args.input.exists() {
        eprintln!("Error: Input file does not exist: {:?}", args.input);
        std::process::exit(1);
    }

    // Parse M3U file
    println!("Parsing M3U file: {:?}", args.input);
    let channels = parse_m3u_file(&args.input)?;

    if channels.is_empty() {
        eprintln!("Warning: No channels found in the M3U file");
        return Ok(());
    }

    // Group channels by group-name
    let mut groups: HashMap<String, Vec<Channel>> = HashMap::new();
    for channel in channels {
        groups
            .entry(channel.group_name.clone())
            .or_default()
            .push(channel);
    }

    // Display statistics
    println!("\nFound {} groups:", groups.len());
    for (group_name, channels) in &groups {
        println!("  {}: {} channels", group_name, channels.len());
    }

    if args.dry_run {
        println!("\nDry-run mode: No files written.");
        return Ok(());
    }

    // Create output directory if it doesn't exist
    fs::create_dir_all(&args.output)?;

    // Write output files
    println!("\nWriting output files to: {:?}", args.output);
    for (group_name, channels) in groups {
        write_group_file(&args.output, &group_name, &channels)?;
        println!(
            "  Created: {}.m3u ({} channels)",
            sanitize_filename(&group_name),
            channels.len()
        );
    }

    println!("\nDone!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parse_group_name_double_quotes() {
        let line = r#"#EXTINF:-1 group-title="Sports" tvg-id="channel1",Channel Name"#;
        assert_eq!(parse_group_name(line), Some("Sports".to_string()));
    }

    #[test]
    fn test_parse_group_name_single_quotes() {
        let line = r#"#EXTINF:-1 group-title='News' tvg-id="channel2",Another Channel"#;
        assert_eq!(parse_group_name(line), Some("News".to_string()));
    }

    #[test]
    fn test_parse_group_name_with_spaces() {
        let line = r#"#EXTINF:-1 group-title="Kids & Family" tvg-id="channel3",Kids Channel"#;
        assert_eq!(parse_group_name(line), Some("Kids & Family".to_string()));
    }

    #[test]
    fn test_parse_group_name_missing() {
        let line = r#"#EXTINF:-1 tvg-id="channel4",Channel Without Group"#;
        assert_eq!(parse_group_name(line), None);
    }

    #[test]
    fn test_parse_group_name_empty() {
        let line = r#"#EXTINF:-1 group-title="" tvg-id="channel5",Empty Group"#;
        assert_eq!(parse_group_name(line), Some("".to_string()));
    }

    #[test]
    fn test_parse_group_name_special_characters() {
        let line = r#"#EXTINF:-1 group-title="Café & Música" tvg-id="channel6",Special"#;
        assert_eq!(parse_group_name(line), Some("Café & Música".to_string()));
    }

    #[test]
    fn test_sanitize_filename_simple() {
        assert_eq!(sanitize_filename("Sports"), "Sports");
    }

    #[test]
    fn test_sanitize_filename_with_spaces() {
        assert_eq!(sanitize_filename("Kids & Family"), "Kids__Family");
    }

    #[test]
    fn test_sanitize_filename_non_ascii() {
        assert_eq!(sanitize_filename("Café"), "Caf");
        assert_eq!(sanitize_filename("Música"), "Msica");
        assert_eq!(sanitize_filename("北京"), "");
    }

    #[test]
    fn test_sanitize_filename_special_chars() {
        assert_eq!(sanitize_filename("Group/Name"), "GroupName");
        assert_eq!(sanitize_filename("Group\\Name"), "GroupName");
        assert_eq!(sanitize_filename("Group*Name"), "GroupName");
    }

    #[test]
    fn test_sanitize_filename_leading_trailing_spaces() {
        assert_eq!(sanitize_filename("  Sports  "), "Sports");
    }

    #[test]
    fn test_sanitize_filename_dashes_and_underscores() {
        assert_eq!(sanitize_filename("group-name"), "group-name");
        assert_eq!(sanitize_filename("group_name"), "group_name");
        assert_eq!(sanitize_filename("group-name_test"), "group-name_test");
    }

    #[test]
    fn test_parse_m3u_file_basic() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.m3u");

        let content = r#"#EXTM3U
#EXTINF:-1 group-title="Sports" tvg-id="channel1",Sports Channel 1
http://example.com/sports1.m3u8
#EXTINF:-1 group-title="News" tvg-id="channel2",News Channel 1
http://example.com/news1.m3u8
#EXTINF:-1 group-title="Sports" tvg-id="channel3",Sports Channel 2
http://example.com/sports2.m3u8
"#;

        fs::write(&test_file, content).unwrap();

        let channels = parse_m3u_file(&test_file).unwrap();
        assert_eq!(channels.len(), 3);
        assert_eq!(channels[0].group_name, "Sports");
        assert_eq!(channels[0].url, "http://example.com/sports1.m3u8");
        assert_eq!(channels[1].group_name, "News");
        assert_eq!(channels[2].group_name, "Sports");
    }

    #[test]
    fn test_parse_m3u_file_missing_group() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.m3u");

        let content = r#"#EXTM3U
#EXTINF:-1 tvg-id="channel1",Channel Without Group
http://example.com/channel1.m3u8
"#;

        fs::write(&test_file, content).unwrap();

        let channels = parse_m3u_file(&test_file).unwrap();
        assert_eq!(channels.len(), 1);
        assert_eq!(channels[0].group_name, "Unknown");
    }

    #[test]
    fn test_parse_m3u_file_empty() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.m3u");

        fs::write(&test_file, "#EXTM3U\n").unwrap();

        let channels = parse_m3u_file(&test_file).unwrap();
        assert_eq!(channels.len(), 0);
    }

    #[test]
    fn test_write_group_file() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();

        let channels = vec![
            Channel {
                extinf_line: r#"#EXTINF:-1 group-title="Sports" tvg-id="channel1",Sports Channel"#
                    .to_string(),
                url: "http://example.com/sports.m3u8".to_string(),
                group_name: "Sports".to_string(),
            },
            Channel {
                extinf_line:
                    r#"#EXTINF:-1 group-title="Sports" tvg-id="channel2",Sports Channel 2"#
                        .to_string(),
                url: "http://example.com/sports2.m3u8".to_string(),
                group_name: "Sports".to_string(),
            },
        ];

        write_group_file(output_dir, "Sports", &channels).unwrap();

        let output_file = output_dir.join("Sports.m3u");
        assert!(output_file.exists());

        let content = fs::read_to_string(&output_file).unwrap();
        assert!(content.starts_with("#EXTM3U\n"));
        assert!(content.contains("http://example.com/sports.m3u8"));
        assert!(content.contains("http://example.com/sports2.m3u8"));
    }

    #[test]
    fn test_write_group_file_sanitized_name() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();

        let channels = vec![Channel {
            extinf_line: r#"#EXTINF:-1 group-title="Kids & Family" tvg-id="channel1",Kids Channel"#
                .to_string(),
            url: "http://example.com/kids.m3u8".to_string(),
            group_name: "Kids & Family".to_string(),
        }];

        write_group_file(output_dir, "Kids & Family", &channels).unwrap();

        let output_file = output_dir.join("Kids__Family.m3u");
        assert!(output_file.exists());
    }

    #[test]
    fn test_write_group_file_non_ascii_name() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();

        let channels = vec![Channel {
            extinf_line: r#"#EXTINF:-1 group-title="Café" tvg-id="channel1",Cafe Channel"#
                .to_string(),
            url: "http://example.com/cafe.m3u8".to_string(),
            group_name: "Café".to_string(),
        }];

        write_group_file(output_dir, "Café", &channels).unwrap();

        let output_file = output_dir.join("Caf.m3u");
        assert!(output_file.exists());
    }
}
