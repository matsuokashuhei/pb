# Technical Specification for CLI Command `pb`

## Overview
This document outlines the technical implementation details for the `pb` progress bar CLI command.

## Technology Stack

### Programming Language
**Rust** - Selected for:
- Memory safety and performance
- Excellent cross-platform support
- Strong ecosystem for CLI applications
- Zero-cost abstractions

### Development Environment
**Docker-based Development** - Provides:
- Consistent development environment across platforms
- Isolated dependency management
- Reproducible builds
- Volume caching for fast incremental development

#### Docker Configuration
- **Base Image**: `rust:latest`
- **Development Tools**: rustfmt, clippy, git, vim, curl
- **Volume Mounts**:
  - Source code: `$(pwd):/app`
  - Cargo cache: `pb-cargo-cache:/usr/local/cargo/registry`
  - Target cache: `pb-target-cache:/app/target`
- **Multi-stage Support**: Development, Builder, Production stages

### Dependencies (Crates)

#### Core Dependencies
- **`clap`** (v4.x) - Command-line argument parsing
  - Provides derive API for clean CLI definitions
  - Built-in help generation and validation

- **`chrono`** (v0.4.x) - Date and time handling
  - Robust date/time parsing and manipulation
  - Timezone support
  - Duration calculations

- **`colored`** (v2.x) - Terminal color output
  - Cross-platform color support
  - Simple API for colored text

- **`crossterm`** (v0.27.x) - Cross-platform terminal manipulation
  - Cursor control for progress bar updates
  - Terminal size detection
  - Signal handling

#### Error Handling
- **`anyhow`** (v1.x) - Simplified error handling
  - Ergonomic error propagation
  - Context addition for better error messages

- **`thiserror`** (v1.x) - Custom error types
  - Derive macros for error types
  - Integration with `anyhow`

#### Development Dependencies
- **`assert_cmd`** - Command-line testing
- **`predicates`** - Assertion helpers for tests
- **`tempfile`** - Temporary file handling in tests

## Project Structure

```
pb/
├── Cargo.toml              # Project configuration and dependencies
├── Dockerfile              # Multi-stage Docker configuration
├── README.md               # Project documentation
├── docs/
│   ├── specification.md    # Functional specification
│   ├── technical_specification.md  # This document
│   ├── statement_of_work.md # Project phases and deliverables
│   └── validation_report.md # Development environment validation
├── scripts/
│   ├── build.sh            # Build script (debug/release)
│   ├── test.sh             # Test execution script
│   ├── run.sh              # Application runner script
│   └── dev.sh              # Development utilities
├── src/
│   ├── main.rs            # Application entry point
│   ├── cli.rs             # Command-line interface definition
│   ├── time_parser.rs     # Time parsing and validation
│   ├── progress_bar.rs    # Progress bar rendering logic
│   ├── error.rs           # Custom error types
│   └── lib.rs             # Library exports for testing
└── tests/
    ├── integration_tests.rs  # End-to-end tests
    └── time_parser_tests.rs  # Time parsing unit tests
```

## Development Scripts

### `scripts/build.sh`
- **Purpose**: Docker-based build automation
- **Options**: `--release`, `--verbose`, `--help`
- **Features**: Automatic Docker image management, cargo caching

### `scripts/test.sh`
- **Purpose**: Test execution with multiple options
- **Options**: `--unit`, `--integration`, `--doc`, `--verbose`
- **Features**: Comprehensive test coverage, CI-friendly output

### `scripts/run.sh`
- **Purpose**: Application execution in containerized environment
- **Options**: `--release`, argument pass-through with `--`
- **Features**: TTY support for interactive progress bars

### `scripts/dev.sh`
- **Purpose**: Development utilities and workflow management
- **Commands**: `shell`, `clean`, `deps`, `fmt`, `clippy`, `check`
- **Features**: Interactive development shell, code formatting, linting

## Module Responsibilities

### `main.rs`
- Application entry point
- High-level error handling
- Signal handling setup (Ctrl+C)

### `cli.rs`
- Command-line argument definition using `clap`
- Input validation
- Help message generation

### `time_parser.rs`
- Parse multiple time formats:
  - Date: `2025-07-21`
  - DateTime: `2025-07-21 00:00:00`
  - Relative: `30m`, `2h`, `1d`
- Convert relative times to absolute timestamps
- Validation logic

### `progress_bar.rs`
- Progress calculation
- Bar rendering with fixed 40-character width
- Color management (normal vs overtime)
- Terminal output management

### `error.rs`
- Custom error types using `thiserror`
- Error messages matching specification
- Integration with `anyhow`

## Implementation Details

### Time Parsing Strategy

#### Date Format (`2025-07-21`)
```rust
chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d")
    .map(|date| date.and_hms_opt(0, 0, 0).unwrap())
```

#### DateTime Format (`2025-07-21 00:00:00`)
```rust
chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")
```

#### Relative Time Format (`30m`, `2h`, `1d`)
```rust
regex: r"^(\d+)([mhd])$"
// Convert to seconds then add to current time
```

### Progress Bar Calculation
```rust
fn calculate_progress(start: DateTime, end: DateTime, current: DateTime) -> f64 {
    let total_duration = end - start;
    let elapsed_duration = current - start;

    if total_duration.num_seconds() == 0 {
        return 100.0;
    }

    (elapsed_duration.num_seconds() as f64 / total_duration.num_seconds() as f64) * 100.0
}
```

### Progress Bar Rendering
```rust
const BAR_WIDTH: usize = 40;

fn render_progress_bar(percentage: f64) -> String {
    let filled_chars = ((percentage / 100.0) * BAR_WIDTH as f64) as usize;
    let filled_chars = filled_chars.min(BAR_WIDTH);

    let filled = "█".repeat(filled_chars);
    let empty = " ".repeat(BAR_WIDTH - filled_chars);

    let bar = format!("[{}{}] {:.0}%", filled, empty, percentage);

    if percentage > 100.0 {
        bar.red().to_string()
    } else {
        bar
    }
}
```

### Signal Handling
```rust
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

// In main loop
if event::poll(Duration::from_millis(100))? {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        ..
    }) = event::read()? {
        break; // Exit gracefully
    }
}
```

## Error Handling Strategy

### Custom Error Types
```rust
#[derive(thiserror::Error, Debug)]
pub enum PbError {
    #[error("Start time is later than end time")]
    StartAfterEnd,

    #[error("Invalid time format: {input}")]
    InvalidTimeFormat { input: String },

    #[error("The specified end time has already passed")]
    EndTimeAlreadyPassed,

    #[error("Invalid relative time format: {input}")]
    InvalidRelativeTimeFormat { input: String },

    #[error("--start and --end options are required")]
    MissingRequiredOptions,
}
```

### Error Propagation
- Use `anyhow::Result<T>` for main function results
- Use `anyhow::Context` to add context to errors
- Convert custom errors to `anyhow::Error` when needed

## Testing Strategy

### Unit Tests
- **Time Parser**: Test all time format parsing
- **Progress Calculation**: Test edge cases (0%, 100%, >100%)
- **Error Handling**: Test all error conditions

### Integration Tests
- **CLI Interface**: Test actual command execution
- **End-to-End**: Test complete workflows
- **Error Cases**: Test error message output

### Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_valid_datetime_parsing() {
        // Test implementation
    }

    #[test]
    fn test_progress_calculation() {
        // Test implementation
    }
}
```

## Performance Considerations

### Memory Usage
- Minimal memory footprint
- No unnecessary allocations in render loop
- Efficient string handling

### CPU Usage
- Update only when needed (configurable interval)
- Minimal computation per update cycle
- Efficient terminal I/O

### Terminal Efficiency
- Use crossterm for optimal terminal control
- Minimize screen redraws
- Cursor positioning to avoid flicker

## Cross-Platform Compatibility

### Supported Platforms
- **macOS**: Native support via crossterm
- **Linux**: Full feature support
- **Windows**: Windows Terminal and Command Prompt

### Terminal Compatibility
- ANSI color support detection
- Fallback for terminals without color support
- Unicode character handling for progress bar

## Build Configuration

### Cargo.toml
```toml
[package]
name = "pb"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A CLI progress bar tool for time-based visualization"
license = "MIT"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
colored = "2.0"
crossterm = "0.27"
anyhow = "1.0"
thiserror = "1.0"
regex = "1.0"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.0"

[[bin]]
name = "pb"
path = "src/main.rs"
```

### Release Optimization
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

## Future Enhancements

### Potential Features
- Configuration file support
- Multiple progress bar themes
- Sound notifications
- Integration with system notifications
- JSON output mode for scripting

### Architecture Extensibility
- Plugin system for custom renderers
- Configurable time formats
- Custom progress calculation algorithms
