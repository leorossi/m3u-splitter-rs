# Contributing to M3U Splitter

Thank you for your interest in contributing to M3U Splitter! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/m3u-splitter.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes: `cargo test`
6. Ensure code formatting: `cargo fmt`
7. Check for linting issues: `cargo clippy`
8. Commit your changes: `git commit -m "Add your meaningful commit message"`
9. Push to your fork: `git push origin feature/your-feature-name`
10. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_test
```

### Code Quality

Before submitting a PR, ensure:

- All tests pass: `cargo test`
- Code is formatted: `cargo fmt --all`
- No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- Code compiles: `cargo build --release`

#### Auto-fixing Formatting Issues

If the formatting check fails, you can automatically fix it:

```bash
# Auto-fix all formatting issues
cargo fmt --all

# Check formatting without fixing
cargo fmt --all -- --check
```

The `--check` flag only checks for formatting issues without modifying files. Remove it to automatically format your code.

## Code Style

- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions focused and small
- Write tests for new functionality

## Commit Messages

- Use clear, descriptive commit messages
- Start with a capital letter
- Use imperative mood ("Add feature" not "Added feature")
- Reference issue numbers if applicable: "Fix #123: Description"

## Pull Request Process

1. Update documentation if needed
2. Add tests for new features
3. Ensure all tests pass
4. Update CHANGELOG.md if applicable
5. Request review from maintainers

## Reporting Bugs

Use the GitHub issue tracker to report bugs. Include:

- Description of the bug
- Steps to reproduce
- Expected vs actual behavior
- Environment (OS, Rust version)
- Sample M3U file if applicable (remove sensitive URLs)

## Feature Requests

We welcome feature requests! Use the GitHub issue tracker and:

- Clearly describe the feature
- Explain the use case
- Discuss potential implementation approaches

## Questions?

Feel free to open an issue for questions or discussions.

Thank you for contributing! 🎉
