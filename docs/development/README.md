# 💻 Development Guide

> **Navigation:** [Documentation Home](../README.md) → Development

Resources for developers contributing to the Platter project.

---

## 📚 Development Documentation

### [🔧 Development Setup Guide](setup.md)

**What this covers:** Set up your complete development environment

**Topics included:**
- Prerequisites and required tooling
- IDE (Integrated Development Environment) configuration recommendations
- Building and running the application locally
- Development workflow and best practices
- Troubleshooting common setup issues

**Reading time:** Approximately 20 minutes

---

### [🤝 Contributing Guidelines](contributing.md)

**What this covers:** How to contribute effectively to Platter

**Topics included:**
- Code style and naming conventions
- Commit message format standards
- Pull request submission process
- Documentation standards and requirements
- Code review expectations

**Reading time:** Approximately 20 minutes

---

### [🧪 Testing Guide](testing.md)

**What this covers:** Testing practices and procedures

**Topics included:**
- How to run different types of tests
- Writing effective unit tests
- Integration testing strategies
- Test coverage measurement and goals
- Best practices for test organisation

**Reading time:** Approximately 18 minutes

---

### [🔀 Git Workflow Guide](git-workflow.md)

**What this covers:** Git workflow, versioning, and release tagging

**Topics included:**
- Semantic versioning standards
- Creating and managing release tags
- Branch naming conventions
- Tag format and best practices
- Automated tagging scripts

**Reading time:** Approximately 15 minutes

---

### [🛡️ Branch Protection Rules](branch-protection.md)

**What this covers:** GitHub branch protection configuration and enforcement

**Topics included:**
- Why branch protection is critical
- Required status checks and their purposes
- GPG commit signing setup and requirements
- Merge strategies and when to use each
- Troubleshooting branch protection issues

**Reading time:** Approximately 15 minutes

---

### [🔀 Merge Strategies Guide](merge-strategies.md)

**What this covers:** Detailed guide to Git merge strategies

**Topics included:**
- Squash and merge (recommended default)
- Rebase and merge (for clean commits)
- Interactive rebase techniques
- Conflict resolution strategies
- Best practices for each strategy

**Reading time:** Approximately 20 minutes

---

### [🪝 Git Hooks Guide](git-hooks.md)

**What this covers:** Pre-commit and commit message validation hooks

**Topics included:**
- Automated code quality checks
- Commit message format validation
- Setting up Git hooks locally
- Customizing hook behavior
- Troubleshooting hook issues

**Reading time:** Approximately 12 minutes

---

## 🛠️ Development Tools

### Recommended Tools for Development

**IDE support:**
- **rust-analyzer** – Real-time compiler feedback and code intelligence
- **Error Lens** – Inline error display for faster debugging
- **CodeLLDB** – Debugging support for Rust applications

**Build and quality tools:**
- **cargo-watch** – Automatic rebuild on file changes
- **cargo-audit** – Security vulnerability scanning
- **cargo-clippy** – Linting for code quality
- **cargo-fmt** – Automatic code formatting

---

### Useful Development Commands

Execute these commands during development:

```bash
# Run application in development mode
cargo run

# Run all tests
cargo test

# Format code automatically
cargo fmt

# Check code quality with linter
cargo clippy

# Automatic rebuild on file changes
cargo watch -x run

# Build optimised release version
cargo build --release
```

---

## 🎯 Quick Navigation Links

**Core documentation:**
- [Main README](../../README.md) – Project overview and introduction
- [Getting Started Guide](../guides/getting-started.md) – Installation and quick start
- [API Reference](../api/reference.md) – Complete API documentation
- [Design Documentation](../architecture/design.md) – System architecture overview

---

## 🔍 Common Development Tasks

Find quick links to frequently performed operations:

- [Set up development environment](setup.md#initial-setup) – Configure your workspace
- [Run tests](testing.md#running-tests) – Execute test suite
- [Build for production](setup.md#building-for-production) – Create release builds
- [Submit a pull request](contributing.md#pull-request-process) – Contribute code
- [Choose merge strategy](merge-strategies.md#section-6-merge-strategy-decision-framework) – Select the right merge approach
- [Configure branch protection](branch-protection.md#section-2-configuring-branch-protection) – Set up GitHub protection rules
- [Set up GPG signing](branch-protection.md#section-4-gpg-commit-signing) – Configure commit signing
- [Resolve merge conflicts](merge-strategies.md#section-5-conflict-resolution) – Handle Git conflicts
- [Write documentation](contributing.md#documentation-guidelines) – Document your changes

---

## 📖 Related Documentation

Access additional resources for development:

- **[Architecture Design](../architecture/design.md)** – System architecture and design decisions
- **[Security Guidelines](../architecture/security.md)** – Security best practices for developers
- **[API Reference](../api/reference.md)** – API endpoints and usage patterns

---

[← Back to Documentation Home](../README.md)