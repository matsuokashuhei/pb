# GitHub Copilot Instructions for pmon

## Project Overview
`pmon` is a Rust CLI tool for time-based progress visualization. Users provide start/end times, and it displays real-time progress bars with colored output. The architecture is modular with clear separation between parsing, calculation, and rendering.

## Core Architecture

### Module Structure (`src/`)
- **`main.rs`**: Application entry point with TTY detection and signal handling
- **`cli.rs`**: CLI parsing using `clap` derive API with custom error handling
- **`time_parser.rs`**: Multi-format time parsing (dates, datetimes, relative time like "2h", "30m")
- **`progress_bar.rs`**: Progress calculation and colored terminal rendering
- **`error.rs`**: Custom error types using `thiserror` with `anyhow` integration

### Key Design Patterns
- **Error Handling**: Uses `PbError` enum with `thiserror` for structured errors, propagated via `anyhow::Result`
- **Time Consistency**: All timestamps are `NaiveDateTime` in local time via `get_current_time()`
- **TTY Awareness**: Different output behavior for terminals vs pipes using `crossterm::tty::IsTty`
- **Signal Handling**: Raw mode cleanup with panic hooks for graceful Ctrl+C exit

## Development Workflow

### Essential Commands
```bash
# Development with file watching
cargo install cargo-watch
cargo watch -x "test --lib" -x "clippy" -x "fmt"

# Using project scripts (prefer over direct cargo)
./scripts/build.sh --release --target x86_64-apple-darwin
./scripts/test.sh --unit --verbose
./scripts/run.sh -- --start "2025-01-27 09:00:00" --end "8h"
```

### Testing Strategy
- **Unit tests**: In `src/` modules using `#[cfg(test)]`
- **Integration tests**: In `tests/` directory with `common/helpers.rs` shared utilities
- **CLI tests**: Using `assert_cmd` crate for end-to-end command testing
- **Test data**: `TimeTestData` helper in `tests/common/helpers.rs` for consistent test scenarios

## Code Conventions

### Time Handling
- Always use `get_current_time()` for current time (ensures local timezone consistency)
- Parse times via `parse_time()` which delegates to specific parsers based on format detection
- Relative times use regex patterns: `(\d+)([smhd])` for seconds/minutes/hours/days

### Error Patterns
```rust
// Custom errors with context
return Err(PbError::invalid_time_format(input));

// Error propagation in functions
pub fn parse_something(input: &str) -> PbResult<NaiveDateTime> {
    // Implementation
}
```

### Progress Calculation
- Fixed `BAR_WIDTH = 40` for consistent terminal output
- Color thresholds: Green (0-80%), Yellow (80-100%), Red (>100%)
- Handle edge cases: zero duration returns 100%, negative progress clamped to 0%

## Build & CI

### Multi-target Builds
The project builds for Linux (x86_64, ARM64) and macOS (Intel, Apple Silicon). Cross-compilation setup in CI uses platform-specific linkers and `rustup target add`.

### Performance Profile
Release builds use aggressive optimization (`opt-level = 3`, LTO, single codegen unit) for minimal binary size and fast startup (<50ms, <10MB memory).

### Testing Coverage
CI runs comprehensive test suite across stable/beta/nightly Rust on Ubuntu/macOS with `cargo-tarpaulin` for coverage reporting to Codecov.

## Integration Points

### External Dependencies
- **`clap`**: CLI parsing with derive API
- **`chrono`**: All datetime operations, parsing, and formatting
- **`crossterm`**: Cross-platform terminal handling and TTY detection
- **`colored`**: Terminal color output with RGB support

### Output Modes
- **TTY Mode**: Interactive progress bars with real-time updates
- **Pipe Mode**: Line-by-line output for scripting integration
- **Background Mode**: Suitable for logging and monitoring scripts

## AI Coding Guidelines

### Operational Quality (Testing)
- **Test Coverage**: Generate code that passes all specified unit tests with 90%+ statement coverage
- **TDD Approach**: Write tests first, then implementation code following the existing test patterns in `tests/`
- **Coverage Standards**: Use `cargo tarpaulin` to verify coverage meets project standards before submitting code
- **Test Structure**: Follow the established pattern of unit tests in `src/` modules and integration tests in `tests/` directory

### Functional Requirements
- **Specification Compliance**: Implement features that strictly comply with documented requirements and user scenarios
- **BDD Patterns**: When user scenarios are provided, structure tests using the existing `TimeTestData` helper patterns
- **Requirement Validation**: Ensure all features satisfy the documented CLI interface and time parsing specifications

### Code Quality
- **Rust Standards**: Adhere to `rustfmt` and `clippy` configurations - all code must pass `cargo clippy --all-targets --all-features`
- **Static Analysis**: Generate code that passes the project's quality gates and CI checks
- **Codebase Patterns**: Follow existing patterns for error handling (`PbError` enum), time handling (`get_current_time()`), and module organization
- **Maintainability**: Write code that integrates seamlessly with the modular architecture and existing conventions

### Security
- **Vulnerability Prevention**: Generate code free from common security issues, especially in time parsing and CLI input handling
- **Input Validation**: Follow existing patterns for robust input validation in `time_parser.rs` and `cli.rs`
- **Safe Practices**: Use Rust's memory safety features and avoid unsafe code blocks unless absolutely necessary
- **Dependency Security**: Ensure any new dependencies align with the project's security standards

### General Approach
- **Pattern Analysis**: Study existing codebase patterns in `src/` modules before implementing new features
- **Architecture Integration**: Ensure new code integrates seamlessly with the current modular structure
- **Production Standards**: All code must meet the performance requirements (<50ms startup, <10MB memory) and pass CI checks

When contributing, follow the existing error handling patterns, maintain timezone consistency, and ensure all new time parsing functions handle edge cases with comprehensive tests.
