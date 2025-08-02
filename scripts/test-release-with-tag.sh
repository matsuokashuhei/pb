#!/bin/bash

# GitHub Actions Release Test - Creates a test tag to trigger the release workflow
# This script safely creates and manages test releases

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default test tag
DEFAULT_TAG="v0.0.1-test-$(date +%s)"
TEST_TAG="${1:-$DEFAULT_TAG}"

echo -e "${BLUE}=== GitHub Actions Release Workflow Test ===${NC}"
echo -e "${YELLOW}This script tests the release workflow by creating a real git tag${NC}"
echo -e "${YELLOW}Test tag: $TEST_TAG${NC}"
echo ""

# Safety checks
echo -e "${BLUE}Performing safety checks...${NC}"

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}✗ Not in a Git repository${NC}"
    exit 1
fi

# Check if we have uncommitted changes
if git status --porcelain | grep -q "^"; then
    echo -e "${RED}✗ Working directory has uncommitted changes${NC}"
    echo -e "${YELLOW}Please commit or stash changes before creating a test release${NC}"
    exit 1
fi

# Check if we have a remote
if ! git remote get-url origin > /dev/null 2>&1; then
    echo -e "${RED}✗ No origin remote found${NC}"
    exit 1
fi

# Check if we can push to remote
if ! git push --dry-run origin HEAD > /dev/null 2>&1; then
    echo -e "${RED}✗ Cannot push to remote. Check your permissions.${NC}"
    exit 1
fi

# Check if tag already exists
if git tag -l | grep -q "^$TEST_TAG$"; then
    echo -e "${RED}✗ Tag $TEST_TAG already exists${NC}"
    exit 1
fi

# Check if remote tag exists
if git ls-remote --tags origin | grep -q "refs/tags/$TEST_TAG"; then
    echo -e "${RED}✗ Tag $TEST_TAG already exists on remote${NC}"
    exit 1
fi

echo -e "${GREEN}✓ All safety checks passed${NC}"
echo ""

# Get repository info
repo_url=$(git remote get-url origin)
repo_name=$(basename "$repo_url" .git)
owner=$(basename "$(dirname "$repo_url")")
current_branch=$(git branch --show-current)

echo -e "${BLUE}Repository: $owner/$repo_name${NC}"
echo -e "${BLUE}Current branch: $current_branch${NC}"
echo -e "${BLUE}Current commit: $(git rev-parse --short HEAD)${NC}"
echo ""

# Confirmation prompt
echo -e "${YELLOW}This will:${NC}"
echo "1. Create git tag: $TEST_TAG"
echo "2. Push tag to origin, triggering the release workflow"
echo "3. GitHub Actions will build binaries for all platforms"
echo "4. A test release will be created with artifacts"
echo ""
echo -e "${YELLOW}⚠️  This creates a real release that you'll need to clean up manually${NC}"
echo ""

read -p "Do you want to continue? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Operation cancelled${NC}"
    exit 0
fi

echo ""

# Create and push the tag
echo -e "${BLUE}Creating and pushing test tag...${NC}"

git tag "$TEST_TAG"
echo -e "${GREEN}✓ Created local tag: $TEST_TAG${NC}"

git push origin "$TEST_TAG"
echo -e "${GREEN}✓ Pushed tag to origin${NC}"

echo ""
echo -e "${BLUE}=== Workflow Triggered! ===${NC}"
echo ""

# Generate URLs for monitoring
actions_url="https://github.com/$owner/$repo_name/actions"
workflow_url="$actions_url/workflows/release.yml"
releases_url="https://github.com/$owner/$repo_name/releases"

echo -e "${YELLOW}Monitor the workflow:${NC}"
echo "1. Actions page: $actions_url"
echo "2. Release workflow: $workflow_url"
echo "3. Releases page: $releases_url"
echo ""

echo -e "${YELLOW}Expected timeline:${NC}"
echo "- Workflow starts: ~30 seconds"
echo "- Builds complete: ~10-15 minutes"
echo "- Release created: ~15-20 minutes"
echo ""

echo -e "${YELLOW}What to check:${NC}"
echo "✓ All 5 build jobs start (Linux x86_64, Linux ARM64, macOS Intel, macOS ARM64, Windows)"
echo "✓ Cross-compilation works for Linux ARM64"
echo "✓ All jobs complete successfully"
echo "✓ Release is created automatically"
echo "✓ All 5 binary artifacts are uploaded"
echo "✓ Release notes are properly formatted"
echo ""

echo -e "${BLUE}Expected artifacts:${NC}"
echo "- pmon-linux-x86_64"
echo "- pmon-linux-aarch64"
echo "- pmon-macos-x86_64"
echo "- pmon-macos-aarch64" 
echo "- pmon-windows-x86_64.exe"
echo ""

# Cleanup instructions
echo -e "${BLUE}=== Cleanup Instructions ===${NC}"
echo ""
echo -e "${YELLOW}After testing is complete, clean up with:${NC}"
echo ""
echo "# Delete the test release from GitHub:"
echo "1. Go to: $releases_url"
echo "2. Find release: $TEST_TAG"
echo "3. Click 'Delete' and confirm"
echo ""
echo "# Delete the local and remote tag:"
echo "git tag -d $TEST_TAG"
echo "git push origin --delete $TEST_TAG"
echo ""

# Auto-cleanup script
cat > "/tmp/cleanup-$TEST_TAG.sh" << EOF
#!/bin/bash
# Auto-generated cleanup script for $TEST_TAG

echo "Cleaning up test release: $TEST_TAG"

# Delete local tag
if git tag -l | grep -q "^$TEST_TAG$"; then
    git tag -d "$TEST_TAG"
    echo "✓ Deleted local tag"
else
    echo "! Local tag not found"
fi

# Delete remote tag
if git ls-remote --tags origin | grep -q "refs/tags/$TEST_TAG"; then
    git push origin --delete "$TEST_TAG"
    echo "✓ Deleted remote tag"
else
    echo "! Remote tag not found"
fi

echo ""
echo "⚠️  Remember to manually delete the GitHub release at:"
echo "$releases_url"
echo ""
echo "Cleanup script completed!"
rm "\$0"  # Self-delete
EOF

chmod +x "/tmp/cleanup-$TEST_TAG.sh"

echo -e "${GREEN}Auto-cleanup script created: /tmp/cleanup-$TEST_TAG.sh${NC}"
echo -e "${YELLOW}Run it after testing to clean up the git tags${NC}"
echo ""

echo -e "${GREEN}✅ Test release workflow initiated!${NC}"
echo -e "${BLUE}Check $actions_url to monitor progress${NC}"

# Wait a moment and check if workflow started
echo ""
echo -e "${YELLOW}Waiting 30 seconds to check if workflow started...${NC}"
sleep 30

# Try to check if workflow started (basic check)
echo -e "${BLUE}You can now check the GitHub Actions page to see the workflow running.${NC}"
echo -e "${BLUE}The Release workflow should appear at the top of the actions list.${NC}"