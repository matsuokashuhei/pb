# GitHub Actions CI/CD Documentation

This document describes the GitHub Actions workflows implemented for the `pb` project.

## Workflows Overview

### 1. CI Workflow (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `main` branch
- Pull requests to `main` branch  
- Manual workflow dispatch

**Features:**
- **Matrix Testing**: Tests across Ubuntu, macOS, and Windows
- **Multi-Version Rust**: Tests on stable, beta, and nightly Rust (nightly failures don't fail the workflow)
- **Comprehensive Testing**: Runs unit tests, integration tests, and doc tests
- **Code Quality**: Includes formatting checks (`cargo fmt`) and linting (`cargo clippy`)
- **Binary Validation**: Builds and tests the binary functionality
- **Dependency Caching**: Caches Cargo dependencies for faster builds
- **Code Coverage**: Generates coverage reports using `cargo-tarpaulin` and uploads to Codecov

**Jobs:**
1. **test**: Runs the full test suite across matrix configurations
2. **build**: Creates build artifacts for multiple architectures
3. **coverage**: Generates and uploads code coverage reports

### 2. Release Workflow (`.github/workflows/release.yml`)

**Triggers:**
- Git tags matching `v*` pattern
- Manual workflow dispatch with tag input

**Features:**
- **Cross-Platform Builds**: Builds binaries for 5 different architectures:
  - Linux x86_64
  - Linux aarch64 (ARM64)
  - macOS x86_64 (Intel)
  - macOS aarch64 (Apple Silicon)
  - Windows x86_64
- **Automated Releases**: Creates GitHub releases with proper release notes
- **Binary Validation**: Tests binaries before uploading (where possible)
- **Asset Management**: Uploads properly named binary assets

**Jobs:**
1. **create-release**: Creates the GitHub release
2. **build-release**: Builds and uploads binary assets
3. **validate-release**: Validates the release

## Supported Platforms

| Platform | Architecture | Target Triple | Binary Name |
|----------|-------------|---------------|-------------|
| Linux | x86_64 | x86_64-unknown-linux-gnu | pb-linux-x86_64 |
| Linux | aarch64 | aarch64-unknown-linux-gnu | pb-linux-aarch64 |
| macOS | x86_64 | x86_64-apple-darwin | pb-macos-x86_64 |
| macOS | aarch64 | aarch64-apple-darwin | pb-macos-aarch64 |
| Windows | x86_64 | x86_64-pc-windows-msvc | pb-windows-x86_64.exe |

## Usage

### Development Workflow
1. Create a pull request - triggers CI workflow automatically
2. CI runs all tests, linting, and building across multiple platforms
3. Code coverage reports are generated and uploaded
4. Status checks ensure code quality before merging

### Release Workflow
1. Create and push a version tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
2. Release workflow automatically:
   - Creates a GitHub release
   - Builds binaries for all supported platforms
   - Uploads binary assets to the release
   - Generates comprehensive release notes

### Manual Triggers
Both workflows support manual triggering via GitHub's web interface:
- Go to "Actions" tab in GitHub
- Select the workflow
- Click "Run workflow"

## Caching Strategy

The workflows implement intelligent caching to improve build performance:
- **Cargo Registry**: Caches downloaded crates
- **Cargo Index**: Caches the crates.io index
- **Build Artifacts**: Caches compiled dependencies
- **Platform-Specific**: Separate caches for different OS/architecture combinations

## Error Handling

- **Nightly Rust**: Failures on nightly Rust don't fail the workflow (marked as experimental)
- **Cross-Compilation**: Proper error handling for cross-compilation scenarios
- **Binary Validation**: Skips validation for cross-compiled binaries that can't run on the build machine

## Code Coverage

Code coverage is measured using `cargo-tarpaulin` and uploaded to Codecov:
- Runs on Ubuntu latest with stable Rust
- Includes all features and workspace
- Generates XML reports for better integration
- Non-blocking (failures don't fail the CI)

## Status Badges

The following badges can be added to README.md:

```markdown
![CI](https://github.com/matsuokashuhei/pmon/workflows/CI/badge.svg)
![Release](https://github.com/matsuokashuhei/pmon/workflows/Release/badge.svg)
[![codecov](https://codecov.io/gh/matsuokashuhei/pmon/branch/main/graph/badge.svg)](https://codecov.io/gh/matsuokashuhei/pmon)
```

## Local Testing

To test the same commands that CI runs locally:

```bash
# Format checking
cargo fmt --all -- --check

# Linting
cargo clippy --all-targets --all-features

# Building
cargo build --verbose

# Testing
cargo test --lib --verbose        # Unit tests
cargo test --test '*' --verbose   # Integration tests
cargo test --doc --verbose        # Doc tests

# Release build and validation
cargo build --release
./target/release/pb --help
```