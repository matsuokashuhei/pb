# Installation Guide

This guide covers installation instructions for pb (progress bar tool) on all supported platforms.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation Methods](#installation-methods)
- [Platform-Specific Instructions](#platform-specific-instructions)
- [Building from Source](#building-from-source)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

## Prerequisites

pb requires minimal system resources and dependencies:

- **Operating System**: Linux, macOS, Windows, or any Unix-like system
- **Terminal**: Any terminal emulator with ANSI color support (recommended)
- **System Requirements**: 
  - 10MB available disk space
  - 10MB available RAM during execution

## Installation Methods

### Method 1: Pre-built Binaries (Recommended)

Download the appropriate binary for your platform from the [releases page](https://github.com/matsuokashuhei/pb/releases).

#### Linux (x86_64)
```bash
# Download the latest release
curl -L -o pb https://github.com/matsuokashuhei/pb/releases/latest/download/pb-linux-x86_64

# Make it executable
chmod +x pb

# Move to PATH (optional)
sudo mv pb /usr/local/bin/
```

#### macOS (x86_64)
```bash
# Download the latest release
curl -L -o pb https://github.com/matsuokashuhei/pb/releases/latest/download/pb-macos-x86_64

# Make it executable
chmod +x pb

# Move to PATH (optional)
sudo mv pb /usr/local/bin/
```

#### macOS (ARM64/Apple Silicon)
```bash
# Download the latest release
curl -L -o pb https://github.com/matsuokashuhei/pb/releases/latest/download/pb-macos-arm64

# Make it executable
chmod +x pb

# Move to PATH (optional)
sudo mv pb /usr/local/bin/
```

#### Windows (x86_64)
1. Download `pb-windows-x86_64.exe` from the releases page
2. Rename it to `pb.exe`
3. Place it in a directory that's in your PATH, or create a new directory and add it to PATH

### Method 2: Package Managers

#### Homebrew (macOS/Linux)
```bash
# Add the tap (once available)
brew tap matsuokashuhei/pb

# Install pb
brew install pb
```

#### Cargo (Rust Package Manager)
```bash
# Install from crates.io (once published)
cargo install pb-cli

# Or install directly from git
cargo install --git https://github.com/matsuokashuhei/pb.git
```

#### Snap (Linux)
```bash
# Install from Snap Store (once available)
sudo snap install pb-progress-bar
```

### Method 3: Building from Source

See the [Building from Source](#building-from-source) section below.

## Platform-Specific Instructions

### Linux

#### Ubuntu/Debian
```bash
# Using pre-built binary
wget https://github.com/matsuokashuhei/pb/releases/latest/download/pb-linux-x86_64
chmod +x pb-linux-x86_64
sudo mv pb-linux-x86_64 /usr/local/bin/pb

# Verify installation
pb --version
```

#### CentOS/RHEL/Fedora
```bash
# Using pre-built binary
curl -L -o pb https://github.com/matsuokashuhei/pb/releases/latest/download/pb-linux-x86_64
chmod +x pb
sudo mv pb /usr/local/bin/

# Verify installation
pb --version
```

#### Arch Linux
```bash
# Using pre-built binary
curl -L -o pb https://github.com/matsuokashuhei/pb/releases/latest/download/pb-linux-x86_64
chmod +x pb
sudo mv pb /usr/local/bin/

# Alternative: Build from AUR (once available)
# yay -S pb-progress-bar
```

### macOS

#### Using Homebrew (Recommended)
```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install pb (once available in Homebrew)
brew tap matsuokashuhei/pb
brew install pb
```

#### Manual Installation
```bash
# Determine your architecture
arch=$(uname -m)

if [ "$arch" = "arm64" ]; then
    # Apple Silicon (M1/M2/M3)
    curl -L -o pb https://github.com/matsuokashuhei/pb/releases/latest/download/pb-macos-arm64
else
    # Intel Mac
    curl -L -o pb https://github.com/matsuokashuhei/pb/releases/latest/download/pb-macos-x86_64
fi

chmod +x pb
sudo mv pb /usr/local/bin/

# Verify installation
pb --version
```

### Windows

#### Using PowerShell
```powershell
# Create a directory for pb (if it doesn't exist)
New-Item -ItemType Directory -Force -Path "C:\Program Files\pb"

# Download the binary
Invoke-WebRequest -Uri "https://github.com/matsuokashuhei/pb/releases/latest/download/pb-windows-x86_64.exe" -OutFile "C:\Program Files\pb\pb.exe"

# Add to PATH (requires administrator privileges)
$env:PATH += ";C:\Program Files\pb"
[Environment]::SetEnvironmentVariable("PATH", $env:PATH, [EnvironmentVariableTarget]::Machine)
```

#### Using Command Prompt
```cmd
# Create directory
mkdir "C:\Program Files\pb"

# Download using curl (available in Windows 10+)
curl -L -o "C:\Program Files\pb\pb.exe" "https://github.com/matsuokashuhei/pb/releases/latest/download/pb-windows-x86_64.exe"

# Add to PATH manually through System Properties > Environment Variables
# Or use PowerShell command above
```

#### Using Chocolatey (once available)
```cmd
choco install pb-progress-bar
```

#### Using Scoop (once available)
```cmd
scoop install pb
```

### Unix-like Systems (FreeBSD, OpenBSD, etc.)

```bash
# Download and install binary
curl -L -o pb https://github.com/matsuokashuhei/pb/releases/latest/download/pb-unix-x86_64
chmod +x pb
sudo mv pb /usr/local/bin/

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
   git clone https://github.com/matsuokashuhei/pb.git
   cd pb
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
   sudo cp target/release/pb /usr/local/bin/
   ```

### Cross-Compilation

To build for different platforms:

```bash
# Add target architectures
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for specific targets
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

### Development Build with Docker

If you prefer using Docker for a consistent build environment:

```bash
# Clone the repository
git clone https://github.com/matsuokashuhei/pb.git
cd pb

# Build using the development script
./scripts/build.sh --release

# The binary will be available in target/release/pb
```

## Verification

After installation, verify that pb is working correctly:

```bash
# Check version
pb --version

# View help
pb --help

# Test with a simple example
pb --start "2025-01-27 12:00:00" --end "2025-01-27 13:00:00" --interval 5
```

Expected output should show:
- Version information when using `--version`
- Help text when using `--help`
- A progress bar when running the test command

## Troubleshooting

### Common Issues

#### "pb: command not found"
**Cause**: pb is not in your system PATH.

**Solutions**:
- Ensure the directory containing pb is in your PATH
- Reinstall using a method that automatically adds to PATH
- Run pb using its full path (e.g., `./pb` or `/usr/local/bin/pb`)

#### "Permission denied" when running pb
**Cause**: The binary doesn't have execute permissions.

**Solution**:
```bash
chmod +x /path/to/pb
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

#### Windows Defender or antivirus blocking pb
**Cause**: Some antivirus software may flag unknown binaries.

**Solutions**:
- Add pb.exe to your antivirus whitelist
- Download from the official GitHub releases page
- Build from source to ensure authenticity

### Getting Help

If you encounter issues not covered here:

1. **Check the troubleshooting guide**: [docs/troubleshooting.md](troubleshooting.md)
2. **Search existing issues**: [GitHub Issues](https://github.com/matsuokashuhei/pb/issues)
3. **Create a new issue**: Include your operating system, installation method, and error messages
4. **Check system requirements**: Ensure your system meets the minimum requirements

### Uninstallation

To remove pb from your system:

#### Binary Installation
```bash
# Remove the binary
sudo rm /usr/local/bin/pb

# Or remove from custom location
rm /path/to/pb
```

#### Cargo Installation
```bash
cargo uninstall pb-cli
```

#### Package Manager Installation
```bash
# Homebrew
brew uninstall pb

# Chocolatey
choco uninstall pb-progress-bar

# Snap
sudo snap remove pb-progress-bar
```

## Next Steps

After successful installation:

1. Read the [User Guide](user_guide.md) for detailed usage instructions
2. Check out [examples](examples/) for common use cases
3. Review the [man page](man/pb.1) for quick reference
4. See the [troubleshooting guide](troubleshooting.md) for common issues

For development and contribution, see the [Development Guide](development_guide.md).