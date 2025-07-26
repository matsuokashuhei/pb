#!/bin/bash

# Native build script for pb project (without Docker)
# This script builds pb directly using the local Rust toolchain

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
BUILD_TYPE="debug"
VERBOSE=false
TARGET=""

# Function to show usage
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo "  -r, --release     Build in release mode"
    echo "  -v, --verbose     Verbose output"
    echo "  -t, --target TARGET"
    echo "                    Specify target architecture (e.g., aarch64-apple-darwin, x86_64-unknown-linux-gnu)"
    echo "  -h, --help        Show this help message"
    echo ""
    echo "Common targets:"
    echo "  x86_64-unknown-linux-gnu    Linux x86_64"
    echo "  aarch64-unknown-linux-gnu   Linux ARM64"
    echo "  x86_64-apple-darwin         macOS x86_64 (Intel)"
    echo "  aarch64-apple-darwin        macOS ARM64 (Apple Silicon)"
    echo "  x86_64-pc-windows-msvc      Windows x86_64"
    echo ""
    echo "Note: Make sure the target is installed with 'rustup target add <target>'"
    exit 1
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -r|--release)
            BUILD_TYPE="release"
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -t|--target)
            TARGET="$2"
            shift 2
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            usage
            ;;
    esac
done

# Build arguments
BUILD_ARGS=""
if [ "$BUILD_TYPE" = "release" ]; then
    BUILD_ARGS="--release"
fi
if [ "$VERBOSE" = true ]; then
    BUILD_ARGS="$BUILD_ARGS --verbose"
fi
if [ -n "$TARGET" ]; then
    BUILD_ARGS="$BUILD_ARGS --target $TARGET"
    echo -e "${YELLOW}Building pb for target: $TARGET${NC}"
    
    # Check if target is installed
    if ! rustup target list --installed | grep -q "^$TARGET$"; then
        echo -e "${YELLOW}Target $TARGET is not installed. Installing...${NC}"
        rustup target add "$TARGET"
    fi
else
    echo -e "${YELLOW}Building pb for default target${NC}"
fi

echo -e "${YELLOW}Building pb in ${BUILD_TYPE} mode...${NC}"
echo -e "${YELLOW}Build command: cargo build $BUILD_ARGS${NC}"

# Execute build command
cargo build $BUILD_ARGS

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build completed successfully!${NC}"
    
    # Show binary information
    if [ -n "$TARGET" ]; then
        BINARY_PATH="target/$TARGET/$BUILD_TYPE/pb"
    else
        BINARY_PATH="target/$BUILD_TYPE/pb"
    fi
    
    if [ -f "$BINARY_PATH" ]; then
        echo -e "${GREEN}Binary created at: $BINARY_PATH${NC}"
        echo -e "${YELLOW}Binary info:${NC}"
        file "$BINARY_PATH"
    fi
else
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi