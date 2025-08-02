#!/bin/bash

# Development environment script for pmon project using Docker

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to show usage
usage() {
    echo "Usage: $0 [COMMAND]"
    echo "Commands:"
    echo "  shell             Start interactive development shell"
    echo "  clean             Clean build artifacts and caches"
    echo "  deps              Install/update dependencies"
    echo "  fmt               Format code"
    echo "  clippy            Run clippy linter"
    echo "  check             Quick check without building"
    exit 1
}

# Build Docker image if it doesn't exist or Dockerfile is newer
IMAGE_NAME="pmon-dev"
if [ ! "$(docker images -q $IMAGE_NAME 2> /dev/null)" ] || [ "Dockerfile" -nt "$(docker inspect -f '{{.Created}}' $IMAGE_NAME 2>/dev/null)" ]; then
    echo -e "${YELLOW}Building development Docker image...${NC}"
    docker build -t $IMAGE_NAME --target development . > /dev/null
fi

# Docker run command with volume mounts using our built image
DOCKER_CMD="docker run --rm -it \
    -v $(pwd):/app \
    -v pmon-cargo-cache:/usr/local/cargo/registry \
    -v pmon-target-cache:/app/target \
    -w /app \
    $IMAGE_NAME"

# Parse command
case "${1:-shell}" in
    "shell")
        echo -e "${YELLOW}Starting development shell...${NC}"
        $DOCKER_CMD bash
        ;;
    "clean")
        echo -e "${YELLOW}Cleaning build artifacts...${NC}"
        $DOCKER_CMD cargo clean
        echo -e "${YELLOW}Removing Docker volumes...${NC}"
        docker volume rm pmon-cargo-cache pmon-target-cache 2>/dev/null || true
        echo -e "${GREEN}Clean completed!${NC}"
        ;;
    "deps")
        echo -e "${YELLOW}Installing/updating dependencies...${NC}"
        $DOCKER_CMD cargo fetch
        echo -e "${GREEN}Dependencies updated!${NC}"
        ;;
    "fmt")
        echo -e "${YELLOW}Formatting code...${NC}"
        $DOCKER_CMD cargo fmt
        echo -e "${GREEN}Code formatted!${NC}"
        ;;
    "clippy")
        echo -e "${YELLOW}Running clippy...${NC}"
        $DOCKER_CMD cargo clippy -- -D warnings
        echo -e "${GREEN}Clippy check completed!${NC}"
        ;;
    "check")
        echo -e "${YELLOW}Running cargo check...${NC}"
        $DOCKER_CMD cargo check
        echo -e "${GREEN}Check completed!${NC}"
        ;;
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        usage
        ;;
esac
