# pmon - CLI Progress Monitor Tool

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/matsuokashuhei/pmon/workflows/CI/badge.svg)](https://github.com/matsuokashuhei/pmon/actions)

A command-line progress monitor tool for time-based visualization. Track time between two points with real-time progress updates, perfect for monitoring deadlines, work sessions, or any time-based process.

![pmon demo](https://user-images.githubusercontent.com/example/pmon-demo.gif)

## Quick Start

```bash
# Track an 8-hour work day (traditional usage)
pmon --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"

# Track remaining work day (start from now)
pmon --end "17:00:00"

# Monitor a 2-hour meeting timer (start from now)
pmon --end "2h" --interval 30

# Create a countdown to a deadline (start from today 00:00:00)
pmon --end "2025-02-15" --interval 3600
```

## Features

- ⏱️ **Time-based progress visualization** - Track progress between any two points in time
- 📅 **Multiple time formats** - Support for dates, datetimes, and relative time expressions
- 🔄 **Real-time updates** - Configurable update intervals from seconds to hours
- 🎨 **Colored output** - Green/yellow/red progress bars with status indicators
- 🖥️ **Cross-platform** - Works on Linux, macOS, and Windows
- 📊 **Multiple output modes** - TTY-aware for terminals and piping
- ⚡ **Lightweight** - Minimal resource usage, <10MB memory
- 🚀 **Fast** - Optimized for frequent updates and long-running sessions

## Installation

### Pre-built Binaries (Recommended)

#### Linux
```bash
curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-linux-x86_64
chmod +x pmon
sudo mv pmon /usr/local/bin/
```

#### macOS
```bash
# Intel Mac
curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-macos-x86_64

# Apple Silicon
curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-macos-arm64

chmod +x pmon
sudo mv pmon /usr/local/bin/
```

#### Windows
Download `pmon-windows-x86_64.exe` from the [releases page](https://github.com/matsuokashuhei/pmon/releases) and add to your PATH.

### Package Managers

```bash
# Homebrew (macOS/Linux)
brew tap matsuokashuhei/pmon
brew install pmon

# Cargo (Rust)
cargo install pmon-cli
```

### From Source

```bash
git clone https://github.com/matsuokashuhei/pmon.git
cd pmon
cargo build --release
sudo mv target/release/pmon /usr/local/bin/
```

For detailed installation instructions for all platforms, see [Installation Guide](docs/installation.md).

## Usage

### Basic Syntax

```bash
pmon [--start START_TIME] --end END_TIME [--interval SECONDS]
```

**Note**: The `--start` parameter is optional. When omitted, pmon automatically determines the start time based on the end time format:
- **Time-containing formats** (e.g., "17:00:00", "2h", "+30m") → Start from current time
- **Date-only formats** (e.g., "2025-12-31") → Start from today at 00:00:00

### Time Formats

pmon supports three flexible time formats:

#### Date Format (`YYYY-MM-DD`)
```bash
pmon --start "2025-01-27" --end "2025-01-28"
```
*Time defaults to 00:00:00 (midnight)*

#### Datetime Format (`YYYY-MM-DD HH:MM:SS`)
```bash
pmon --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"
```
*Full datetime in 24-hour format*

#### Relative Time Format
```bash
pmon --start "2025-01-27 14:00:00" --end "2h"    # 2 hours
pmon --start "2025-01-27 14:00:00" --end "90m"   # 90 minutes
pmon --start "2025-01-27" --end "7d"             # 7 days
pmon --start "2025-01-27 14:00:00" --end "3600s" # 3600 seconds
```
*Supports hours (h), minutes (m), days (d), and seconds (s)*

### Automatic Start Time Detection

When the `--start` parameter is omitted, pmon automatically determines the appropriate start time based on the end time format:

#### Time-containing end formats → Current time as start
```bash
pmon --end "17:00:00"             # Work day tracking - start from now
pmon --end "2025-07-27 17:00:00"  # Datetime format - start from now  
pmon --end "2h"                   # Meeting timer - 2 hours from now
pmon --end "+30m"                 # Study session - 30 minutes from now
```

#### Date-only end formats → Today 00:00:00 as start
```bash
pmon --end "2025-12-31"           # Project deadline - track from start of today
```

This feature makes pmon more intuitive for common use cases while maintaining backward compatibility.

### Common Use Cases

#### Work Day Tracking
```bash
# Standard 8-hour work day (traditional usage)
pmon --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"

# Track remaining work day from now
pmon --end "17:00:00"

# Flexible start with relative end time
pmon --start "$(date '+%Y-%m-%d 09:00:00')" --end "8h"
```

#### Meeting Timer
```bash
# 1-hour meeting with minute-by-minute updates (traditional usage)
pmon --start "2025-01-27 14:00:00" --end "1h" --interval 60

# 2-hour meeting starting now
pmon --end "2h" --interval 60

# 30-minute quick meeting starting now
pmon --end "30m" --interval 30
```

#### Project Deadline
```bash
# Track progress to deadline with daily updates (traditional usage)
pmon --start "2025-01-20" --end "2025-02-15" --interval 86400

# Track remaining time to deadline from today
pmon --end "2025-02-15" --interval 86400
```

#### Study/Focus Sessions
```bash
# Pomodoro timer (25 minutes) - traditional usage
pmon --start "$(date '+%Y-%m-%d %H:%M:%S')" --end "25m" --interval 60

# Pomodoro timer starting now
pmon --end "25m" --interval 60

# 2-hour study session
pmon --end "2h" --interval 300
```

### Output Examples

#### Terminal Output (TTY Mode)
```
pmon - Progress Bar Tool
Start time: 2025-01-27 09:00:00
End time: 2025-01-27 17:00:00
Update interval: 60 seconds
Press Ctrl+C to exit

[████████░░░░░░░░░░░░░░░░░░░░] 32.5% (2h 36m elapsed, 5h 24m remaining)
```

#### Piped Output (Non-TTY Mode)
```bash
# Traditional usage with explicit start time
pmon --start "2025-01-27 09:00:00" --end "8h" | while read line; do
    echo "$(date): $line" >> progress.log
done

# New usage with auto-determined start time
pmon --end "8h" | while read line; do
    echo "$(date): $line" >> progress.log
done
```

#### Color Coding
- 🟢 **Green** (0-80%): Normal progress
- 🟡 **Yellow** (80-100%): Nearing completion  
- 🔴 **Red** (>100%): Overtime

## Command Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--start` | `-s` | Start time (optional) | Auto-determined from end time |
| `--end` | `-e` | End time (required) | - |
| `--interval` | `-i` | Update interval in seconds | 60 |
| `--help` | `-h` | Show help message | - |
| `--version` | `-V` | Show version | - |

## Advanced Usage

### Scripting Integration

```bash
#!/bin/bash
# Track work day and send notifications (traditional usage)

pmon --start "2025-01-27 09:00:00" --end "8h" --interval 300 | while read -r line; do
    percentage=$(echo "$line" | grep -o '[0-9]*\.[0-9]*%')
    
    case $percentage in
        "50.0%") notify-send "Work Progress" "Halfway through the day!" ;;
        "100.0%") notify-send "Work Complete" "Time to go home!" ;;
    esac
done

# Or use the simplified version starting from now
pmon --end "8h" --interval 300 | while read -r line; do
    percentage=$(echo "$line" | grep -o '[0-9]*\.[0-9]*%')
    
    case $percentage in
        "50.0%") notify-send "Work Progress" "Halfway through the work session!" ;;
        "100.0%") notify-send "Work Complete" "Session finished!" ;;
    esac
done
```

### Background Monitoring

```bash
# Start pmon in background and monitor with logs (traditional usage)
pmon --start "2025-01-27 09:00:00" --end "8h" > work_progress.log 2>&1 &
tail -f work_progress.log

# Or start tracking from now
pmon --end "8h" > work_progress.log 2>&1 &
tail -f work_progress.log
```

### Multiple Timers

```bash
# Track multiple time periods simultaneously (traditional usage)
pmon --start "2025-01-27 09:00:00" --end "8h" > work.log &
pmon --start "2025-01-27 12:00:00" --end "1h" > lunch.log &
pmon --start "2025-01-27 14:00:00" --end "2h" > meeting.log &

# Or use simplified syntax where appropriate
pmon --end "8h" > work.log &           # Work session from now
pmon --end "1h" > lunch.log &          # 1-hour timer from now  
pmon --end "2h" > meeting.log &        # 2-hour timer from now
```

## Documentation

- 📖 **[User Guide](docs/user_guide.md)** - Comprehensive usage guide with examples
- 🔧 **[Installation Guide](docs/installation.md)** - Platform-specific installation instructions
- 💻 **[Development Guide](docs/development_guide.md)** - For contributors and developers
- 🚀 **[Build & Deployment](docs/build_deployment.md)** - Building and deployment procedures
- 📚 **[API Documentation](docs/api_documentation.md)** - Library API reference
- 🔍 **[Troubleshooting](docs/troubleshooting.md)** - Common issues and solutions
- 📋 **[Examples](docs/examples/)** - Practical usage examples
- 📄 **[Man Page](docs/man/pmon.1)** - Unix manual page

## Development

### Quick Development Setup

```bash
# Clone and setup
git clone https://github.com/matsuokashuhei/pmon.git
cd pmon

# Build and test
cargo build
cargo test

# Run with sample data (traditional usage)
cargo run -- --start "2025-01-27 12:00:00" --end "2025-01-27 13:00:00" --interval 5

# Run with simplified syntax
cargo run -- --end "1h" --interval 5
```

### Docker Development Environment

```bash
# Use Docker for consistent development environment
./scripts/build.sh      # Build project
./scripts/test.sh       # Run tests  
./scripts/run.sh -- --help  # Run application
./scripts/dev.sh shell  # Development shell
```

### Contributing

We welcome contributions! Please see our [Development Guide](docs/development_guide.md) for:

- Setting up the development environment
- Code organization and standards
- Testing guidelines
- Pull request process

## Performance

pmon is designed to be lightweight and efficient:

- **Startup time**: <50ms
- **Memory usage**: <10MB during operation
- **CPU usage**: Minimal, only during updates
- **Time parsing**: <10μs per operation
- **Progress calculation**: <1μs per calculation

## Platform Support

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux | x86_64 | ✅ Fully supported |
| Linux | ARM64 | ✅ Fully supported |
| macOS | Intel (x86_64) | ✅ Fully supported |
| macOS | Apple Silicon (ARM64) | ✅ Fully supported |
| Windows | x86_64 | ✅ Fully supported |
| FreeBSD | x86_64 | 🟡 Community supported |

## FAQ

**Q: Can I pause and resume pmon?**  
A: pmon tracks real time, so it can't be paused. Stop with Ctrl+C and restart with adjusted times.

**Q: What happens when the end time is reached?**  
A: pmon shows 100% completion and exits. If current time exceeds end time, it shows >100% in red.

**Q: Does pmon work across time zones?**  
A: pmon uses local system time. Ensure your system clock is correct for accurate tracking.

**Q: Can I run multiple pmon instances?**  
A: Yes! Run multiple instances in different terminals to track multiple time periods.

For more questions, see [Troubleshooting Guide](docs/troubleshooting.md).

## License

pmon is licensed under the [MIT License](LICENSE). See LICENSE file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and reliability
- Uses [clap](https://clap.rs/) for command-line parsing
- Uses [chrono](https://github.com/chronotope/chrono) for robust time handling
- Uses [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal support

---

**⭐ Star this repo if you find pmon useful!**
