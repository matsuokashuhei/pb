#!/bin/bash

# Build script for pmon project
# This script builds pmon using either the local Rust toolchain or Docker
# It prefers local cargo for better performance but falls back to Docker when needed

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
FORCE_DOCKER=false

# Function to show usage
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo "  -r, --release     Build in release mode"
    echo "  -v, --verbose     Verbose output"
    echo "  -t, --target TARGET"
    echo "                    Specify target architecture (e.g., aarch64-apple-darwin, x86_64-unknown-linux-gnu)"
    echo "  -d, --docker      Force use of Docker even if cargo is available locally"
    echo "  -h, --help        Show this help message"
    echo ""
    echo "Common targets:"
    echo "  x86_64-unknown-linux-gnu    Linux x86_64"
    echo "  aarch64-unknown-linux-gnu   Linux ARM64"
    echo "  x86_64-apple-darwin         macOS x86_64 (Intel)"
    echo "  aarch64-apple-darwin        macOS ARM64 (Apple Silicon)"
    echo "  x86_64-pc-windows-msvc      Windows x86_64"
    echo ""
    echo "Note: When using local cargo, make sure the target is installed with 'rustup target add <target>'"
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
        -d|--docker)
            FORCE_DOCKER=true
            shift
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
    echo -e "${YELLOW}Building pmon for target: $TARGET${NC}"
else
    echo -e "${YELLOW}Building pmon for default target${NC}"
fi

# Determine build method: local cargo or Docker
USE_DOCKER=false
if [ "$FORCE_DOCKER" = true ]; then
    echo -e "${YELLOW}Docker build forced by --docker flag${NC}"
    USE_DOCKER=true

    # Warn about cross-compilation limitations in Docker
    if [ -n "$TARGET" ]; then
        echo -e "${YELLOW}Warning: Cross-compilation in Docker containers is complex and may not work${NC}"
        echo -e "${YELLOW}For best cross-compilation support, consider using local cargo instead${NC}"
        echo -e "${YELLOW}Attempting to proceed with Docker build...${NC}"
    fi
elif ! command -v cargo >/dev/null 2>&1; then
    echo -e "${YELLOW}cargo command not found locally. Checking for Docker...${NC}"

    if ! command -v docker >/dev/null 2>&1; then
        echo -e "${RED}Error: Neither cargo nor docker command found!${NC}"
        echo -e "${RED}Please install either:${NC}"
        echo -e "${YELLOW}  1. Rust and Cargo: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
        echo -e "${YELLOW}  2. Docker: https://docs.docker.com/get-docker/${NC}"
        exit 1
    fi

    echo -e "${GREEN}Docker found! Will use Docker container for building.${NC}"
    USE_DOCKER=true

    # Warn about cross-compilation limitations in Docker
    if [ -n "$TARGET" ]; then
        echo -e "${YELLOW}Warning: Cross-compilation in Docker containers is complex and may not work${NC}"
        echo -e "${YELLOW}For best cross-compilation support, consider installing Rust locally${NC}"
        echo -e "${YELLOW}Attempting to proceed with Docker build...${NC}"
    fi
else
    echo -e "${GREEN}cargo is available locally! Using local build for better performance and cross-compilation support.${NC}"

    # Check if target is installed when using local cargo
    if [ -n "$TARGET" ]; then
        if ! rustup target list --installed | grep -q "^$TARGET$"; then
            echo -e "${YELLOW}Target $TARGET is not installed. Installing...${NC}"
            rustup target add "$TARGET"
        fi
    fi
fi

echo -e "${YELLOW}Building pmon in ${BUILD_TYPE} mode...${NC}"

# Execute build command
if [ "$USE_DOCKER" = true ]; then
    # Check if we should use the optimized Docker setup or simple rust:latest
    if command -v docker >/dev/null 2>&1; then
        # Try to use the optimized setup first
        IMAGE_NAME="pmon-dev"
        if docker images -q $IMAGE_NAME 2>/dev/null | grep -q .; then
            echo -e "${YELLOW}Using cached development Docker image for optimized build...${NC}"

            # Install target if specified and using Docker
            if [ -n "$TARGET" ]; then
                echo -e "${YELLOW}Installing target $TARGET in Docker container...${NC}"
                docker run --rm \
                    -v "$(pwd):/app" \
                    -v pmon-cargo-cache:/usr/local/cargo/registry \
                    -v pmon-target-cache:/app/target \
                    -w /app \
                    $IMAGE_NAME rustup target add "$TARGET" || echo -e "${YELLOW}Target installation completed (may have warnings)${NC}"
            fi

            echo -e "${YELLOW}Build command: docker run --rm -v $(pwd):/app -v pmon-cargo-cache:/usr/local/cargo/registry -v pmon-target-cache:/app/target -w /app $IMAGE_NAME cargo build $BUILD_ARGS${NC}"

            # Docker run command with volume mounts using cached image
            docker run --rm \
                -v "$(pwd):/app" \
                -v pmon-cargo-cache:/usr/local/cargo/registry \
                -v pmon-target-cache:/app/target \
                -w /app \
                $IMAGE_NAME cargo build $BUILD_ARGS
            BUILD_EXIT_CODE=$?
        else
            # Build the optimized image if it doesn't exist
            if [ -f "Dockerfile" ]; then
                echo -e "${YELLOW}Building optimized development Docker image...${NC}"
                docker build -t $IMAGE_NAME --target development . > /dev/null

                # Install target if specified and using Docker
                if [ -n "$TARGET" ]; then
                    echo -e "${YELLOW}Installing target $TARGET in Docker container...${NC}"
                    docker run --rm \
                        -v "$(pwd):/app" \
                        -v pmon-cargo-cache:/usr/local/cargo/registry \
                        -v pmon-target-cache:/app/target \
                        -w /app \
                        $IMAGE_NAME rustup target add "$TARGET" || echo -e "${YELLOW}Target installation completed (may have warnings)${NC}"
                fi

                echo -e "${YELLOW}Build command: docker run --rm -v $(pwd):/app -v pmon-cargo-cache:/usr/local/cargo/registry -v pmon-target-cache:/app/target -w /app $IMAGE_NAME cargo build $BUILD_ARGS${NC}"

                # Docker run command with volume mounts using built image
                docker run --rm \
                    -v "$(pwd):/app" \
                    -v pmon-cargo-cache:/usr/local/cargo/registry \
                    -v pmon-target-cache:/app/target \
                    -w /app \
                    $IMAGE_NAME cargo build $BUILD_ARGS
                BUILD_EXIT_CODE=$?
            else
                # Fallback to simple rust:latest image
                echo -e "${YELLOW}No Dockerfile found, using simple rust:latest image...${NC}"

                # Install target if specified and using Docker
                if [ -n "$TARGET" ]; then
                    echo -e "${YELLOW}Installing target $TARGET in Docker container...${NC}"
                    docker run --rm -v "$(pwd):/app" -w /app rust:latest rustup target add "$TARGET" || echo -e "${YELLOW}Target installation completed (may have warnings)${NC}"
                fi

                echo -e "${YELLOW}Build command: docker run --rm -v $(pwd):/app -w /app rust:latest cargo build $BUILD_ARGS${NC}"

                docker run --rm -v "$(pwd):/app" -w /app rust:latest cargo build $BUILD_ARGS
                BUILD_EXIT_CODE=$?
            fi
        fi
    fi
else
    echo -e "${YELLOW}Build command: cargo build $BUILD_ARGS${NC}"

    # Use local cargo
    cargo build $BUILD_ARGS
    BUILD_EXIT_CODE=$?
fi

if [ $BUILD_EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}Build completed successfully!${NC}"

    # Show binary information
    if [ -n "$TARGET" ]; then
        BINARY_PATH="target/$TARGET/$BUILD_TYPE/pmon"
    else
        BINARY_PATH="target/$BUILD_TYPE/pmon"
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
