# AGENTS.md - Project Context and Setup Guide

## Project Overview

This is a Rust command-line tool that splits M3U playlist files based on group names. M3U files contain streaming channel information, and this tool organizes channels by their group-name field into separate output files.

## Project Status

**Current State**: ✅ **Project is fully implemented and functional**. The tool is complete with:
- Full CLI implementation using `clap` for argument parsing
- M3U file parsing with group-title extraction
- File splitting functionality with proper error handling
- Comprehensive test suite (unit tests and integration tests)
- Complete documentation (README.md, CONTRIBUTING.md, CHANGELOG.md)
- CI/CD setup (GitHub Actions)
- Release-ready binary builds

The tool is ready for use and can be built with `cargo build --release`.

## Requirements

### Rust Installation

**Important**: The user may not have Rust installed or may be unsure of their Rust version.

**Recommended Approach**:
1. Check if Rust is installed: `rustc --version` or `cargo --version`
2. If not installed, use rustup (the official Rust installer):
   - Visit: https://rustup.rs/
   - Or run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. **Rust Version**: This project should work with Rust 1.70+ (stable). If the user has an older version, suggest updating via `rustup update stable`

### Project Dependencies

The project uses:
- **clap** (v4.5): CLI argument parsing with derive features
- **tempfile** (v3.8): For integration tests (dev-dependency)
- Standard Rust std library for file I/O and string parsing

All dependencies are specified in `Cargo.toml`.

## M3U File Format

The tool expects M3U files with the following structure:
- Each channel consists of 2 consecutive lines:
  1. **Line 1**: Starts with `#EXTINF:` and contains metadata including a `group-name` field
  2. **Line 2**: The stream URL

Example:
```
#EXTINF:-1 group-title="Sports" tvg-id="channel1",Channel Name
http://example.com/stream1.m3u8
#EXTINF:-1 group-title="News" tvg-id="channel2",Another Channel
http://example.com/stream2.m3u8
```

## CLI Interface

The tool accepts the following arguments:

- `-i` or `--input`: Full path to the input M3U file (required)
- `-o` or `--output`: Output directory where split M3U files will be written (required)
- `--dry-run`: Optional flag. When present, only displays statistics (number of channels per group) without writing files

## Expected Behavior

1. Parse the input M3U file
2. Extract the `group-name` field from each `#EXTINF` line
3. Group channels by their `group-name`
4. Create separate M3U files for each group in the output directory
5. If `--dry-run` is specified, only print statistics

## Implementation Notes

### Key Considerations

1. **M3U Parsing**: Need to handle:
   - Parsing `#EXTINF` lines to extract `group-name` attribute
   - Handling malformed lines gracefully
   - Preserving the original format when writing output files

2. **File Naming**: Output files should be named appropriately (e.g., based on group name, sanitized for filesystem compatibility)

3. **Error Handling**: 
   - File not found errors
   - Invalid M3U format
   - Permission errors when writing to output directory
   - Missing group-name fields

4. **CLI Parsing**: Use a crate like `clap` for robust argument parsing with proper help text

5. **Output Directory**: Should create the directory if it doesn't exist

## Development Workflow

✅ **Completed**: All core functionality has been implemented:
1. ✅ Rust project initialized with `cargo init --name m3u-splitter`
2. ✅ Dependencies added to `Cargo.toml` (clap, tempfile)
3. ✅ CLI parsing implemented in `main.rs` using clap derive macros
4. ✅ M3U parsing logic implemented with group-title extraction
5. ✅ File writing logic implemented with filename sanitization
6. ✅ Comprehensive test suite created (unit + integration tests)

### Current Development Commands

```bash
# Build the project
cargo build --release

# Run tests
cargo test

# Run with output
cargo test -- --nocapture

# Format code
cargo fmt

# Check linting
cargo clippy

# Run the tool
cargo run -- --input <file> --output <dir> [--dry-run]
```

## Testing Strategy

- Create sample M3U files with multiple groups
- Test with `--dry-run` flag
- Test with various edge cases (missing group-name, empty groups, etc.)
- Verify output files are correctly formatted

## File Structure

```
.
├── Cargo.toml              # Rust project manifest with dependencies
├── Cargo.lock              # Dependency lock file
├── src/
│   └── main.rs            # Main application code (fully implemented)
├── tests/
│   └── integration_test.rs # Integration tests
├── target/                 # Build artifacts (gitignored)
├── README.md               # Comprehensive project documentation
├── CONTRIBUTING.md         # Contribution guidelines
├── CHANGELOG.md            # Version history
├── LICENSE                 # MIT License
└── AGENTS.md              # This file
```

## Implementation Status

### ✅ Completed Features

- [x] Rust project initialization
- [x] CLI argument parsing (clap with derive macros)
- [x] M3U file parsing with group-title extraction
- [x] Support for both single and double quotes in group-title
- [x] Filename sanitization (handles non-ASCII, special characters)
- [x] File writing with proper M3U format preservation
- [x] Dry-run mode for previewing splits
- [x] Error handling (missing files, empty files, malformed entries)
- [x] Unknown group handling (channels without group-title)
- [x] Comprehensive unit tests
- [x] Integration tests
- [x] Documentation (README, CONTRIBUTING, CHANGELOG)
- [x] CI/CD setup

### 🔄 Future Enhancements (Optional)

Potential improvements that could be added:
- Progress bar for large files
- Custom output filename patterns
- Filtering options (include/exclude specific groups)
- JSON/CSV output format options
- Parallel processing for very large files
- Support for additional M3U metadata preservation
