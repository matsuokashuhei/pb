# Development Guide

This guide is for developers who want to contribute to pb or understand its internal architecture.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Project Architecture](#project-architecture)
- [Code Organization](#code-organization)
- [Development Workflow](#development-workflow)
- [Testing Guidelines](#testing-guidelines)
- [Contributing Guidelines](#contributing-guidelines)
- [Release Process](#release-process)
- [Performance Considerations](#performance-considerations)

## Getting Started

### Prerequisites

- **Rust**: Version 1.70 or later
- **Git**: For version control
- **Docker**: For consistent development environment (optional but recommended)
- **A text editor or IDE**: VS Code with rust-analyzer recommended

### Development Setup

1. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/pb.git
   cd pb
   ```

2. **Set up the development environment**:
   ```bash
   # Using Docker (recommended)
   ./scripts/dev.sh shell
   
   # Or install Rust locally
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **Install development dependencies**:
   ```bash
   # Install additional Rust components
   rustup component add rustfmt clippy
   
   # Install cargo-watch for development
   cargo install cargo-watch
   ```

4. **Verify the setup**:
   ```bash
   # Build the project
   ./scripts/build.sh
   
   # Run tests
   ./scripts/test.sh
   
   # Run the application
   ./scripts/run.sh -- --help
   ```

## Development Environment

### Docker-based Development

The project includes a comprehensive Docker setup for consistent development across platforms.

#### Development Scripts

| Script | Purpose | Usage |
|--------|---------|-------|
| `./scripts/build.sh` | Build the project | `./scripts/build.sh [--release] [--verbose]` |
| `./scripts/test.sh` | Run tests | `./scripts/test.sh [--unit] [--integration] [--verbose]` |
| `./scripts/run.sh` | Run the application | `./scripts/run.sh [--release] -- [ARGS]` |
| `./scripts/dev.sh` | Development utilities | `./scripts/dev.sh [shell\|clean\|fmt\|clippy\|check\|deps]` |

#### Common Development Tasks

```bash
# Start an interactive development shell
./scripts/dev.sh shell

# Format code
./scripts/dev.sh fmt

# Run linter
./scripts/dev.sh clippy

# Quick syntax check
./scripts/dev.sh check

# Clean build artifacts
./scripts/dev.sh clean

# Update dependencies
./scripts/dev.sh deps
```

### Local Development (without Docker)

If you prefer to develop without Docker:

```bash
# Build and test
cargo build
cargo test

# Format and lint
cargo fmt
cargo clippy

# Watch for changes and rebuild
cargo watch -x build

# Watch for changes and run tests
cargo watch -x test
```

## Project Architecture

### High-Level Overview

pb follows a modular architecture with clear separation of concerns:

```
pb/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library interface
│   ├── cli.rs            # Command-line interface
│   ├── error.rs          # Error handling
│   ├── progress_bar.rs   # Progress calculation and rendering
│   └── time_parser.rs    # Time parsing logic
├── tests/               # Integration tests
├── docs/               # Documentation
└── scripts/            # Development scripts
```

### Core Components

#### 1. CLI Module (`src/cli.rs`)
- Handles command-line argument parsing using `clap`
- Defines the CLI interface and validation
- Provides structured access to user input

#### 2. Time Parser Module (`src/time_parser.rs`)
- Parses various time formats (date, datetime, relative)
- Validates time inputs and relationships
- Handles edge cases and error reporting

#### 3. Progress Bar Module (`src/progress_bar.rs`)
- Calculates progress between time points
- Renders progress bars with colors and formatting
- Handles different output modes (TTY vs non-TTY)

#### 4. Error Handling (`src/error.rs`)
- Defines custom error types using `thiserror`
- Provides consistent error reporting
- Integrates with `anyhow` for error context

#### 5. Main Application (`src/main.rs`)
- Orchestrates the main application loop
- Handles terminal state management
- Implements graceful shutdown

### Dependencies

#### Core Dependencies
- **clap**: Command-line argument parsing
- **chrono**: Date and time handling
- **colored**: Terminal color output
- **crossterm**: Cross-platform terminal manipulation
- **anyhow**: Error handling and context
- **thiserror**: Custom error types
- **regex**: Pattern matching for time parsing

#### Development Dependencies
- **assert_cmd**: Command-line testing
- **predicates**: Test assertions
- **tempfile**: Temporary file handling in tests

## Code Organization

### Naming Conventions

- **Functions**: Use `snake_case`
- **Types**: Use `PascalCase`
- **Constants**: Use `SCREAMING_SNAKE_CASE`
- **Modules**: Use `snake_case`

### File Structure Guidelines

```rust
// Each module should follow this structure:

//! Module documentation
//!
//! Brief description of the module's purpose and functionality.

use std::...;       // Standard library imports
use external_crate::...; // External crate imports
use crate::...;     // Internal imports

// Type definitions
pub struct SomeType {
    // Fields
}

// Implementation blocks
impl SomeType {
    // Associated functions and methods
}

// Free functions
pub fn some_function() -> Result<()> {
    // Implementation
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_some_function() {
        // Test implementation
    }
}
```

### Error Handling Patterns

```rust
// Use custom error types for domain-specific errors
#[derive(Debug, thiserror::Error)]
pub enum PbError {
    #[error("Invalid time format: {input}")]
    InvalidTimeFormat { input: String },
    
    #[error("Time validation error: {message}")]
    TimeValidation { message: String },
}

// Use Result types for fallible operations
pub fn parse_time(input: &str) -> PbResult<NaiveDateTime> {
    // Implementation with proper error handling
}

// Provide context using anyhow
pub fn main() -> Result<()> {
    let result = parse_time(input)
        .context("Failed to parse start time")?;
    Ok(())
}
```

## Development Workflow

### Branch Management

1. **Main Branch**: `main` - stable, release-ready code
2. **Feature Branches**: `feature/description` - new features
3. **Bug Fixes**: `fix/description` - bug fixes
4. **Documentation**: `docs/description` - documentation updates

### Commit Messages

Follow conventional commit format:
```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test additions or modifications
- `chore`: Maintenance tasks

Examples:
```
feat(time_parser): add support for ISO 8601 format

fix(progress_bar): correct percentage calculation for edge cases

docs(user_guide): add troubleshooting section
```

### Development Process

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/new-time-format
   ```

2. **Make changes with tests**:
   - Write tests first (TDD approach recommended)
   - Implement the feature
   - Ensure all tests pass

3. **Validate code quality**:
   ```bash
   ./scripts/dev.sh fmt     # Format code
   ./scripts/dev.sh clippy  # Run linter
   ./scripts/test.sh        # Run all tests
   ```

4. **Commit changes**:
   ```bash
   git add .
   git commit -m "feat(time_parser): add ISO 8601 support"
   ```

5. **Push and create PR**:
   ```bash
   git push origin feature/new-time-format
   # Create pull request on GitHub
   ```

## Testing Guidelines

### Test Organization

```
tests/
├── common/
│   └── helpers.rs        # Shared test utilities
├── cli_tests.rs          # CLI interface tests
├── time_parser_tests.rs  # Time parsing tests
├── progress_tests.rs     # Progress calculation tests
└── performance_tests.rs  # Performance benchmarks
```

### Test Categories

#### 1. Unit Tests
Located within each module using `#[cfg(test)]`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // Test individual functions
    }
}
```

#### 2. Integration Tests
Located in the `tests/` directory:
```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}
```

#### 3. Performance Tests
Benchmark critical functions:
```rust
#[test]
fn test_time_parsing_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    for _ in 0..1000 {
        parse_time("2025-01-27 12:00:00").unwrap();
    }
    let duration = start.elapsed();
    
    // Assert performance criteria
    assert!(duration.as_millis() < 100);
}
```

### Test Best Practices

1. **Use descriptive test names** that explain what is being tested
2. **Follow AAA pattern**: Arrange, Act, Assert
3. **Test edge cases** and error conditions
4. **Use property-based testing** for complex logic
5. **Mock external dependencies** where appropriate

### Running Tests

```bash
# Run all tests
./scripts/test.sh

# Run specific test categories
./scripts/test.sh --unit        # Unit tests only
./scripts/test.sh --integration # Integration tests only

# Run with verbose output
./scripts/test.sh --verbose

# Run specific test file
cargo test time_parser_tests

# Run specific test function
cargo test test_parse_datetime_valid_cases
```

## Contributing Guidelines

### Before Contributing

1. **Check existing issues** to avoid duplicate work
2. **Create an issue** for significant changes
3. **Read the code of conduct** and contribution guidelines
4. **Set up the development environment** properly

### Pull Request Process

1. **Fork the repository** and create a feature branch
2. **Write tests** for your changes
3. **Ensure all tests pass** and code is properly formatted
4. **Update documentation** if needed
5. **Submit a pull request** with a clear description

### Code Review Criteria

- **Correctness**: Does the code work as intended?
- **Test Coverage**: Are there adequate tests?
- **Performance**: Does it meet performance requirements?
- **Documentation**: Is the code well-documented?
- **Style**: Does it follow project conventions?

## Release Process

### Version Management

pb follows semantic versioning (SemVer):
- **Major** (x.0.0): Breaking changes
- **Minor** (0.x.0): New features, backward compatible
- **Patch** (0.0.x): Bug fixes, backward compatible

### Release Steps

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with release notes
3. **Create release tag**: `git tag v0.1.0`
4. **Push tag**: `git push origin v0.1.0`
5. **GitHub Actions** will automatically build and create release

### Release Checklist

- [ ] All tests pass
- [ ] Documentation is up to date
- [ ] Version number is updated
- [ ] CHANGELOG.md is updated
- [ ] Release notes are prepared
- [ ] Binaries are built for all platforms

## Performance Considerations

### Optimization Guidelines

1. **Minimize allocations** in hot paths
2. **Use appropriate data structures** for the use case  
3. **Profile before optimizing** to identify bottlenecks
4. **Consider memory vs. CPU tradeoffs**

### Critical Performance Areas

#### Time Parsing
- Cache compiled regex patterns
- Minimize string allocations
- Use efficient parsing algorithms

#### Progress Calculation
- Use integer arithmetic where possible
- Minimize floating-point operations
- Cache calculated values

#### Terminal Output
- Minimize terminal escape sequences
- Batch output operations
- Use efficient string formatting

### Profiling Tools

```bash
# Profile with cargo
cargo install cargo-profiler
cargo profiler callgrind --bin pb

# Benchmark with criterion (if added)
cargo bench

# Memory profiling with valgrind
valgrind --tool=massif target/debug/pb --help
```

### Performance Testing

Include performance tests in your development:
```rust
#[test]
fn performance_baseline_time_parsing() {
    use std::time::Instant;
    
    let iterations = 10_000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        parse_time("2025-01-27 12:00:00").unwrap();
    }
    
    let duration = start.elapsed();
    let per_iteration = duration / iterations;
    
    // Ensure parsing takes less than 10μs per iteration
    assert!(per_iteration.as_micros() < 10);
}
```

## Debugging and Troubleshooting

### Debug Build
```bash
# Build with debug symbols
cargo build

# Run with debug output
RUST_LOG=debug ./target/debug/pb --help
```

### Common Development Issues

#### Compilation Errors
- Check Rust version compatibility
- Update dependencies: `cargo update`
- Clean build artifacts: `cargo clean`

#### Test Failures
- Run tests in single-threaded mode: `cargo test -- --test-threads=1`
- Use `RUST_BACKTRACE=1` for detailed error traces
- Check for timing-dependent tests in CI

#### Docker Issues
- Rebuild container: `docker build --no-cache .`
- Check volume mounts and permissions
- Verify Docker daemon is running

For more detailed troubleshooting, see [troubleshooting.md](troubleshooting.md).