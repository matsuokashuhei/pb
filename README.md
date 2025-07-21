# pb - CLI Progress Bar Tool

## Development Environment Setup

This project uses Docker for a consistent development environment across different platforms.

### Prerequisites

- Docker
- Git

### Quick Start

1. **Clone the repository** (if you haven't already):
   ```bash
   git clone https://github.com/matsuokashuhei/pb.git
   cd pb
   ```

2. **Build the project**:
   ```bash
   ./scripts/build.sh
   ```

3. **Run tests**:
   ```bash
   ./scripts/test.sh
   ```

4. **Try the application**:
   ```bash
   ./scripts/run.sh -- --help
   ```

## Development Scripts

All development tasks are handled through shell scripts in the `scripts/` directory:

### Build Script (`./scripts/build.sh`)
```bash
# Debug build
./scripts/build.sh

# Release build
./scripts/build.sh --release

# Verbose output
./scripts/build.sh --verbose
```

### Test Script (`./scripts/test.sh`)
```bash
# Run all tests
./scripts/test.sh

# Run only unit tests
./scripts/test.sh --unit

# Run only integration tests
./scripts/test.sh --integration

# Run with verbose output
./scripts/test.sh --verbose

# Pass additional arguments to cargo test
./scripts/test.sh -- --nocapture
```

### Run Script (`./scripts/run.sh`)
```bash
# Show help
./scripts/run.sh -- --help

# Run with debug build
./scripts/run.sh -- --start "2025-07-21 10:00:00" --end "2025-07-21 18:00:00"

# Run with release build
./scripts/run.sh --release -- --start "2025-07-21" --end "1d"
```

### Development Script (`./scripts/dev.sh`)
```bash
# Start interactive development shell
./scripts/dev.sh shell

# Clean all build artifacts
./scripts/dev.sh clean

# Format code
./scripts/dev.sh fmt

# Run clippy linter
./scripts/dev.sh clippy

# Quick check without building
./scripts/dev.sh check

# Update dependencies
./scripts/dev.sh deps
```

## Docker Architecture

The project uses a multi-stage Dockerfile:

- **Development stage**: Full Rust toolchain with dev tools
- **Builder stage**: Optimized build environment
- **Production stage**: Minimal runtime image

### Volume Mounts

The development environment uses Docker volumes for caching:

- `pb-cargo-cache`: Cargo registry cache
- `pb-target-cache`: Build output cache
- Source code: Live mounted from host

## Project Structure

```
pb/
├── Dockerfile              # Multi-stage Docker configuration
├── .dockerignore           # Docker ignore file
├── scripts/                # Development scripts
│   ├── build.sh           # Build script
│   ├── test.sh            # Test script
│   ├── run.sh             # Run script
│   └── dev.sh             # Development utilities
├── docs/                   # Documentation
│   ├── specification.md   # Functional specification
│   └── technical_specification.md  # Technical details
├── src/                    # Source code (to be created)
├── tests/                  # Integration tests (to be created)
└── Cargo.toml             # Project configuration (to be created)
```

## Commands Overview

| Task | Command |
|------|---------|
| Build (debug) | `./scripts/build.sh` |
| Build (release) | `./scripts/build.sh -r` |
| Test (all) | `./scripts/test.sh` |
| Test (unit only) | `./scripts/test.sh -u` |
| Run application | `./scripts/run.sh -- [ARGS]` |
| Development shell | `./scripts/dev.sh shell` |
| Format code | `./scripts/dev.sh fmt` |
| Lint code | `./scripts/dev.sh clippy` |
| Clean everything | `./scripts/dev.sh clean` |

## Features

- **Time-based progress visualization**: Display progress between start and end times
- **Multiple time formats**: Date, datetime, and relative time support
- **Real-time updates**: Configurable update intervals
- **Cross-platform**: Works on macOS, Linux, and Windows
- **Dockerized development**: Consistent environment across platforms

## Usage Examples

```bash
# Basic usage with datetime
./scripts/run.sh -- --start "2025-07-21 10:00:00" --end "2025-07-21 18:00:00"

# Using relative time
./scripts/run.sh -- --start "2025-07-21 10:00:00" --end "8h"

# Custom update interval (30 seconds)
./scripts/run.sh -- --start "2025-07-21" --end "2025-07-22" --interval 30
```

## License

MIT License
