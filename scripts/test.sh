#!/bin/bash

# Test script for pb project using Docker

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
TEST_TYPE="all"
VERBOSE=false

# Function to show usage
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo "  -u, --unit        Run unit tests only"
    echo "  -i, --integration Run integration tests only"
    echo "  -d, --doc         Run doc tests"
    echo "  -v, --verbose     Verbose output"
    echo "  -h, --help        Show this help message"
    echo "  --                Pass remaining arguments to cargo test"
    exit 1
}

# Parse command line arguments
CARGO_ARGS=""
while [[ $# -gt 0 ]]; do
    case $1 in
        -u|--unit)
            TEST_TYPE="unit"
            shift
            ;;
        -i|--integration)
            TEST_TYPE="integration"
            shift
            ;;
        -d|--doc)
            TEST_TYPE="doc"
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            usage
            ;;
        --)
            shift
            CARGO_ARGS="$@"
            break
            ;;
        *)
            CARGO_ARGS="$CARGO_ARGS $1"
            shift
            ;;
    esac
done

echo -e "${YELLOW}Running tests...${NC}"

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

# Build test command based on type
case $TEST_TYPE in
    "unit")
        TEST_CMD="cargo test --lib"
        ;;
    "integration")
        TEST_CMD="cargo test --test '*'"
        ;;
    "doc")
        TEST_CMD="cargo test --doc"
        ;;
    *)
        TEST_CMD="cargo test"
        ;;
esac

# Add verbose flag if requested
if [ "$VERBOSE" = true ]; then
    TEST_CMD="$TEST_CMD --verbose"
fi

# Add any additional cargo arguments
if [ ! -z "$CARGO_ARGS" ]; then
    TEST_CMD="$TEST_CMD $CARGO_ARGS"
fi

echo -e "${YELLOW}Executing: $TEST_CMD${NC}"

$DOCKER_CMD $TEST_CMD

if [ $? -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
