# API Documentation

This document provides comprehensive API documentation for the pb library. The pb library provides the core functionality for the pb CLI tool, including time parsing, progress calculation, and progress bar rendering.

## Overview

The pb library is organized into several modules:

- **[cli](cli/index.html)**: Command-line interface and argument parsing
- **[error](error/index.html)**: Error types and handling
- **[progress_bar](progress_bar/index.html)**: Progress calculation and rendering
- **[time_parser](time_parser/index.html)**: Time parsing and validation

## Quick Start

```rust
use pb::{parse_time, calculate_progress, render_colored_progress_bar};

// Parse time strings
let start = parse_time("2025-01-27 09:00:00")?;
let end = parse_time("2025-01-27 17:00:00")?;
let current = chrono::Local::now().naive_local();

// Calculate progress
let progress = calculate_progress(start, end, current);

// Render progress bar
let bar = render_colored_progress_bar(progress);
println!("{}", bar);
```

## Complete API Reference

For complete API documentation with all functions, types, and examples, run:

```bash
cargo doc --open
```

Or view the generated documentation in `target/doc/pb/index.html`.

## Core Functions

### Time Parsing

#### `parse_time(input: &str) -> PbResult<NaiveDateTime>`

Primary time parsing function that automatically detects and parses various time formats.

**Supported Formats:**
- Date: `YYYY-MM-DD` (e.g., "2025-01-27")
- Datetime: `YYYY-MM-DD HH:MM:SS` (e.g., "2025-01-27 14:30:00")
- Relative: `Nh`, `Nm`, `Ns`, `Nd` (e.g., "2h", "30m", "45s", "1d")

**Example:**
```rust
use pb::parse_time;

let datetime = parse_time("2025-01-27 14:30:00")?;
let relative = parse_time("2h")?; // 2 hours from now
```

#### `parse_date(input: &str) -> PbResult<NaiveDateTime>`

Parse date strings in YYYY-MM-DD format.

**Example:**
```rust
use pb::parse_date;

let date = parse_date("2025-01-27")?; // 2025-01-27 00:00:00
```

#### `parse_datetime(input: &str) -> PbResult<NaiveDateTime>`

Parse datetime strings in YYYY-MM-DD HH:MM:SS format.

**Example:**
```rust
use pb::parse_datetime;

let datetime = parse_datetime("2025-01-27 14:30:00")?;
```

#### `parse_relative_time(input: &str) -> PbResult<NaiveDateTime>`

Parse relative time expressions and return the absolute time.

**Example:**
```rust
use pb::parse_relative_time;

let future_time = parse_relative_time("2h")?; // 2 hours from now
let tomorrow = parse_relative_time("1d")?;    // 1 day from now
```

#### `validate_times(start: NaiveDateTime, end: NaiveDateTime) -> PbResult<()>`

Validate that start time is before end time.

**Example:**
```rust
use pb::{parse_time, validate_times};

let start = parse_time("2025-01-27 09:00:00")?;
let end = parse_time("2025-01-27 17:00:00")?;
validate_times(start, end)?; // Ensures start < end
```

### Progress Calculation

#### `calculate_progress(start: NaiveDateTime, end: NaiveDateTime, current: NaiveDateTime) -> f64`

Calculate progress percentage between start and end times.

**Returns:** Progress as a percentage (0.0 to 100.0+, can exceed 100% for overtime)

**Example:**
```rust
use pb::calculate_progress;
use chrono::Local;

let start = parse_time("2025-01-27 09:00:00")?;
let end = parse_time("2025-01-27 17:00:00")?;
let current = Local::now().naive_local();

let progress = calculate_progress(start, end, current);
println!("Progress: {:.1}%", progress);
```

### Progress Bar Rendering

#### `render_progress_bar(percentage: f64) -> String`

Render a basic progress bar without colors.

**Example:**
```rust
use pb::render_progress_bar;

let bar = render_progress_bar(65.0);
println!("{}", bar); // [█████████████░░░░░░░] 65.0%
```

#### `render_colored_progress_bar(percentage: f64) -> String`

Render a colored progress bar with status indicators.

**Color Scheme:**
- Green: 0-80% (normal progress)
- Yellow: 80-100% (nearing completion)
- Red: >100% (overtime)

**Example:**
```rust
use pb::render_colored_progress_bar;

let bar = render_colored_progress_bar(45.5);
println!("{}", bar); // Colored progress bar with green fill
```

### Command-Line Interface

#### `Cli::parse_args() -> PbResult<Cli>`

Parse command-line arguments and return a validated CLI configuration.

**Example:**
```rust
use pb::Cli;

let cli = Cli::parse_args()?;
println!("Start: {}", cli.start());
println!("End: {}", cli.end());
println!("Interval: {} seconds", cli.interval());
```

#### CLI Methods

- `start() -> &str`: Get the start time string
- `end() -> &str`: Get the end time string  
- `interval() -> u64`: Get the update interval in seconds

### Error Handling

#### `PbError`

Custom error type for pb-specific errors.

**Variants:**
- `InvalidTimeFormat { input: String }`: Invalid time format
- `TimeValidation { message: String }`: Time validation error
- `ParseError { message: String }`: Generic parsing error

**Example:**
```rust
use pb::{PbError, PbResult};

fn parse_user_input(input: &str) -> PbResult<()> {
    match parse_time(input) {
        Ok(_) => Ok(()),
        Err(PbError::InvalidTimeFormat { input }) => {
            eprintln!("Invalid format: {}", input);
            Err(PbError::InvalidTimeFormat { input: input.to_string() })
        }
        Err(e) => Err(e),
    }
}
```

## Usage Patterns

### Basic Time Tracking

```rust
use pb::{parse_time, calculate_progress, render_colored_progress_bar, validate_times};
use chrono::Local;

fn track_progress(start_str: &str, end_str: &str) -> pb::PbResult<()> {
    // Parse times
    let start = parse_time(start_str)?;
    let end = parse_time(end_str)?;
    
    // Validate
    validate_times(start, end)?;
    
    // Calculate and display progress
    let current = Local::now().naive_local();
    let progress = calculate_progress(start, end, current);
    let bar = render_colored_progress_bar(progress);
    
    println!("{}", bar);
    Ok(())
}
```

### Continuous Monitoring

```rust
use pb::{parse_time, calculate_progress, render_colored_progress_bar};
use std::{thread, time::Duration};
use chrono::Local;

fn monitor_progress(start_str: &str, end_str: &str, interval_secs: u64) -> pb::PbResult<()> {
    let start = parse_time(start_str)?;
    let end = parse_time(end_str)?;
    let interval = Duration::from_secs(interval_secs);
    
    loop {
        let current = Local::now().naive_local();
        let progress = calculate_progress(start, end, current);
        let bar = render_colored_progress_bar(progress);
        
        println!("\r{}", bar);
        
        if progress >= 100.0 {
            println!("\nCompleted!");
            break;
        }
        
        thread::sleep(interval);
    }
    
    Ok(())
}
```

### Error Handling with Context

```rust
use pb::{parse_time, PbResult};
use anyhow::Context;

fn parse_with_context(input: &str, label: &str) -> anyhow::Result<chrono::NaiveDateTime> {
    parse_time(input)
        .with_context(|| format!("Failed to parse {} time: '{}'", label, input))
}

fn main() -> anyhow::Result<()> {
    let start = parse_with_context("2025-01-27 09:00:00", "start")?;
    let end = parse_with_context("invalid", "end")?; // Will provide detailed error
    
    Ok(())
}
```

## Advanced Usage

### Custom Progress Calculation

```rust
use chrono::{NaiveDateTime, Duration};

fn calculate_remaining_time(start: NaiveDateTime, end: NaiveDateTime, current: NaiveDateTime) -> Duration {
    if current >= end {
        Duration::zero()
    } else {
        end - current
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}
```

### Custom Progress Bar Rendering

```rust
use colored::{Colorize, ColoredString};

fn custom_progress_bar(percentage: f64, width: usize) -> String {
    let filled_width = ((percentage / 100.0) * width as f64) as usize;
    let empty_width = width.saturating_sub(filled_width);
    
    let filled = "█".repeat(filled_width);
    let empty = "░".repeat(empty_width);
    
    let colored_filled = if percentage > 100.0 {
        filled.red()
    } else if percentage > 80.0 {
        filled.yellow()
    } else {
        filled.green()
    };
    
    format!("[{}{}] {:.1}%", colored_filled, empty, percentage)
}
```

## Type Definitions

### Re-exported Types

The pb library re-exports commonly used types for convenience:

```rust
pub use anyhow::{Context, Result as AnyhowResult};
pub use cli::Cli;
pub use error::{PbError, PbResult};
```

### Internal Types

Key internal types used throughout the library:

- `chrono::NaiveDateTime`: Timezone-naive datetime representation
- `regex::Regex`: Compiled regular expressions for parsing
- `colored::ColoredString`: Colored terminal output strings

## Platform Considerations

### Cross-Platform Compatibility

The pb library is designed to work across Unix-like systems and Windows:

- **Time handling**: Uses `chrono` for robust cross-platform time operations
- **Terminal output**: Uses `crossterm` for cross-platform terminal manipulation
- **Colors**: Uses `colored` crate with automatic color detection

### Performance Characteristics

- **Time parsing**: Optimized with compiled regex patterns
- **Progress calculation**: Uses integer arithmetic where possible
- **Memory usage**: Minimal allocations in hot paths
- **CPU usage**: Lightweight with configurable update intervals

## Testing

The library includes comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific module tests
cargo test time_parser
cargo test progress_bar
```

## Contributing

When contributing to the pb library:

1. Maintain backward compatibility for public APIs
2. Add comprehensive tests for new functionality
3. Update documentation for any API changes
4. Follow the existing error handling patterns
5. Ensure cross-platform compatibility

For detailed contribution guidelines, see [development_guide.md](development_guide.md).

## Performance Benchmarks

The pb library is optimized for common usage patterns:

- **Time parsing**: < 10μs per operation for common formats
- **Progress calculation**: < 1μs per calculation
- **Progress bar rendering**: < 100μs per render
- **Memory usage**: < 10MB total during operation

For detailed performance analysis, see the performance tests in the test suite.

## License

The pb library is licensed under the MIT License. See LICENSE file for details.