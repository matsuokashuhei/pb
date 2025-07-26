# Release Notes - pb v1.0.0

## Overview
We're excited to announce the first stable release of **pb** - a CLI progress bar tool for time-based visualization. This release provides a robust and efficient solution for tracking progress between any two points in time.

## ✨ Key Features

### Time-Based Progress Tracking
- **Multiple time formats**: Support for dates (`2025-01-27`), datetimes (`2025-01-27 14:00:00`), and relative times (`2h`, `30m`, `7d`)
- **Real-time updates**: Configurable update intervals from seconds to hours
- **Smart time parsing**: Automatic format detection with comprehensive validation

### Visual Progress Bars
- **Colored output**: Green (0-80%), yellow (80-100%), red (>100% overtime)
- **TTY-aware**: Different output modes for terminal vs piped usage
- **Cross-platform**: Works seamlessly on Linux, macOS, and Windows

### User Experience
- **Graceful interruption**: Ctrl+C handling for clean exits
- **Clear error messages**: Helpful feedback for invalid inputs
- **Lightweight**: <10MB memory usage, 2.4MB binary size
- **Fast startup**: <50ms initialization time

## 📦 Installation

### Pre-built Binaries
Download from [GitHub Releases](https://github.com/matsuokashuhei/pb/releases/v1.0.0):

**Linux x86_64:**
```bash
curl -L -o pb https://github.com/matsuokashuhei/pb/releases/v1.0.0/download/pb-linux-x86_64
chmod +x pb && sudo mv pb /usr/local/bin/
```

**macOS:**
```bash
# Intel Macs
curl -L -o pb https://github.com/matsuokashuhei/pb/releases/v1.0.0/download/pb-macos-x86_64

# Apple Silicon
curl -L -o pb https://github.com/matsuokashuhei/pb/releases/v1.0.0/download/pb-macos-arm64

chmod +x pb && sudo mv pb /usr/local/bin/
```

**Windows:**
Download `pb-windows-x86_64.exe` from the releases page.

### From Source
```bash
cargo install pb-cli --version 1.0.0
```

## 🚀 Quick Start

```bash
# Track an 8-hour work day
pb --start "2025-01-27 09:00:00" --end "2025-01-27 17:00:00"

# Monitor a 1-hour meeting with 30-second updates
pb --start "2025-01-27 14:00:00" --end "1h" --interval 30

# Create a countdown to a deadline
pb --start "2025-01-27" --end "2025-02-15" --interval 3600
```

## 🔧 Technical Specifications

### Performance
- **Binary size**: 2.4MB (optimized with LTO)
- **Memory usage**: <10MB during operation
- **CPU usage**: Minimal, only during updates
- **Startup time**: <50ms

### Platform Support
- ✅ Linux x86_64, ARM64
- ✅ macOS Intel (x86_64), Apple Silicon (ARM64)
- ✅ Windows x86_64
- 🟡 FreeBSD x86_64 (community supported)

### Dependencies
- Zero external runtime dependencies
- No network access required
- Standalone binary execution

## 🛡️ Security & Quality

### Security Features
- Input validation for all time formats
- Safe error handling without information leakage
- Memory-safe Rust implementation
- No privilege escalation required

### Quality Assurance
- **120+ tests** covering all functionality
- Performance benchmarks and memory profiling
- Cross-platform compatibility testing
- Comprehensive error handling validation

## 📚 Documentation

Complete documentation is available:
- [User Guide](docs/user_guide.md) - Usage examples and tutorials
- [Installation Guide](docs/installation.md) - Platform-specific setup
- [API Documentation](docs/api_documentation.md) - Library usage
- [Troubleshooting](docs/troubleshooting.md) - Common issues and solutions

## 🤝 Contributing

We welcome contributions! See our [Development Guide](docs/development_guide.md) for:
- Setting up the development environment  
- Code organization and standards
- Testing guidelines
- Pull request process

## 📝 License

pb is licensed under the [MIT License](LICENSE).

## 🙏 Acknowledgments

Built with [Rust](https://www.rust-lang.org/) for performance and reliability, using:
- [clap](https://clap.rs/) for command-line parsing
- [chrono](https://github.com/chronotope/chrono) for robust time handling
- [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal support

---

**Download pb v1.0.0 today and start tracking your time-based progress!**

For questions, issues, or feature requests, visit our [GitHub repository](https://github.com/matsuokashuhei/pb).