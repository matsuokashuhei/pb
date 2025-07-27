#!/bin/bash

# Manual Release Test Script
# This script helps test the manual workflow dispatch functionality

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Manual Release Workflow Test Guide ===${NC}"
echo -e "${YELLOW}This script guides you through testing the manual workflow dispatch${NC}"
echo ""

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}✗ Not in a Git repository${NC}"
    exit 1
fi

# Check if we have a remote
if ! git remote get-url origin > /dev/null 2>&1; then
    echo -e "${RED}✗ No origin remote found${NC}"
    exit 1
fi

# Get repository information
repo_url=$(git remote get-url origin)
repo_name=$(basename "$repo_url" .git)
owner=$(dirname "$repo_url" | xargs basename)

echo -e "${BLUE}Repository: $owner/$repo_name${NC}"
echo -e "${BLUE}Current branch: $(git branch --show-current)${NC}"
echo ""

# Check workflow file exists
if [ ! -f ".github/workflows/release.yml" ]; then
    echo -e "${RED}✗ Release workflow file not found${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Release workflow file found${NC}"

# Validate workflow has manual dispatch trigger
if ! grep -q "workflow_dispatch:" ".github/workflows/release.yml"; then
    echo -e "${RED}✗ Release workflow doesn't have manual dispatch trigger${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Release workflow has manual dispatch trigger${NC}"

# Check if workflow has tag input
if ! grep -q "inputs:" ".github/workflows/release.yml"; then
    echo -e "${YELLOW}! Release workflow may not have tag input parameter${NC}"
else
    echo -e "${GREEN}✓ Release workflow has input parameters${NC}"
fi

echo ""
echo -e "${BLUE}=== Manual Testing Instructions ===${NC}"
echo ""

echo -e "${YELLOW}Step 1: Access GitHub Actions${NC}"
echo "1. Go to: https://github.com/$owner/$repo_name/actions"
echo "2. Click on the 'Release' workflow in the left sidebar"
echo "3. You should see a 'Run workflow' button on the right side"
echo ""

echo -e "${YELLOW}Step 2: Prepare Test Tag${NC}"
test_tag="v0.0.1-manual-$(date +%s)"
echo "Use this test tag: ${BLUE}$test_tag${NC}"
echo "This ensures uniqueness and won't conflict with existing releases"
echo ""

echo -e "${YELLOW}Step 3: Run Manual Workflow${NC}"
echo "1. Click 'Run workflow' button"
echo "2. Select the branch: ${BLUE}$(git branch --show-current)${NC}"
echo "3. Enter the tag: ${BLUE}$test_tag${NC}"
echo "4. Click 'Run workflow' to start"
echo ""

echo -e "${YELLOW}Step 4: Monitor Execution${NC}"
echo "1. The workflow should appear in the actions list immediately"
echo "2. Click on the workflow run to see detailed progress"
echo "3. Monitor all 5 build jobs (one for each platform):"
echo "   - Ubuntu (Linux x86_64)"
echo "   - Ubuntu (Linux ARM64)" 
echo "   - macOS (Intel x86_64)"
echo "   - macOS (Apple Silicon)"
echo "   - Windows (x86_64)"
echo ""

echo -e "${YELLOW}Step 5: Verify Results${NC}"
echo "Expected outcomes:"
echo "✓ All 5 build jobs complete successfully"
echo "✓ Release created at: https://github.com/$owner/$repo_name/releases/tag/$test_tag"
echo "✓ 5 binary artifacts uploaded:"
echo "   - pb-linux-x86_64"
echo "   - pb-linux-aarch64"
echo "   - pb-macos-x86_64"
echo "   - pb-macos-aarch64"
echo "   - pb-windows-x86_64.exe"
echo "✓ Release has proper title and description"
echo "✓ Installation instructions are included"
echo ""

echo -e "${YELLOW}Step 6: Test Binary Downloads${NC}"
echo "1. Download binaries for your platform from the release"
echo "2. Test basic functionality:"
echo ""
echo "Linux/macOS:"
echo "  chmod +x pb-*"
echo "  ./pb-* --help"
echo "  ./pb-* --version"
echo ""
echo "Windows:"
echo "  pb-windows-x86_64.exe --help"
echo "  pb-windows-x86_64.exe --version"
echo ""

echo -e "${YELLOW}Step 7: Cleanup${NC}"
echo "After successful testing:"
echo "1. Go to: https://github.com/$owner/$repo_name/releases"
echo "2. Find the test release: $test_tag"
echo "3. Click 'Edit' and then 'Delete this release'"
echo "4. Confirm deletion"
echo ""
echo "Note: This will remove both the release and the associated git tag"
echo ""

echo -e "${BLUE}=== Troubleshooting Common Issues ===${NC}"
echo ""

echo -e "${YELLOW}Issue: 'Run workflow' button not visible${NC}"
echo "Solution: Ensure you have write access to the repository"
echo ""

echo -e "${YELLOW}Issue: Build fails on certain platforms${NC}"
echo "Solutions:"
echo "- Check GitHub Actions logs for specific error messages"
echo "- Verify all build dependencies are available"
echo "- macOS/Windows builds require native runners"
echo ""

echo -e "${YELLOW}Issue: Release not created${NC}"
echo "Solutions:"
echo "- Check workflow permissions (needs contents: write)"
echo "- Verify all build jobs completed successfully"
echo "- Check for conflicting tag names"
echo ""

echo -e "${YELLOW}Issue: Missing artifacts${NC}"
echo "Solutions:"
echo "- Verify all build matrix jobs completed"
echo "- Check upload step logs for errors"
echo "- Ensure binary paths are correct"
echo ""

echo -e "${BLUE}=== Success Criteria Checklist ===${NC}"
echo ""
echo "□ Manual workflow dispatch triggers successfully"
echo "□ All 5 platform builds complete without errors"
echo "□ GitHub release is created automatically"
echo "□ All binary artifacts are uploaded correctly"
echo "□ Release notes include platform-specific instructions"
echo "□ Downloaded binaries execute and show help/version"
echo "□ Tag extraction works correctly for manual dispatch"
echo ""

echo -e "${GREEN}=== Ready to Test! ===${NC}"
echo -e "${BLUE}Open https://github.com/$owner/$repo_name/actions and look for the Release workflow${NC}"
echo -e "${BLUE}Use test tag: $test_tag${NC}"
echo ""
echo -e "${YELLOW}Remember to clean up the test release after successful testing!${NC}"