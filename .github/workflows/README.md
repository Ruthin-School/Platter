# GitHub Actions Workflows

This directory contains the CI/CD workflows for the Platter project.

## Workflows Overview

### 🔄 CI Workflow (`ci.yml`)

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop` branches

**Jobs:**
1. **Test Suite** - Runs on Ubuntu with stable and beta Rust
   - Code formatting check (`cargo fmt`)
   - Linting with Clippy (`cargo clippy`)
   - Build verification
   - Test execution with all feature combinations
   - Documentation build

2. **Security Audit** - Checks for known vulnerabilities
   - Runs `cargo audit` on all dependencies

3. **Code Coverage** - Measures test coverage (main branch only)
   - Generates coverage report with `cargo-tarpaulin`
   - Uploads to Codecov

4. **Build Check** - Verifies compilation
   - Tests with all features and no features

5. **Accessibility Check** - Validates accessibility standards
   - Runs the `a11y-check` binary

**Status Badge:**
```markdown
[![CI](https://github.com/Ruthin-School/Platter/workflows/CI/badge.svg)](https://github.com/Ruthin-School/Platter/actions/workflows/ci.yml)
```

### 🚀 Release Workflow (`release.yml`)

**Triggers:**
- Push of version tags matching `v*.*.*` (e.g., `v0.5.0`)

**Jobs:**
1. **Create Release** - Creates GitHub release
   - Extracts version from tag
   - Extracts release notes from CHANGELOG.md
   - Creates GitHub release

2. **Build Binaries** - Builds for multiple platforms
   - Linux (x86_64-unknown-linux-gnu)
   - macOS (x86_64-apple-darwin)
   - Windows (x86_64-pc-windows-msvc)

3. **Publish Crate** - Publishes to crates.io (optional)
   - Requires `CARGO_REGISTRY_TOKEN` secret

4. **Verify Release** - Validates release
   - Checks tag version matches Cargo.toml

**Creating a Release:**
```bash
# Ensure version in Cargo.toml is updated
# Ensure CHANGELOG.md has entry for the version
git tag v0.5.0
git push origin v0.5.0
```

**Status Badge:**
```markdown
[![Release](https://github.com/Ruthin-School/Platter/workflows/Release/badge.svg)](https://github.com/Ruthin-School/Platter/actions/workflows/release.yml)
```

### 🔒 Security Workflow (`security.yml`)

**Triggers:**
- Weekly schedule (Mondays at 00:00 UTC)
- Push to `main` branch
- Pull requests to `main` branch
- Manual workflow dispatch

**Jobs:**
1. **Cargo Audit** - Checks for security vulnerabilities
   - Runs `cargo audit`
   - Creates GitHub issue if vulnerabilities found (scheduled runs only)
   - Uploads audit report as artifact

2. **Dependency Review** - Reviews dependency changes (PRs only)
   - Checks for security issues in new dependencies
   - Fails on moderate or higher severity issues

3. **Cargo Deny** - Additional security checks
   - License compliance
   - Banned dependencies
   - Advisory database
   - Source validation

4. **Outdated Dependencies** - Checks for updates (scheduled runs only)
   - Identifies outdated dependencies
   - Creates GitHub issue with report

**Status Badge:**
```markdown
[![Security](https://github.com/Ruthin-School/Platter/workflows/Security%20Audit/badge.svg)](https://github.com/Ruthin-School/Platter/actions/workflows/security.yml)
```

## Required Secrets

### Optional Secrets

- `CODECOV_TOKEN` - For uploading coverage reports to Codecov
- `CARGO_REGISTRY_TOKEN` - For publishing releases to crates.io

To add secrets:
1. Go to repository Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Add the name and value

## Local Testing

Before pushing, verify your changes locally:

```bash
# Check formatting
cargo fmt -- --check

# Run Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features

# Check for security issues
cargo install cargo-audit
cargo audit

# Build release
cargo build --release --all-features
```

## Workflow Maintenance

### Updating Rust Version

To test with a specific Rust version, modify the `rust` matrix in `ci.yml`:

```yaml
matrix:
  rust: [stable, beta, nightly]  # Add or remove versions
```

### Adjusting Coverage Threshold

Modify the `cargo tarpaulin` command in `ci.yml`:

```bash
cargo tarpaulin --all-features --fail-under 80  # Adjust percentage
```

### Modifying Release Targets

To add or remove build targets, add/remove jobs in `release.yml` or modify existing jobs to use different runners and targets.

## Troubleshooting

### CI Failures

1. **Formatting Issues:**
   - Run `cargo fmt` locally and commit changes

2. **Clippy Warnings:**
   - Fix warnings or add `#[allow(clippy::warning_name)]` if intentional

3. **Test Failures:**
   - Run tests locally with `cargo test --all-features`
   - Check for environment-specific issues

4. **Build Failures:**
   - Verify Cargo.toml dependencies are correct
   - Check for missing feature flags

### Release Issues

1. **Tag/Version Mismatch:**
   - Ensure Cargo.toml version matches tag version

2. **Missing Changelog Entry:**
   - Add entry to CHANGELOG.md before tagging

3. **Failed Binary Upload:**
   - Check build logs for compilation errors
   - Verify target platform is supported

## Best Practices

1. **Before Committing:**
   - Run local tests and checks
   - Update CHANGELOG.md for significant changes
   - Ensure code is formatted

2. **For Pull Requests:**
   - Ensure all CI checks pass
   - Address any security warnings
   - Update documentation if needed

3. **For Releases:**
   - Update version in Cargo.toml
   - Update CHANGELOG.md with release notes
   - Create annotated tag: `git tag -a v0.5.0 -m "Release v0.5.0"`
   - Push tag: `git push origin v0.5.0`

## Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI Best Practices](https://github.com/actions-rs)
- [Cargo Audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
- [Codecov Documentation](https://docs.codecov.io/)
