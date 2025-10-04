#!/bin/bash
# Script to create annotated Git tags for historical releases
# Usage: ./scripts/create-release-tags.sh [--push]

set -e

PUSH_TAGS=false

# Parse command line arguments
if [[ "$1" == "--push" ]]; then
    PUSH_TAGS=true
fi

# Color codes for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "========================================="
echo "Git Release Tag Creation Script"
echo "========================================="
echo ""

# Function to check if a tag already exists
tag_exists() {
    git tag -l "$1" | grep -q "^$1$"
}

# Function to create an annotated tag
create_tag() {
    local tag_name=$1
    local tag_date=$2
    local tag_message=$3
    
    if tag_exists "$tag_name"; then
        echo -e "${YELLOW}Tag $tag_name already exists. Skipping...${NC}"
        return 0
    fi
    
    echo -e "${GREEN}Creating tag: $tag_name${NC}"
    
    # Create annotated tag with message
    GIT_COMMITTER_DATE="$tag_date" git tag -a "$tag_name" -m "$tag_message"
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Successfully created tag $tag_name${NC}"
    else
        echo -e "${RED}✗ Failed to create tag $tag_name${NC}"
        return 1
    fi
}

# Create tags for each release
echo "Creating tags for historical releases..."
echo ""

# v0.1.0 - 2025-06-01
create_tag "v0.1.0" "2025-06-01 12:00:00" \
"Release v0.1.0 - 2025-06-01

Initial release of Platter

Added:
- Initial release of Platter
- Basic menu management
- User authentication
- Web-based administration interface
- File-based data persistence"

# v0.2.0 - 2025-07-10
create_tag "v0.2.0" "2025-07-10 12:00:00" \
"Release v0.2.0 - 2025-07-10

Major security and feature enhancements

Added:
- Admin authentication system with Argon2 password hashing
- Menu item CRUD operations
- Notice system with priority levels
- JSON file-based storage
- Responsive web interface

Changed:
- Implemented secure session management
- Added CSRF protection
- Improved input validation
- Enhanced security headers"

# v0.3.0 - 2025-08-15
create_tag "v0.3.0" "2025-08-15 12:00:00" \
"Release v0.3.0 - 2025-08-15

Enhanced functionality and performance improvements

Added:
- Menu preset functionality
- Menu scheduling system
- Admin interface improvements
- Data validation and sanitization
- User session management

Changed:
- Improved error handling and reporting
- Enhanced template system with Tera
- Updated to Rust 2024 edition
- Refactored storage system for better performance"

# v0.4.0 - 2025-09-28
create_tag "v0.4.0" "2025-09-28 12:00:00" \
"Release v0.4.0 - 2025-09-28

Comprehensive documentation and security improvements

Added:
- Comprehensive API documentation in API.md
- Detailed design documentation in DESIGN.md
- Complete development guide in DEVELOPMENT.md
- Production deployment guide in DEPLOYMENT.md
- Security best practices in SECURITY.md
- Enhanced README with detailed configuration instructions
- Basic test module in src/main.rs

Changed:
- Updated application from version 0.3.0 to 0.4.0
- Replaced #[actix_web::main] with #[tokio::main] for async runtime
- Improved session security with environment variable support for SESSION_SECRET
- Updated deprecated std::io::Error::new() to std::io::Error::other()
- Simplified error mapping syntax in handlers
- Enhanced documentation with comprehensive tables of contents

Security:
- Added security-focused session management with environment-based secrets
- Improved CORS configuration for production environments
- Enhanced security documentation with best practices checklist
- Updated error handling to prevent information disclosure"

echo ""
echo "========================================="
echo "Tag Creation Summary"
echo "========================================="
git tag -l "v*" --sort=version:refname

echo ""
echo "Total tags: $(git tag -l "v*" | wc -l)"
echo ""

if [ "$PUSH_TAGS" = true ]; then
    echo -e "${YELLOW}Pushing tags to remote repository...${NC}"
    git push origin --tags
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Successfully pushed all tags to remote${NC}"
    else
        echo -e "${RED}✗ Failed to push tags to remote${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}Tags created locally but not pushed to remote.${NC}"
    echo "To push tags to remote, run:"
    echo "  git push origin --tags"
    echo ""
    echo "Or run this script again with the --push flag:"
    echo "  ./scripts/create-release-tags.sh --push"
fi

echo ""
echo -e "${GREEN}Done!${NC}"