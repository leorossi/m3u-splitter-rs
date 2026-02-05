# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release
- CLI interface with input, output, and dry-run options
- M3U file parsing and group extraction
- File splitting by group-title
- Filename sanitization (removes non-ASCII characters)
- Comprehensive test suite (unit and integration tests)
- GitHub Actions CI workflow
- Documentation and contributing guidelines

### Changed
- None

### Fixed
- None

## [0.1.0] - 2025-02-05

### Added
- Basic M3U splitting functionality
- Support for group-title extraction from EXTINF lines
- Dry-run mode for previewing splits
- Error handling for missing files and invalid formats
- Unknown group handling for channels without group-title

[Unreleased]: https://github.com/yourusername/m3u-splitter/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/m3u-splitter/releases/tag/v0.1.0
