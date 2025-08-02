#!/bin/bash

# Run script for pmon project using Docker

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
BUILD_TYPE="debug"

# Function to show usage
usage() {
    echo "Usage: $0 [OPTIONS] [-- ARGS]"
    echo "Options:"
    echo "  -r, --release     Use release build"
    echo "  -h, --help        Show this help message"
    echo "  --                Pass remaining arguments to pmon command"
    echo ""
    echo "Examples:"
    echo "  $0 -- --help"
    echo "  $0 -- --start \"2025-07-21 10:00:00\" --end \"2025-07-21 18:00:00\""
    echo "  $0 -r -- --start \"2025-07-21\" --end \"1d\""
    exit 1
}

# Parse command line arguments
PB_ARGS=()
while [[ $# -gt 0 ]]; do
    case $1 in
        -r|--release)
            BUILD_TYPE="release"
            shift
            ;;
        -h|--help)
            usage
            ;;
        --)
            shift
            PB_ARGS=("$@")
            break
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use -- to separate script options from pmon arguments"
            usage
            ;;
    esac
done

# Determine binary path based on build type
if [ "$BUILD_TYPE" = "release" ]; then
    BINARY_PATH="/app/target/release/pmon"
else
    BINARY_PATH="/app/target/debug/pmon"
fi

# Try to use Docker, but fallback to local build if Docker setup fails
IMAGE_NAME="pmon-dev"
USE_DOCKER=true

# Check if Docker image exists or try to build it
if [ ! "$(docker images -q $IMAGE_NAME 2> /dev/null)" ]; then
    echo -e "${YELLOW}Building development Docker image...${NC}"
    if ! docker build -t $IMAGE_NAME --target development . > /dev/null 2>&1; then
        echo -e "${YELLOW}Docker build failed, falling back to local execution...${NC}"
        USE_DOCKER=false
    fi
elif [ "Dockerfile" -nt "$(docker inspect -f '{{.Created}}' $IMAGE_NAME 2>/dev/null)" ]; then
    echo -e "${YELLOW}Rebuilding development Docker image...${NC}"
    if ! docker build -t $IMAGE_NAME --target development . > /dev/null 2>&1; then
        echo -e "${YELLOW}Docker build failed, falling back to local execution...${NC}"
        USE_DOCKER=false
    fi
fi

echo -e "${YELLOW}Running pmon (${BUILD_TYPE} build)...${NC}"

if [ "$USE_DOCKER" = true ]; then
    # Build the binary with current source code in the container first
    echo -e "${YELLOW}Building binary with current source code in Docker...${NC}"
    docker run --rm \
        -v $(pwd):/app \
        -v pmon-cargo-cache:/usr/local/cargo/registry \
        -v pmon-target-cache:/app/target \
        -w /app \
        $IMAGE_NAME \
        sh -c "if [ '$BUILD_TYPE' = 'release' ]; then cargo build --release; else cargo build; fi"

    if [ $? -ne 0 ]; then
        echo -e "${RED}Docker build failed, trying local build...${NC}"
        USE_DOCKER=false
    else
        # Docker run command with volume mounts and TTY for interactive progress bar
        docker run --rm -it \
            -v $(pwd):/app \
            -v pmon-cargo-cache:/usr/local/cargo/registry \
            -v pmon-target-cache:/app/target \
            -w /app \
            $IMAGE_NAME \
            $BINARY_PATH "${PB_ARGS[@]}"
        exit $?
    fi
fi

# Fallback to local execution
echo -e "${YELLOW}Using local Rust toolchain...${NC}"

# Build locally if needed
if [ "$BUILD_TYPE" = "release" ]; then
    LOCAL_BINARY_PATH="./target/release/pmon"
    if [ ! -f "$LOCAL_BINARY_PATH" ] || [ "src/" -nt "$LOCAL_BINARY_PATH" ]; then
        echo -e "${YELLOW}Building release binary locally...${NC}"
        cargo build --release
        if [ $? -ne 0 ]; then
            echo -e "${RED}Local build failed${NC}"
            exit 1
        fi
    fi
else
    LOCAL_BINARY_PATH="./target/debug/pmon"
    if [ ! -f "$LOCAL_BINARY_PATH" ] || [ "src/" -nt "$LOCAL_BINARY_PATH" ]; then
        echo -e "${YELLOW}Building debug binary locally...${NC}"
        cargo build
        if [ $? -ne 0 ]; then
            echo -e "${RED}Local build failed${NC}"
            exit 1
        fi
    fi
fi

# Run the local binary
$LOCAL_BINARY_PATH "${PB_ARGS[@]}"
