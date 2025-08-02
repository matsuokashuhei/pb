# Installation Guide

This guide covers installation instructions for pmon (progress monitor tool) on all supported platforms.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation Methods](#installation-methods)
- [Platform-Specific Instructions](#platform-specific-instructions)
- [Building from Source](#building-from-source)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

## Prerequisites

pmon requires minimal system resources and dependencies:

- **Operating System**: Linux, macOS, Windows, or any Unix-like system
- **Terminal**: Any terminal emulator with ANSI color support (recommended)
- **System Requirements**:
  - 10MB available disk space
  - 10MB available RAM during execution

## Installation Methods

### Method 1: Pre-built Binaries (Recommended)

Download the appropriate binary for your platform from the [releases page](https://github.com/matsuokashuhei/pmon/releases).

#### Linux (x86_64)
```bash
# Download the latest release
curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-linux-x86_64

# Make it executable
chmod +x pmon

# Move to PATH (optional)
sudo mv pmon /usr/local/bin/
```

#### macOS (x86_64)
```bash
# Download the latest release
curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-macos-x86_64

# Make it executable
chmod +x pmon

# Move to PATH (optional)
sudo mv pmon /usr/local/bin/
```

#### macOS (ARM64/Apple Silicon)
```bash
# Download the latest release
curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-macos-arm64

# Make it executable
chmod +x pmon

# Move to PATH (optional)
sudo mv pmon /usr/local/bin/
```

#### Windows (x86_64)
1. Download `pmon-windows-x86_64.exe` from the releases page
2. Rename it to `pmon.exe`
3. Place it in a directory that's in your PATH, or create a new directory and add it to PATH

### Method 2: Package Managers

#### Homebrew (macOS/Linux)
```bash
# Add the tap (once available)
brew tap matsuokashuhei/pmon

# Install pmon
brew install pmon
```

#### Cargo (Rust Package Manager)
```bash
# Install from crates.io (once published)
cargo install pmon-cli

# Or install directly from git
cargo install --git https://github.com/matsuokashuhei/pmon.git
```

#### Snap (Linux)
```bash
# Install from Snap Store (once available)
sudo snap install pmon-progress-bar
```

### Method 3: Building from Source

See the [Building from Source](#building-from-source) section below.

## Platform-Specific Instructions

### Linux

#### Ubuntu/Debian
```bash
# Using pre-built binary
wget https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-linux-x86_64
chmod +x pmon-linux-x86_64
sudo mv pmon-linux-x86_64 /usr/local/bin/pmon

# Verify installation
pmon --version
```

#### CentOS/RHEL/Fedora
```bash
# Using pre-built binary
curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-linux-x86_64
chmod +x pmon
sudo mv pmon /usr/local/bin/

# Verify installation
pmon --version
```

#### Arch Linux
```bash
# Using pre-built binary
curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-linux-x86_64
chmod +x pmon
sudo mv pmon /usr/local/bin/

# Alternative: Build from AUR (once available)
# yay -S pmon-progress-bar
```

### macOS

#### Using Homebrew (Recommended)
```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install pmon (once available in Homebrew)
brew tap matsuokashuhei/pmon
brew install pmon
```

#### Manual Installation
```bash
# Determine your architecture
arch=$(uname -m)

if [ "$arch" = "arm64" ]; then
    # Apple Silicon (M1/M2/M3)
    curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-macos-arm64
else
    # Intel Mac
    curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-macos-x86_64
fi

chmod +x pmon
sudo mv pmon /usr/local/bin/

# Verify installation
pmon --version
```

### Windows

#### Using PowerShell
```powershell
# Create a directory for pmon (if it doesn't exist)
New-Item -ItemType Directory -Force -Path "C:\Program Files\pmon"

# Download the binary
Invoke-WebRequest -Uri "https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-windows-x86_64.exe" -OutFile "C:\Program Files\pmon\pmon.exe"

# Add to PATH (requires administrator privileges)
$env:PATH += ";C:\Program Files\pmon"
[Environment]::SetEnvironmentVariable("PATH", $env:PATH, [EnvironmentVariableTarget]::Machine)
```

#### Using Command Prompt
```cmd
# Create directory
mkdir "C:\Program Files\pmon"

# Download using curl (available in Windows 10+)
curl -L -o "C:\Program Files\pmon\pmon.exe" "https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-windows-x86_64.exe"

# Add to PATH manually through System Properties > Environment Variables
# Or use PowerShell command above
```

#### Using Chocolatey (once available)
```cmd
choco install pmon-progress-bar
```

#### Using Scoop (once available)
```cmd
scoop install pmon
```

### Unix-like Systems (FreeBSD, OpenBSD, etc.)

```bash
# Download and install binary
curl -L -o pmon https://github.com/matsuokashuhei/pmon/releases/latest/download/pmon-unix-x86_64
chmod +x pmon
sudo mv pmon /usr/local/bin/

# Or build from source (recommended for compatibility)
# See Building from Source section
```

## Building from Source

### Prerequisites for Building

- **Rust**: Version 1.70 or later
- **Git**: For cloning the repository
- **C Compiler**: GCC or Clang (usually pre-installed)

### Step-by-Step Build Instructions

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/matsuokashuhei/pmon.git
   cd pmon
   ```

3. **Build the project**:
   ```bash
   # Debug build (faster compilation, larger binary)
   cargo build

   # Release build (optimized, smaller binary)
   cargo build --release
   ```

4. **Install globally** (optional):
   ```bash
   # Install to ~/.cargo/bin (ensure it's in your PATH)
   cargo install --path .

   # Or copy the binary to a system directory
   sudo cp target/release/pmon /usr/local/bin/
   ```

### Cross-Compilation

To build for different platforms, you need to install the appropriate targets and configure cross-compilation tools.

#### Installing Targets

```bash
# Add target architectures
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add aarch64-unknown-linux-gnu
```

#### Building for Specific Targets

For simple targets (same architecture family):
```bash
# Build for specific targets
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

For cross-architecture compilation, you may need additional tools:
```bash
# For ARM64 on x86_64 Linux (requires cross-compilation toolchain)
sudo apt-get install gcc-aarch64-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

#### Using the Build Script

The repository includes a build script that simplifies target selection:

```bash
# Build for default target
./scripts/build.sh --release

# Build for specific target (will auto-install target if needed)
./scripts/build.sh --release --target aarch64-apple-darwin

# See all options
./scripts/build.sh --help
```

#### Avoiding Target Configuration Issues

If you encounter issues where `cargo build` produces binaries for the wrong target (e.g., Linux ARM64 instead of macOS ARM64), check:

1. **Rust toolchain default target**:
   ```bash
   rustup show
   ```

2. **Cargo configuration**: Check `.cargo/config.toml` for target overrides

3. **Environment variables**: Check for `CARGO_BUILD_TARGET` or similar

4. **Use explicit target**: Always specify the target explicitly:
   ```bash
   cargo build --release --target $(rustc -vV | sed -n 's|host: ||p')
   ```

5. **Verify binary after build**:
   ```bash
   file target/release/pmon
   # or
   file target/$(rustc -vV | sed -n 's|host: ||p')/release/pmon
   ```

### Development Build with Docker

If you prefer using Docker for a consistent build environment:

```bash
# Clone the repository
git clone https://github.com/matsuokashuhei/pmon.git
cd pmon

# Build using the development script
./scripts/build.sh --release

# The binary will be available in target/release/pmon
```

## Verification

After installation, verify that pmon is working correctly:

```bash
# Check version
pmon --version

# View help
pmon --help

# Test with a simple example
pmon --start "2025-01-27 12:00:00" --end "2025-01-27 13:00:00" --interval 5
```

Expected output should show:
- Version information when using `--version`
- Help text when using `--help`
- A progress bar when running the test command

## Troubleshooting

### Common Issues

#### "pmon: command not found"
**Cause**: pmon is not in your system PATH.

**Solutions**:
- Ensure the directory containing pmon is in your PATH
- Reinstall using a method that automatically adds to PATH
- Run pmon using its full path (e.g., `./pmon` or `/usr/local/bin/pmon`)

#### "Permission denied" when running pmon
**Cause**: The binary doesn't have execute permissions.

**Solution**:
```bash
chmod +x /path/to/pmon
```

#### Colors not displaying correctly
**Cause**: Terminal doesn't support ANSI colors or colors are disabled.

**Solutions**:
- Use a modern terminal emulator
- Check if colors are enabled in your terminal settings
- Try a different terminal (e.g., Terminal.app on macOS, Windows Terminal on Windows)

#### Binary won't run on Linux: "No such file or directory"
**Cause**: Missing dynamic libraries or architecture mismatch.

**Solutions**:
- Ensure you downloaded the correct architecture (x86_64 vs arm64)
- Install required libraries: `sudo apt-get install libc6` (Ubuntu/Debian)
- Consider building from source for better compatibility

#### Binary compatibility issue: Wrong target architecture
**Cause**: Binary was compiled for wrong target (e.g., Linux ARM64 binary on macOS ARM64).

**Symptoms**:
- `zsh: exec format error: ./pmon` on macOS
- `cannot execute binary file: Exec format error` on Linux
- `file pmon` shows wrong architecture/format

**Solutions**:
1. **Check the current binary**:
   ```bash
   file ./target/release/pmon
   ```

2. **Rebuild for correct target**:
   ```bash
   # For macOS ARM64 (Apple Silicon)
   cargo build --release --target aarch64-apple-darwin

   # For macOS x86_64 (Intel)
   cargo build --release --target x86_64-apple-darwin

   # For Linux x86_64
   cargo build --release --target x86_64-unknown-linux-gnu

   # For Linux ARM64
   cargo build --release --target aarch64-unknown-linux-gnu
   ```

3. **Use the build script**:
   ```bash
   # Auto-detects and builds for current platform
   ./scripts/build.sh --release

   # Or specify target explicitly
   ./scripts/build.sh --release --target aarch64-apple-darwin
   ```

4. **Check Rust toolchain configuration**:
   ```bash
   # Check default target
   rustup show

   # Check if target is available
   rustup target list --installed

   # Add missing target
   rustup target add aarch64-apple-darwin
   ```

5. **Verify `.cargo/config.toml`**: Ensure no incorrect target overrides

#### Windows Defender or antivirus blocking pmon
**Cause**: Some antivirus software may flag unknown binaries.

**Solutions**:
- Add pmon.exe to your antivirus whitelist
- Download from the official GitHub releases page
- Build from source to ensure authenticity

### Getting Help

If you encounter issues not covered here:

1. **Check the troubleshooting guide**: [docs/troubleshooting.md](troubleshooting.md)
2. **Search existing issues**: [GitHub Issues](https://github.com/matsuokashuhei/pmon/issues)
3. **Create a new issue**: Include your operating system, installation method, and error messages
4. **Check system requirements**: Ensure your system meets the minimum requirements

### Uninstallation

To remove pmon from your system:

#### Binary Installation
```bash
# Remove the binary
sudo rm /usr/local/bin/pmon

# Or remove from custom location
rm /path/to/pmon
```

#### Cargo Installation
```bash
cargo uninstall pmon-cli
```

#### Package Manager Installation
```bash
# Homebrew
brew uninstall pmon

# Chocolatey
choco uninstall pmon-progress-bar

# Snap
sudo snap remove pmon-progress-bar
```

## Next Steps

After successful installation:

1. Read the [User Guide](user_guide.md) for detailed usage instructions
2. Check out [examples](examples/) for common use cases
3. Review the [man page](man/pmon.1) for quick reference
4. See the [troubleshooting guide](troubleshooting.md) for common issues

For development and contribution, see the [Development Guide](development_guide.md).
