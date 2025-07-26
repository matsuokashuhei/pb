# Changelog

All notable changes to the pb CLI tool will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-01-27

### Added
- Initial release of pb CLI progress bar tool
- Time-based progress visualization between two points in time
- Support for multiple time formats:
  - Date format (`YYYY-MM-DD`)
  - Datetime format (`YYYY-MM-DD HH:MM:SS`)
  - Relative time format (`1h`, `30m`, `7d`, `3600s`)
- Real-time progress updates with configurable intervals
- Colored progress bar output (green/yellow/red)
- Cross-platform terminal support (Linux, macOS, Windows)
- TTY-aware output mode for terminal vs pipe detection
- Graceful Ctrl+C handling for interactive sessions
- Comprehensive error handling and input validation
- Performance optimizations for long-running sessions

### Performance
- Binary size: 2.4MB (optimized with LTO and strip)
- Memory usage: <10MB during operation
- Startup time: <50ms
- Efficient time parsing and progress calculations

### Security
- Input validation for all time formats
- Safe error handling without information leakage
- No external network dependencies
- Memory-safe Rust implementation

### Documentation
- Complete user guide with examples
- Installation instructions for all platforms
- API documentation for library usage
- Man page for Unix systems
- Troubleshooting guide
- Development and contribution guide

### Quality Assurance
- 120+ comprehensive tests covering all functionality
- Performance benchmarks and memory usage tests
- Cross-platform compatibility verification
- Error handling and edge case testing
- Integration tests for CLI interface

### Distribution
- Pre-built binaries for Linux (x86_64, ARM64)
- Pre-built binaries for macOS (Intel, Apple Silicon)
- Pre-built binaries for Windows (x86_64)
- Cargo/crates.io distribution
- Homebrew formula (planned)
- Docker container support

[1.0.0]: https://github.com/matsuokashuhei/pb/releases/tag/v1.0.0