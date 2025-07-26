#!/bin/bash

# Target detection and verification script for pb project
# This script helps diagnose target-related issues

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== pb Target Detection and Verification ===${NC}"
echo

# Show current system information
echo -e "${YELLOW}System Information:${NC}"
echo "Operating System: $(uname -s)"
echo "Architecture: $(uname -m)"
echo "Kernel: $(uname -r)"
if [ -f /etc/os-release ]; then
    echo "OS Release: $(grep PRETTY_NAME /etc/os-release | cut -d'"' -f2)"
fi
echo

# Show Rust toolchain information
echo -e "${YELLOW}Rust Toolchain Information:${NC}"
if command -v rustc &> /dev/null; then
    echo "Rust version: $(rustc --version)"
    echo "Default host target: $(rustc -vV | grep 'host:' | cut -d' ' -f2)"
    echo
    
    echo "Installed targets:"
    rustup target list --installed | while read target; do
        echo "  - $target"
    done
    echo
else
    echo -e "${RED}Rust is not installed or not in PATH${NC}"
    exit 1
fi

# Show Cargo configuration
echo -e "${YELLOW}Cargo Configuration:${NC}"
if [ -f ".cargo/config.toml" ]; then
    echo "Found .cargo/config.toml:"
    if grep -q "^target\s*=" .cargo/config.toml; then
        echo "  - Target override: $(grep '^target\s*=' .cargo/config.toml | head -1)"
    else
        echo "  - No target override (uses default)"
    fi
else
    echo "No .cargo/config.toml found (uses global configuration)"
fi
echo

# Check environment variables
echo -e "${YELLOW}Environment Variables:${NC}"
env_vars_found=false
for var in CARGO_BUILD_TARGET TARGET RUST_TARGET; do
    if [ -n "${!var}" ]; then
        echo "  - $var=${!var}"
        env_vars_found=true
    fi
done
if [ "$env_vars_found" = false ]; then
    echo "  - No target-related environment variables set"
fi
echo

# Test build target detection
echo -e "${YELLOW}Build Target Detection:${NC}"
current_target=$(rustc -vV | grep 'host:' | cut -d' ' -f2)
echo "Expected build target: $current_target"

# Check if binary exists and show information
if [ -f "target/release/pb" ]; then
    echo -e "${GREEN}Found existing release binary:${NC}"
    echo "  - Path: target/release/pb"
    echo "  - Size: $(ls -lh target/release/pb | awk '{print $5}')"
    echo "  - File type: $(file target/release/pb | cut -d: -f2- | sed 's/^ *//')"
    echo
    
    # Test if binary works
    echo -e "${YELLOW}Binary Test:${NC}"
    if ./target/release/pb --version >/dev/null 2>&1; then
        echo -e "${GREEN}  ✓ Binary executes successfully${NC}"
        echo "  Version: $(./target/release/pb --version)"
    else
        echo -e "${RED}  ✗ Binary execution failed${NC}"
    fi
else
    echo -e "${YELLOW}No release binary found at target/release/pb${NC}"
    echo "Run 'cargo build --release' or './scripts/build-native.sh --release' to build"
fi
echo

# Check for target-specific binaries
echo -e "${YELLOW}Target-specific Binaries:${NC}"
target_dirs_found=false
for target_dir in target/*/release/pb; do
    if [ -f "$target_dir" ]; then
        target_name=$(basename $(dirname $(dirname $target_dir)))
        echo "  - $target_name: $(file $target_dir | cut -d: -f2- | sed 's/^ *//')"
        target_dirs_found=true
    fi
done
if [ "$target_dirs_found" = false ]; then
    echo "  - No target-specific binaries found"
fi
echo

# Provide recommendations
echo -e "${YELLOW}Recommendations:${NC}"
echo "1. To build for current system:"
echo "   cargo build --release"
echo 
echo "2. To build for specific target:"
echo "   cargo build --release --target $current_target"
echo
echo "3. To use the native build script:"
echo "   ./scripts/build-native.sh --release"
echo
echo "4. To verify binary compatibility:"
echo "   file target/release/pb"
echo "   ./target/release/pb --version"
echo

echo -e "${BLUE}=== End of Verification ===${NC}"