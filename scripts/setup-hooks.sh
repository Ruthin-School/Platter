#!/bin/sh
#
# Git Hooks Setup Script
# 
# This script installs Git hooks for the Platter project.
# It copies hooks from .github/hooks/ to .git/hooks/ and configures
# the commit message template.
#
# Usage: ./scripts/setup-hooks.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "${RED}❌ Error: Not in a git repository root${NC}"
    echo "Please run this script from the project root directory."
    exit 1
fi

# Check if .github/hooks directory exists
if [ ! -d ".github/hooks" ]; then
    echo "${RED}❌ Error: .github/hooks directory not found${NC}"
    exit 1
fi

echo "${BLUE}════════════════════════════════════════════════════════${NC}"
echo "${BLUE}  Git Hooks Setup for Platter${NC}"
echo "${BLUE}════════════════════════════════════════════════════════${NC}"
echo ""

# Create .git/hooks directory if it doesn't exist
if [ ! -d ".git/hooks" ]; then
    mkdir -p .git/hooks
    echo "${GREEN}✓${NC} Created .git/hooks directory"
fi

# Copy and install hooks
HOOKS_INSTALLED=0
HOOKS_FAILED=0

for hook_file in .github/hooks/*; do
    if [ -f "$hook_file" ]; then
        hook_name=$(basename "$hook_file")
        dest_path=".git/hooks/$hook_name"
        
        # Copy the hook
        if cp "$hook_file" "$dest_path"; then
            # Make it executable
            chmod +x "$dest_path"
            echo "${GREEN}✓${NC} Installed: $hook_name"
            HOOKS_INSTALLED=$((HOOKS_INSTALLED + 1))
        else
            echo "${RED}✗${NC} Failed to install: $hook_name"
            HOOKS_FAILED=$((HOOKS_FAILED + 1))
        fi
    fi
done

echo ""

# Configure commit message template
if [ -f ".gitmessage" ]; then
    if git config commit.template .gitmessage; then
        echo "${GREEN}✓${NC} Configured commit message template (.gitmessage)"
    else
        echo "${YELLOW}⚠${NC}  Warning: Failed to configure commit message template"
    fi
else
    echo "${YELLOW}⚠${NC}  Warning: .gitmessage file not found"
fi

echo ""
echo "${BLUE}────────────────────────────────────────────────────────${NC}"

# Summary
if [ $HOOKS_FAILED -eq 0 ]; then
    echo "${GREEN}✓ Successfully installed $HOOKS_INSTALLED hook(s)${NC}"
else
    echo "${YELLOW}⚠ Installed $HOOKS_INSTALLED hook(s), failed $HOOKS_FAILED${NC}"
fi

echo ""
echo "${BLUE}Next Steps:${NC}"
echo ""
echo "1. Install pre-commit framework (if not already installed):"
echo "   ${YELLOW}pip install pre-commit${NC}"
echo ""
echo "2. Install pre-commit hooks:"
echo "   ${YELLOW}pre-commit install${NC}"
echo "   ${YELLOW}pre-commit install --hook-type commit-msg${NC}"
echo ""
echo "3. (Optional) Run hooks on all files to test:"
echo "   ${YELLOW}pre-commit run --all-files${NC}"
echo ""
echo "${BLUE}Usage:${NC}"
echo "• Hooks will run automatically on 'git commit'"
echo "• To bypass hooks (not recommended):"
echo "  ${YELLOW}git commit --no-verify${NC}"
echo "• To manually run hooks:"
echo "  ${YELLOW}pre-commit run${NC}"
echo ""
echo "${BLUE}Commit Message Format:${NC}"
echo "• Use: ${GREEN}type(scope): description${NC}"
echo "• Example: ${GREEN}feat(api): add user authentication${NC}"
echo "• See .gitmessage for full guide"
echo ""
echo "${BLUE}════════════════════════════════════════════════════════${NC}"
echo "${GREEN}Setup complete!${NC}"
echo "${BLUE}════════════════════════════════════════════════════════${NC}"

exit 0