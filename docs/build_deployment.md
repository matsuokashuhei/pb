# Build and Deployment Guide

This guide covers building, testing, and deploying pb across different platforms and environments.

## Table of Contents

- [Build Requirements](#build-requirements)
- [Local Development Build](#local-development-build)
- [Production Build](#production-build)
- [Cross-Platform Building](#cross-platform-building)
- [Automated Testing](#automated-testing)
- [Performance Benchmarking](#performance-benchmarking)
- [Documentation Generation](#documentation-generation)
- [Release Process](#release-process)
- [Deployment](#deployment)
- [CI/CD Pipeline](#cicd-pipeline)

## Build Requirements

### System Requirements

#### Minimum Requirements
- **CPU**: Any modern x86_64 or ARM64 processor
- **RAM**: 512MB available during build, 10MB for runtime
- **Disk**: 2GB free space for build dependencies, 10MB for binary
- **Network**: Internet connection for downloading dependencies

#### Supported Platforms
- **Linux**: Ubuntu 18.04+, CentOS 7+, Debian 9+, Arch Linux
- **macOS**: macOS 10.15+ (Intel and Apple Silicon)
- **Windows**: Windows 10+ (x86_64)
- **Unix-like**: FreeBSD, OpenBSD, NetBSD

### Software Dependencies

#### Core Build Tools
```bash
# Rust toolchain (required)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version  # Should be 1.70+

# Git for source control
git --version

# C compiler (usually pre-installed)
gcc --version || clang --version
```

#### Optional Tools
```bash
# For cross-compilation
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-apple-darwin

# For enhanced development
cargo install cargo-watch     # Auto-rebuild on changes
cargo install cargo-audit     # Security audit
cargo install cargo-bloat     # Binary size analysis
cargo install cargo-deny      # License compliance
```

#### Docker (Alternative)
```bash
# Docker for consistent build environment
docker --version
docker-compose --version
```

## Local Development Build

### Quick Development Build

```bash
# Clone the repository
git clone https://github.com/matsuokashuhei/pb.git
cd pb

# Development build (fast, with debug symbols)
cargo build

# Run the application
./target/debug/pb --help
```

### Using Development Scripts

```bash
# Build with scripts (Docker-based)
./scripts/build.sh

# Build with verbose output
./scripts/build.sh --verbose

# Build and run tests
./scripts/build.sh && ./scripts/test.sh
```

### Development Workflow

```bash
# Watch for changes and rebuild
cargo watch -x build

# Watch and run tests
cargo watch -x test

# Format code on save
cargo watch -s "cargo fmt"

# Comprehensive development cycle
cargo watch -x "build" -x "test" -x "fmt" -x "clippy"
```

## Production Build

### Optimized Release Build

```bash
# Release build (optimized, smaller binary)
cargo build --release

# Binary location
ls -la target/release/pb

# Strip debug symbols (optional, reduces size)
strip target/release/pb
```

### Build Optimization Options

```bash
# Maximum optimization
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Link-time optimization
RUSTFLAGS="-C lto=fat" cargo build --release

# Size optimization
RUSTFLAGS="-C opt-level=s" cargo build --release

# Combined optimizations
RUSTFLAGS="-C target-cpu=native -C lto=fat -C codegen-units=1" \
  cargo build --release
```

### Static Binary Build

```bash
# Build static binary (Linux)
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

# Verify static linking
ldd target/x86_64-unknown-linux-musl/release/pb
# Should output: "not a dynamic executable"
```

## Cross-Platform Building

### Setting Up Cross-Compilation

```bash
# Add target architectures
rustup target add x86_64-pc-windows-gnu      # Windows
rustup target add x86_64-apple-darwin        # macOS Intel
rustup target add aarch64-apple-darwin       # macOS Apple Silicon
rustup target add aarch64-unknown-linux-gnu  # ARM64 Linux

# Install cross-compilation tools
# Linux: Install mingw-w64 for Windows targets
sudo apt-get install mingw-w64

# macOS: Install osxcross for Linux->macOS
# (Complex setup, consider using CI/CD instead)
```

### Building for Multiple Targets

```bash
# Build for all supported targets
targets=(
    "x86_64-unknown-linux-gnu"
    "x86_64-pc-windows-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "aarch64-unknown-linux-gnu"
)

for target in "${targets[@]}"; do
    echo "Building for $target..."
    cargo build --release --target "$target"
done
```

### Cross-Compilation with Docker

```dockerfile
# Multi-stage Dockerfile for cross-compilation
FROM rust:1.70-slim as builder

# Install cross-compilation tools
RUN apt-get update && apt-get install -y \
    gcc-mingw-w64 \
    gcc-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

# Add targets
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add aarch64-unknown-linux-gnu

WORKDIR /app
COPY . .

# Build for multiple targets
RUN cargo build --release --target x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-pc-windows-gnu
RUN cargo build --release --target aarch64-unknown-linux-gnu
```

### Using GitHub Actions for Cross-Compilation

```yaml
# .github/workflows/build.yml
name: Build
on: [push, pull_request]

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
          - os: windows-latest
            target: x86_64-pc-windows-msvc
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin

    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: pb-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/pb*
```

## Automated Testing

### Test Categories

```bash
# Run all tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Documentation tests
cargo test --doc

# Specific test module
cargo test time_parser

# Run with output
cargo test -- --nocapture
```

### Test Configuration

```bash
# Parallel test execution (default)
cargo test

# Single-threaded tests (for debugging)
cargo test -- --test-threads=1

# Run ignored tests
cargo test -- --ignored

# Run tests matching pattern
cargo test test_parse_time

# Test with environment variables
RUST_BACKTRACE=1 cargo test
```

### Performance Testing

```bash
# Run performance benchmarks
cargo test performance -- --nocapture

# Memory usage tests
cargo test memory -- --nocapture

# Stress tests
cargo test stress -- --nocapture --test-threads=1
```

### Test Coverage

```bash
# Install cargo-tarpaulin (Linux only)
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# Coverage with exclusions
cargo tarpaulin --exclude-files "tests/*" --out Html
```

## Performance Benchmarking

### Built-in Benchmarks

```bash
# Run performance tests
cargo test performance

# Specific performance tests
cargo test test_time_parsing_performance
cargo test test_progress_calculation_performance
cargo test test_render_progress_bar_performance
```

### Custom Benchmarking

```bash
# Install criterion (if adding custom benchmarks)
cargo install cargo-criterion

# Create benches/benchmark.rs (example)
cat > benches/benchmark.rs << 'EOF'
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pb::parse_time;

fn bench_time_parsing(c: &mut Criterion) {
    c.bench_function("parse_datetime", |b| {
        b.iter(|| parse_time(black_box("2025-01-27 12:00:00")))
    });
}

criterion_group!(benches, bench_time_parsing);
criterion_main!(benches);
EOF

# Run benchmarks
cargo bench
```

### Memory Profiling

```bash
# Install valgrind (Linux)
sudo apt-get install valgrind

# Memory leak detection
valgrind --leak-check=full --show-leak-kinds=all \
  ./target/release/pb --start "2025-01-27" --end "2025-01-28"

# Memory usage profiling
valgrind --tool=massif ./target/release/pb --help
```

### Binary Size Analysis

```bash
# Install cargo-bloat
cargo install cargo-bloat

# Analyze binary size
cargo bloat --release

# Top crates by size
cargo bloat --release --crates

# Filter analysis
cargo bloat --release --filter 1KB
```

## Documentation Generation

### API Documentation

```bash
# Generate documentation
cargo doc

# Generate with private items
cargo doc --document-private-items

# Open in browser
cargo doc --open

# Generate for all dependencies
cargo doc --no-deps
```

### Documentation Testing

```bash
# Test code examples in documentation
cargo test --doc

# Test specific module docs
cargo test --doc progress_bar
```

### External Documentation

```bash
# Build user guides (if using mdbook)
cargo install mdbook
mdbook build docs/

# Serve documentation locally
mdbook serve docs/ --port 3000
```

## Release Process

### Version Management

```bash
# Update version in Cargo.toml
sed -i 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml

# Update version in documentation
sed -i 's/pb 0.1.0/pb 0.2.0/' docs/man/pb.1

# Verify version consistency
grep -r "0.2.0" Cargo.toml docs/
```

### Pre-Release Checklist

```bash
# Run all tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Security audit
cargo audit

# Check for outdated dependencies
cargo outdated

# License compliance check (requires cargo-deny)
cargo deny check
```

### Release Build

```bash
# Clean build environment
cargo clean

# Production build for all targets
./scripts/build-release.sh  # (custom script)

# Verify binaries
for binary in target/*/release/pb*; do
    echo "Testing $binary:"
    "$binary" --version
    "$binary" --help > /dev/null
done
```

### Creating Release Artifacts

```bash
#!/bin/bash
# scripts/create-release.sh

VERSION=$(grep "^version" Cargo.toml | cut -d'"' -f2)
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "x86_64-pc-windows-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

mkdir -p dist/

for target in "${TARGETS[@]}"; do
    binary="target/$target/release/pb"
    if [[ "$target" == *"windows"* ]]; then
        binary="${binary}.exe"
    fi
    
    if [[ -f "$binary" ]]; then
        # Copy binary with target suffix
        cp "$binary" "dist/pb-$target"
        
        # Create tarball
        tar -czf "dist/pb-$VERSION-$target.tar.gz" \
            -C "target/$target/release" \
            pb$(if [[ "$target" == *"windows"* ]]; then echo ".exe"; fi)
        
        echo "Created dist/pb-$VERSION-$target.tar.gz"
    fi
done
```

## Deployment

### Package Creation

#### Debian Package

```bash
# Install cargo-deb
cargo install cargo-deb

# Create debian package
cargo deb

# Install locally
sudo dpkg -i target/debian/pb_*.deb
```

#### RPM Package

```bash
# Install cargo-rpm
cargo install cargo-rpm

# Initialize RPM spec
cargo rpm init

# Build RPM
cargo rpm build
```

#### Homebrew Formula

```ruby
# pb.rb (for Homebrew)
class Pb < Formula
  desc "CLI progress bar tool for time-based visualization"
  homepage "https://github.com/matsuokashuhei/pb"
  url "https://github.com/matsuokashuhei/pb/archive/v0.1.0.tar.gz"
  sha256 "..." # Calculate with: shasum -a 256 v0.1.0.tar.gz

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match "pb 0.1.0", shell_output("#{bin}/pb --version")
  end
end
```

### Container Deployment

#### Docker Image

```dockerfile
# Dockerfile.release
FROM alpine:latest

RUN apk --no-cache add ca-certificates
WORKDIR /root/

COPY target/x86_64-unknown-linux-musl/release/pb .

CMD ["./pb"]
```

```bash
# Build container
docker build -f Dockerfile.release -t pb:latest .

# Run container
docker run -it pb:latest --help
```

#### Kubernetes Deployment

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pb-deployment
spec:
  replicas: 1
  selector:
    matchLabels:
      app: pb
  template:
    metadata:
      labels:
        app: pb
    spec:
      containers:
      - name: pb
        image: pb:latest
        args: ["--start", "2025-01-27", "--end", "2025-01-28"]
```

### Binary Distribution

#### GitHub Releases

```bash
# Create GitHub release (using gh CLI)
gh release create v0.1.0 \
  --title "pb v0.1.0" \
  --notes "Initial release of pb CLI tool" \
  dist/pb-*.tar.gz
```

#### Direct Download

```bash
# Upload to CDN or file server
aws s3 cp dist/ s3://releases.example.com/pb/v0.1.0/ --recursive

# Create download script
cat > install.sh << 'EOF'
#!/bin/bash
set -e

VERSION="0.1.0"
ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

case "$OS-$ARCH" in
    linux-x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
    darwin-x86_64) TARGET="x86_64-apple-darwin" ;;
    darwin-arm64) TARGET="aarch64-apple-darwin" ;;
    *) echo "Unsupported platform: $OS-$ARCH"; exit 1 ;;
esac

URL="https://releases.example.com/pb/v$VERSION/pb-$TARGET"
curl -L "$URL" -o pb
chmod +x pb
sudo mv pb /usr/local/bin/
echo "pb installed successfully!"
EOF
```

## CI/CD Pipeline

### GitHub Actions Workflow

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
  release:
    types: [ created ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test Suite
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt --all -- --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Run tests
      run: cargo test --verbose
    
    - name: Run doc tests
      run: cargo test --doc

  build:
    name: Build Release
    needs: test
    if: github.event_name == 'release'
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
          - os: windows-latest
            target: x86_64-pc-windows-msvc
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
    
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        targets: ${{ matrix.target }}
    
    - name: Build
      run: cargo build --release --target ${{ matrix.target }}
    
    - name: Upload to release
      uses: softprops/action-gh-release@v1
      with:
        files: target/${{ matrix.target }}/release/pb*
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Alternative CI Platforms

#### GitLab CI

```yaml
# .gitlab-ci.yml
stages:
  - test
  - build
  - deploy

variables:
  CARGO_HOME: $CI_PROJECT_DIR/.cargo

cache:
  paths:
    - .cargo/
    - target/

test:
  stage: test
  image: rust:latest
  script:
    - cargo test --verbose
    - cargo clippy -- -D warnings
    - cargo fmt --all -- --check

build:
  stage: build
  image: rust:latest
  script:
    - cargo build --release
  artifacts:
    paths:
      - target/release/pb
    expire_in: 1 week
```

#### Travis CI

```yaml
# .travis.yml
language: rust
rust:
  - stable
  - beta
  - nightly

matrix:
  allow_failures:
    - rust: nightly

cache:
  - cargo

script:
  - cargo test --verbose
  - cargo fmt --all -- --check
  - cargo clippy -- -D warnings

deploy:
  provider: releases
  api_key: $GITHUB_TOKEN
  file: target/release/pb
  skip_cleanup: true
  on:
    tags: true
    rust: stable
```

### Deployment Automation

```bash
#!/bin/bash
# scripts/deploy.sh

set -e

VERSION=$(git describe --tags --exact-match 2>/dev/null || echo "dev")
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl" 
    "x86_64-pc-windows-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

echo "Deploying pb version: $VERSION"

# Build all targets
for target in "${TARGETS[@]}"; do
    echo "Building $target..."
    cargo build --release --target "$target"
done

# Create packages
./scripts/create-packages.sh

# Deploy to package repositories
if [[ "$VERSION" != "dev" ]]; then
    echo "Deploying packages..."
    
    # Deploy to GitHub releases
    gh release create "$VERSION" dist/*
    
    # Deploy to package managers
    ./scripts/deploy-packages.sh "$VERSION"
fi

echo "Deployment complete!"
```

This comprehensive build and deployment guide covers all aspects of building, testing, and deploying pb across different platforms and environments. It provides both manual processes and automated CI/CD pipeline configurations for scalable deployment.