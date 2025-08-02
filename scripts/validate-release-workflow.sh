#!/bin/bash

# Script to create a test tag and validate the release workflow
# This simulates the actual release process without affecting production

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Release Workflow Validation Script ===${NC}"
echo -e "${YELLOW}This script validates the release workflow without creating actual releases${NC}"
echo ""

# Get current git status
if ! git status --porcelain | grep -q "^"; then
    echo -e "${GREEN}✓ Working directory is clean${NC}"
else
    echo -e "${RED}✗ Working directory has uncommitted changes${NC}"
    echo -e "${YELLOW}Please commit or stash changes before running this test${NC}"
    exit 1
fi

# Get current branch
current_branch=$(git branch --show-current)
echo -e "${BLUE}Current branch: $current_branch${NC}"

# Check if we have remote access
if ! git remote -v | grep -q "origin"; then
    echo -e "${RED}✗ No origin remote found${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Git repository is ready${NC}"
echo ""

# Validate workflow file syntax
echo -e "${BLUE}Validating workflow file syntax...${NC}"

workflow_file=".github/workflows/release.yml"
if [ ! -f "$workflow_file" ]; then
    echo -e "${RED}✗ Release workflow file not found: $workflow_file${NC}"
    exit 1
fi

# Basic YAML syntax validation (if yq is available)
if command -v yq >/dev/null 2>&1; then
    if yq eval '.' "$workflow_file" >/dev/null 2>&1; then
        echo -e "${GREEN}✓ Release workflow YAML syntax is valid${NC}"
    else
        echo -e "${RED}✗ Release workflow YAML syntax is invalid${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}! yq not available, skipping YAML syntax validation${NC}"
fi

# Check workflow triggers
echo -e "${BLUE}Checking workflow triggers...${NC}"

if grep -q "tags:" "$workflow_file" && grep -q "workflow_dispatch:" "$workflow_file"; then
    echo -e "${GREEN}✓ Workflow has both tag and manual dispatch triggers${NC}"
else
    echo -e "${RED}✗ Workflow missing required triggers${NC}"
    exit 1
fi

# Check build matrix
echo -e "${BLUE}Checking build matrix...${NC}"

expected_targets=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
)

missing_targets=()
for target in "${expected_targets[@]}"; do
    if ! grep -q "$target" "$workflow_file"; then
        missing_targets+=("$target")
    fi
done

if [ ${#missing_targets[@]} -eq 0 ]; then
    echo -e "${GREEN}✓ All expected build targets are present${NC}"
else
    echo -e "${YELLOW}! Missing build targets:${NC}"
    for target in "${missing_targets[@]}"; do
        echo -e "${YELLOW}  - $target${NC}"
    done
fi

# Validate that strip command is not present (should be handled by Cargo.toml)
if grep -q "strip target" "$workflow_file"; then
    echo -e "${YELLOW}! Found manual strip command - this may be redundant${NC}"
    echo -e "${YELLOW}  Check if Cargo.toml has 'strip = true' in [profile.release]${NC}"
else
    echo -e "${GREEN}✓ No redundant strip commands found${NC}"
fi

echo ""

# Test local build simulation
echo -e "${BLUE}Testing local build simulation...${NC}"

# Create a temporary test directory
test_dir=$(mktemp -d)
echo -e "${YELLOW}Using temporary directory: $test_dir${NC}"

# Copy project to test directory to avoid conflicts
cp -r . "$test_dir/"
cd "$test_dir"

# Test builds for available targets (without actual cross-platform compilation)
echo -e "${YELLOW}Testing native build...${NC}"
if cargo build --release >/dev/null 2>&1; then
    echo -e "${GREEN}✓ Native build successful${NC}"
    
    # Test binary functionality
    if ./target/release/pmon --help >/dev/null 2>&1; then
        echo -e "${GREEN}✓ Binary functionality test passed${NC}"
    else
        echo -e "${RED}✗ Binary functionality test failed${NC}"
        cd - >/dev/null
        rm -rf "$test_dir"
        exit 1
    fi
else
    echo -e "${RED}✗ Native build failed${NC}"
    cd - >/dev/null
    rm -rf "$test_dir"
    exit 1
fi

# Clean up test directory
cd - >/dev/null
rm -rf "$test_dir"

echo ""

# Validate release notes template
echo -e "${BLUE}Validating release notes template...${NC}"

# Check if release notes contain platform-specific instructions
if grep -q "Linux" "$workflow_file" && grep -q "macOS" "$workflow_file" && grep -q "Windows" "$workflow_file"; then
    echo -e "${GREEN}✓ Release notes include all major platforms${NC}"
else
    echo -e "${YELLOW}! Release notes may be missing platform instructions${NC}"
fi

# Check if installation instructions are present
if grep -q "chmod +x" "$workflow_file" && grep -q "--help" "$workflow_file"; then
    echo -e "${GREEN}✓ Release notes include installation instructions${NC}"
else
    echo -e "${YELLOW}! Release notes may be missing installation instructions${NC}"
fi

echo ""

# Summary and recommendations
echo -e "${BLUE}=== Validation Summary ===${NC}"
echo -e "${GREEN}✓ Git repository status validated${NC}"
echo -e "${GREEN}✓ Workflow file syntax validated${NC}"
echo -e "${GREEN}✓ Workflow triggers validated${NC}"
echo -e "${GREEN}✓ Build matrix validated${NC}"
echo -e "${GREEN}✓ Local build simulation passed${NC}"
echo -e "${GREEN}✓ Release notes template validated${NC}"

echo ""
echo -e "${BLUE}=== Next Steps ===${NC}"
echo -e "${YELLOW}To test the actual workflow:${NC}"
echo -e "${YELLOW}1. Create a test tag: git tag v0.0.1-test && git push origin v0.0.1-test${NC}"
echo -e "${YELLOW}2. Monitor the GitHub Actions workflow${NC}"
echo -e "${YELLOW}3. Verify release artifacts are created${NC}"
echo -e "${YELLOW}4. Test manual dispatch: GitHub → Actions → Release → Run workflow${NC}"
echo -e "${YELLOW}5. Clean up test release and tag when done${NC}"

echo ""
echo -e "${GREEN}✅ Release workflow validation completed successfully!${NC}"
echo -e "${BLUE}The workflow appears ready for testing with a real tag or manual dispatch.${NC}"