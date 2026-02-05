# M3U Splitter

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/leorossi/m3u-splitter-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/leorossi/m3u-splitter-rs/actions/workflows/ci.yml)

A fast and efficient command-line tool written in Rust that splits M3U playlist files by group names. Organize your streaming channel playlists into separate files based on their `group-title` attribute.

## Features

- 🚀 **Fast**: Built with Rust for optimal performance
- 📁 **Organized**: Automatically splits playlists by group names
- 🔍 **Dry-run mode**: Preview changes before writing files
- 🛡️ **Safe**: Handles edge cases and malformed entries gracefully
- 🌍 **Cross-platform**: Works on Linux, macOS, and Windows
- 📝 **Preserves format**: Maintains original M3U file structure

## Quick Start

```bash
# Build the project
cargo build --release

# Split an M3U file
./target/release/m3u-splitter -i playlist.m3u -o output/

# Preview without writing files
./target/release/m3u-splitter -i playlist.m3u -o output/ --dry-run
```

## CLI Arguments

- `-i, --input`: Full path to the input M3U file (required)
- `-o, --output`: Output directory where split M3U files will be written (required)
- `--dry-run`: Preview mode - shows statistics without writing files (optional)

## How It Works

The M3U format consists of channel entries, each with 2 consecutive lines:
1. **Line 1**: Starts with `#EXTINF:` and contains metadata including a `group-title` field
2. **Line 2**: The stream URL

The tool extracts the `group-title` from each channel and creates separate M3U files for each group in the output directory.

## How-To

### Prerequisites

Make sure you have Rust installed. If not, install it using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:
```bash
rustc --version
cargo --version
```

### Building the Project

Build the project in release mode for optimal performance:

```bash
cargo build --release
```

The executable will be located at `target/release/m3u-splitter` (or `target/release/m3u-splitter.exe` on Windows).

### Usage

#### Basic Usage

Split an M3U file into separate files by group:

```bash
cargo run -- --input /path/to/playlist.m3u --output /path/to/output/directory
```

Or using the compiled binary:

```bash
./target/release/m3u-splitter -i /path/to/playlist.m3u -o /path/to/output/directory
```

#### Dry Run Mode

Preview what will be created without actually writing files:

```bash
cargo run -- --input /path/to/playlist.m3u --output /path/to/output/directory --dry-run
```

This will display:
- The number of groups found
- How many channels are in each group
- No files will be written

#### Example Output

When running the tool, you'll see output like:

```
Parsing M3U file: "/path/to/playlist.m3u"

Found 5 groups:
  Sports: 25 channels
  News: 15 channels
  Movies: 40 channels
  Music: 30 channels
  Kids: 10 channels

Writing output files to: "/path/to/output/directory"
  Created: Sports.m3u (25 channels)
  Created: News.m3u (15 channels)
  Created: Movies.m3u (40 channels)
  Created: Music.m3u (30 channels)
  Created: Kids.m3u (10 channels)

Done!
```

### Output Files

- Each group will be written to a separate `.m3u` file
- Filenames are based on the group name, sanitized for filesystem compatibility
- Files are placed in the specified output directory
- The output directory will be created automatically if it doesn't exist
- Each output file includes the `#EXTM3U` header and all channels for that group

### Notes

- Channels without a `group-title` attribute will be grouped under "Unknown"
- Group names are sanitized for filenames (non-ASCII characters are removed)
- The tool preserves the original M3U format in output files

## Compiling and Distribution

### Building for Release

To create an optimized executable for distribution:

```bash
cargo build --release
```

The compiled binary will be located at:
- **Linux/macOS**: `target/release/m3u-splitter`
- **Windows**: `target/release/m3u-splitter.exe`

### Cross-Compilation

To build for different target platforms, you'll need to install the appropriate target:

#### Linux (from macOS/Windows)

```bash
# Install the target
rustup target add x86_64-unknown-linux-gnu

# Build (requires cross-compilation toolchain)
cargo build --release --target x86_64-unknown-linux-gnu
```

#### Windows (from Linux/macOS)

```bash
# Install the target
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

#### macOS (from Linux/Windows)

```bash
# Install the target
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin  # For Apple Silicon

# Build
cargo build --release --target x86_64-apple-darwin
# or
cargo build --release --target aarch64-apple-darwin
```

**Note**: Cross-compilation may require additional toolchains and dependencies. For Windows targets, you may need MinGW-w64. Consider using Docker or CI/CD for easier cross-compilation.

### Distribution Options

#### Standalone Binary

The simplest distribution method is to provide the standalone binary:

1. Build the release binary for your target platform
2. Optionally strip debug symbols to reduce size:
   ```bash
   strip target/release/m3u-splitter  # Linux/macOS
   ```
3. Compress the binary (optional):
   ```bash
   gzip -c target/release/m3u-splitter > m3u-splitter.gz
   ```

#### Using cargo install

Users can install directly from the source:

```bash
cargo install --path .
```

Or if published to crates.io:

```bash
cargo install m3u-splitter
```

#### Creating Release Packages

For distribution, consider creating platform-specific packages:

- **Linux**: Create a `.tar.gz` or `.deb` package
- **macOS**: Create a `.dmg` or `.pkg` installer
- **Windows**: Create a `.zip` or `.msi` installer

Include the following in your distribution:
- The compiled binary
- README.md with usage instructions
- License file (if applicable)
- Example M3U file (optional, for testing)

### Binary Size Optimization

To reduce the binary size further, you can:

1. Enable link-time optimization in `Cargo.toml`:
   ```toml
   [profile.release]
   lto = true
   codegen-units = 1
   ```

2. Strip debug symbols (as shown above)

3. Use `upx` for compression (may affect compatibility):
   ```bash
   upx --best target/release/m3u-splitter
   ```

## Testing

The project includes comprehensive unit and integration tests:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_test
```

Test coverage includes:
- Group name parsing (double quotes, single quotes, special characters)
- Filename sanitization (non-ASCII removal, special character handling)
- M3U file parsing (basic parsing, missing groups, empty files)
- File writing (basic writing, sanitized names)
- Full workflow integration tests
- Error handling (missing files, empty files)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass: `cargo test`
6. Format code: `cargo fmt`
7. Check linting: `cargo clippy`
8. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI parsing powered by [clap](https://github.com/clap-rs/clap)