#!/bin/bash

# Build script for pb project using Docker

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
BUILD_TYPE="debug"
VERBOSE=false

# Function to show usage
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo "  -r, --release     Build in release mode"
    echo "  -v, --verbose     Verbose output"
    echo "  -h, --help        Show this help message"
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
        -h|--help)
            usage
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            usage
            ;;
    esac
done

echo -e "${YELLOW}Building pb in ${BUILD_TYPE} mode...${NC}"

# Build Docker image if it doesn't exist or Dockerfile is newer
IMAGE_NAME="pb-dev"
if [ ! "$(docker images -q $IMAGE_NAME 2> /dev/null)" ] || [ "Dockerfile" -nt "$(docker inspect -f '{{.Created}}' $IMAGE_NAME 2>/dev/null)" ]; then
    echo -e "${YELLOW}Building development Docker image...${NC}"
    docker build -t $IMAGE_NAME --target development . > /dev/null
fi

# Docker run command with volume mounts using our built image
DOCKER_CMD="docker run --rm \
    -v $(pwd):/app \
    -v pb-cargo-cache:/usr/local/cargo/registry \
    -v pb-target-cache:/app/target \
    -w /app \
    $IMAGE_NAME"

if [ "$BUILD_TYPE" = "release" ]; then
    if [ "$VERBOSE" = true ]; then
        $DOCKER_CMD cargo build --release --verbose
    else
        $DOCKER_CMD cargo build --release
    fi
else
    if [ "$VERBOSE" = true ]; then
        $DOCKER_CMD cargo build --verbose
    else
        $DOCKER_CMD cargo build
    fi
fi

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build completed successfully!${NC}"
else
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi
