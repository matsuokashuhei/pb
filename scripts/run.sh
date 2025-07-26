#!/bin/bash

# Run script for pb project using Docker

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
    echo "  --                Pass remaining arguments to pb command"
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
            echo "Use -- to separate script options from pb arguments"
            usage
            ;;
    esac
done

# Determine binary path based on build type
if [ "$BUILD_TYPE" = "release" ]; then
    BINARY_PATH="/app/target/release/pb"
else
    BINARY_PATH="/app/target/debug/pb"
fi

# Build Docker image if it doesn't exist or Dockerfile is newer
IMAGE_NAME="pb-dev"
if [ ! "$(docker images -q $IMAGE_NAME 2> /dev/null)" ] || [ "Dockerfile" -nt "$(docker inspect -f '{{.Created}}' $IMAGE_NAME 2>/dev/null)" ]; then
    echo -e "${YELLOW}Building development Docker image...${NC}"
    docker build -t $IMAGE_NAME --target development . > /dev/null
fi

echo -e "${YELLOW}Running pb (${BUILD_TYPE} build)...${NC}"

# Docker run command with volume mounts and TTY for interactive progress bar using our built image
docker run --rm -it \
    -v $(pwd):/app \
    -v pb-cargo-cache:/usr/local/cargo/registry \
    -v pb-target-cache:/app/target \
    -w /app \
    $IMAGE_NAME \
    $BINARY_PATH "${PB_ARGS[@]}"
