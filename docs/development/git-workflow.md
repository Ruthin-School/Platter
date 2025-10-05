# 🔀 Git Workflow Guide

> **Navigation:** [Documentation Home](../README.md) → [Development](README.md) → Git Workflow

**Reading Time:** Approximately 15 minutes
**Complexity:** Intermediate
**Prerequisites:** Basic Git knowledge and understanding of version control

## What This Guide Covers

This guide describes the Git workflow and best practices for the Platter project. You will learn about:

1. Semantic versioning standards
2. Release tagging procedures
3. Creating and managing releases
4. Tag naming conventions
5. Best practices for Git workflows
6. Automated tagging processes

---

## Table of Contents

- [Section 1: Semantic Versioning](#section-1-semantic-versioning)
- [Section 2: Release Tagging](#section-2-release-tagging)
- [Section 3: Creating a New Release](#section-3-creating-a-new-release)
- [Section 4: Tag Naming Conventions](#section-4-tag-naming-conventions)
- [Section 5: Best Practices](#section-5-best-practices)
- [Section 6: Automated Tagging](#section-6-automated-tagging)

---

## Section 1: Semantic Versioning

Platter follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) for version numbering.

### Version Format

Version numbers follow the format: `MAJOR.MINOR.PATCH`

- **MAJOR**: Incremented for incompatible API changes
- **MINOR**: Incremented for backwards-compatible functionality additions
- **PATCH**: Incremented for backwards-compatible bug fixes

### Pre-release Versions

Pre-release versions may be denoted by appending a hyphen and additional identifiers:

- `1.0.0-alpha`
- `1.0.0-beta.1`
- `1.0.0-rc.1`

### Version Precedence

Examples in order of precedence:

```
0.1.0 < 0.2.0 < 0.3.0 < 1.0.0-alpha < 1.0.0-beta < 1.0.0-rc.1 < 1.0.0
```

## Section 2: Release Tagging

Git tags are used to mark specific points in the repository's history as important, typically for releases.

### Why Annotated Tags?

Platter uses **annotated tags** (not lightweight tags) for releases because they:

- Store the tagger's name, email, and date
- Include a tagging message with release notes
- Are required for creating GitHub releases
- Can be GPG-signed for verification
- Are treated as full objects in Git's database

### Tag Format

All release tags follow the format: `v{MAJOR}.{MINOR}.{PATCH}`

Examples:
- `v0.1.0`
- `v0.2.0`
- `v1.0.0`
- `v1.2.3`

The `v` prefix is mandatory for all release tags.

## Section 3: Creating a New Release

Follow these steps to create a new release:

### Step 3.1: Update Version Number

Update the version in [`Cargo.toml`](../../Cargo.toml:1):

```toml
[package]
name = "platter"
version = "0.5.0"  # Update this
```

### Step 3.2: Update CHANGELOG.md

Add a new section to [`CHANGELOG.md`](../../CHANGELOG.md:1) following the [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
## [0.5.0] - 2025-10-15

### Added
- New feature description

### Changed
- Changed functionality description

### Fixed
- Bug fix description

### Security
- Security improvement description
```

### Step 3.3: Commit Changes

Commit the version bump and changelog updates:

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.5.0"
```

### Step 3.4: Create Annotated Tag

Create an annotated tag with a detailed message:

```bash
git tag -a v0.5.0 -m "Release v0.5.0 - 2025-10-15

Brief description of the release

Added:
- Feature 1
- Feature 2

Changed:
- Change 1
- Change 2

Fixed:
- Bug fix 1"
```

### Step 3.5: Verify the Tag

Verify that the tag was created correctly:

```bash
# List all tags
git tag -l

# Show tag details
git show v0.5.0
```

### Step 3.6: Push to Remote

Push both the commit and the tag to the remote repository:

```bash
# Push the commit
git push origin main

# Push the tag
git push origin v0.5.0

# Or push all tags at once
git push origin --tags
```

## Section 4: Tag Naming Conventions

### Release Tags

- **Format**: `v{MAJOR}.{MINOR}.{PATCH}`
- **Examples**: `v0.1.0`, `v1.0.0`, `v2.3.4`
- **Use**: Production releases

### Pre-release Tags

- **Format**: `v{MAJOR}.{MINOR}.{PATCH}-{identifier}`
- **Examples**: `v1.0.0-alpha`, `v1.0.0-beta.1`, `v1.0.0-rc.1`
- **Use**: Alpha, beta, and release candidate versions

### Build Metadata (Optional)

- **Format**: `v{MAJOR}.{MINOR}.{PATCH}+{metadata}`
- **Examples**: `v1.0.0+20251015`, `v1.0.0+build.123`
- **Use**: Build-specific metadata (optional, rarely used)

## Section 5: Pushing Tags to Remote

### Push a Single Tag

```bash
git push origin v0.5.0
```

### Push All Tags

```bash
git push origin --tags
```

### Push Tags and Commits Together

```bash
git push origin main --tags
```

### Verify Remote Tags

```bash
# List remote tags
git ls-remote --tags origin

# Fetch remote tags
git fetch --tags
```

## Section 6: Best Practices

### DO

✅ **Always use annotated tags for releases**
```bash
git tag -a v1.0.0 -m "Release message"
```

✅ **Include comprehensive release notes in tag messages**
- Version number and date
- Summary of changes from CHANGELOG.md
- Breaking changes (if any)
- Migration instructions (if needed)

✅ **Tag the correct commit**
- Ensure all release-related commits are included
- Tag after version bump commits

✅ **Follow semantic versioning strictly**
- Breaking changes = MAJOR bump
- New features = MINOR bump
- Bug fixes = PATCH bump

✅ **Update CHANGELOG.md before tagging**
- Document all changes
- Follow Keep a Changelog format
- Include release date

✅ **Test before tagging**
- Run all tests
- Verify build succeeds
- Check for security vulnerabilities

### DON'T

❌ **Don't use lightweight tags for releases**
```bash
# AVOID THIS
git tag v1.0.0
```

❌ **Don't tag without updating CHANGELOG.md**

❌ **Don't push tags immediately**
- Review tags locally first
- Verify tag content with `git show`

❌ **Don't delete or move published tags**
- Once pushed, tags should be immutable
- If a mistake is made, create a new patch version

❌ **Don't skip version numbers**
- Follow sequential versioning
- Don't jump from 0.1.0 to 0.3.0

❌ **Don't tag unreleased or broken code**
- Ensure the codebase is in a releasable state
- All tests should pass

## Section 7: Automated Tagging

For convenience, use the provided script to create tags for historical releases or automate the tagging process.

### Using the Tag Creation Script

The [`scripts/create-release-tags.sh`](../../scripts/create-release-tags.sh:1) script automates tag creation for historical releases:

```bash
# Create tags locally (does not push)
./scripts/create-release-tags.sh

# Create tags and push to remote
./scripts/create-release-tags.sh --push
```

### Script Features

- ✅ Checks if tags already exist (idempotent)
- ✅ Creates annotated tags with comprehensive messages
- ✅ Includes release dates and change summaries from CHANGELOG.md
- ✅ Provides colored output for better readability
- ✅ Optional flag to push tags to remote
- ✅ Shows summary of all created tags

### Manual Tag Verification

After running the script, verify the tags:

```bash
# List all tags
git tag -l "v*" --sort=version:refname

# Show detailed information for a specific tag
git show v0.4.0

# Count total tags
git tag | wc -l
```

### Pushing Historical Tags

If you ran the script without `--push`, push tags manually:

```bash
# Push all tags
git push origin --tags

# Or push specific tags
git push origin v0.1.0 v0.2.0 v0.3.0 v0.4.0
```

## GitHub Releases

Once tags are pushed to GitHub, you can create releases:

1. Go to your repository on GitHub
2. Navigate to **Releases** → **Tags**
3. Click **Create release from tag**
4. Add release notes (can copy from CHANGELOG.md)
5. Attach any binary artifacts if applicable
6. Publish the release

### Automated Releases

Consider using GitHub Actions to automatically create releases when tags are pushed:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/create-release@v1
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}
          draft: false
          prerelease: false
```

## Section 8: Troubleshooting

### 8.1: Deleting a Local Tag

If you need to delete a tag locally:

```bash
git tag -d v0.5.0
```

### 8.2: Deleting a Remote Tag

⚠️ **Use with extreme caution!** Only delete remote tags if absolutely necessary:

```bash
git push origin --delete v0.5.0
```

### 8.3: Recreating a Tag

If you need to recreate a tag (before pushing):

```bash
# Delete local tag
git tag -d v0.5.0

# Recreate the tag
git tag -a v0.5.0 -m "New message"
```

### 8.4: Listing Tags by Date

```bash
git tag --sort=-creatordate
```

### 8.5: Finding Tags Containing a Commit

```bash
git tag --contains <commit-hash>
```

## External References

- [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) – Versioning specification
- [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) – Changelog format standard
- [Git Tagging Documentation](https://git-scm.com/book/en/v2/Git-Basics-Tagging) – Official Git documentation
- [GitHub Releases Documentation](https://docs.github.com/en/repositories/releasing-projects-on-github) – GitHub release guide

## Related Documentation

Access these resources for additional information:

- **[Branch Protection Rules](branch-protection.md)** – GitHub branch protection configuration
- **[Merge Strategies Guide](merge-strategies.md)** – Detailed merge strategy documentation
- **[Git Hooks Guide](git-hooks.md)** – Pre-commit and commit-msg hooks
- **[Contributing Guidelines](contributing.md)** – How to contribute to Platter
- **[Development Setup](setup.md)** – Setting up your development environment
- **[CHANGELOG.md](../../CHANGELOG.md)** – Project changelog

---

[← Back to Development](README.md) | [Documentation Home](../README.md)