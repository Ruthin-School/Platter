# Git Hooks

This document describes the Git hooks setup for the Platter project, including what they do, how to install them, and how to use them effectively.

## Overview

Git hooks are scripts that run automatically at certain points in the Git workflow. This project uses hooks to enforce code quality standards and commit message conventions before code reaches CI/CD pipelines.

### Why Use Git Hooks?

- **Early Feedback**: Catch issues before pushing to remote
- **Consistent Quality**: Enforce standards across all contributors
- **Faster Development**: Fix issues locally instead of waiting for CI/CD
- **Better Git History**: Maintain clean, conventional commit messages

## Installed Hooks

### Pre-Commit Hooks (via pre-commit framework)

These hooks run automatically before each commit:

1. **Trailing Whitespace Removal**
   - Removes unnecessary trailing whitespace
   - Keeps markdown line breaks intact

2. **End-of-File Fixer**
   - Ensures all files end with a newline character

3. **YAML Syntax Check**
   - Validates YAML files for syntax errors
   - Catches configuration issues early

4. **Large File Prevention**
   - Prevents committing files larger than 500KB
   - Keeps repository size manageable

5. **Merge Conflict Check**
   - Detects unresolved merge conflict markers
   - Prevents accidental commits of conflicts

6. **JSON & TOML Syntax Check**
   - Validates JSON and TOML files
   - Ensures configuration files are valid

7. **Branch Protection**
   - Prevents direct commits to `main` or `master`
   - Encourages feature branch workflow

8. **Cargo Format Check** (`cargo fmt`)
   - Ensures Rust code follows standard formatting
   - Runs on all `.rs` files

9. **Cargo Check** (`cargo check`)
   - Verifies code compiles successfully
   - Checks all features and targets

10. **Cargo Clippy** (`cargo clippy`)
    - Runs Rust linter for code quality
    - Treats warnings as errors (`-D warnings`)

### Commit Message Hook

The `commit-msg` hook validates that commit messages follow the [Conventional Commits](https://www.conventionalcommits.org/) format:

```
type(scope): description

[optional body]

[optional footer(s)]
```

#### Valid Commit Types

| Type | Description | Example |
|------|-------------|---------|
| `feat` | New feature | `feat(api): add user authentication` |
| `fix` | Bug fix | `fix(scheduler): resolve memory leak` |
| `docs` | Documentation changes | `docs(readme): update installation steps` |
| `style` | Code style changes | `style: format menu display code` |
| `refactor` | Code refactoring | `refactor(api): simplify error handling` |
| `perf` | Performance improvements | `perf(db): optimize menu queries` |
| `test` | Test additions/updates | `test(auth): add login validation tests` |
| `chore` | Maintenance tasks | `chore(deps): update actix-web to 4.4.0` |
| `ci` | CI/CD changes | `ci: add automated security scanning` |
| `build` | Build system changes | `build: update cargo configuration` |
| `revert` | Revert previous commit | `revert: undo feature x changes` |

#### Commit Message Rules

1. **Type** is required and must be one of the valid types
2. **Scope** is optional and should be in parentheses (e.g., `api`, `ui`, `auth`)
3. **Description** must:
   - Start with a lowercase letter
   - Use imperative, present tense ("add" not "added")
   - Be at least 10 characters long
   - Preferably be under 72 characters
   - Not end with a period

#### Valid Commit Examples

```bash
# Simple commit with type and description
feat: add user registration feature

# Commit with scope
fix(auth): resolve session timeout issue

# Commit with body
feat(api): add pagination support for menu items

Implement pagination to improve performance when loading large menus.
Users can now specify page size and page number in API requests.

# Commit referencing an issue
fix(scheduler): prevent duplicate job execution

Closes #123

# Breaking change
feat(api)!: change authentication response format

BREAKING CHANGE: The authentication endpoint now returns a different
response structure. Clients must update to parse the new format.
```

#### Invalid Commit Examples

```bash
# Missing type
❌ update readme

# Uppercase description
❌ feat: Add new feature

# Too short
❌ fix: bug

# Wrong type
❌ update: add tests

# Missing description
❌ feat(api):
```

## Installation

### Prerequisites

1. **Python 3.7+** (for pre-commit framework)
2. **pip** (Python package manager)
3. **Rust toolchain** (cargo, rustfmt, clippy)

### Step 1: Install Pre-commit Framework

```bash
# Using pip
pip install pre-commit

# Or using pip3
pip3 install pre-commit

# Verify installation
pre-commit --version
```

### Step 2: Run Setup Script

From the project root directory, run:

```bash
./scripts/setup-hooks.sh
```

This script will:
- Copy Git hooks from `.github/hooks/` to `.git/hooks/`
- Make hooks executable
- Configure the commit message template

### Step 3: Install Pre-commit Hooks

```bash
# Install pre-commit hooks
pre-commit install

# Install commit-msg hooks
pre-commit install --hook-type commit-msg
```

### Step 4: Verify Installation

Test that hooks are working:

```bash
# Run all hooks on all files (optional)
pre-commit run --all-files
```

## Usage

### Normal Workflow

Once installed, hooks run automatically:

```bash
# Stage your changes
git add src/main.rs

# Commit (hooks run automatically)
git commit -m "feat(api): add new endpoint"
```

If any hook fails, the commit is aborted and you'll see error messages explaining what needs to be fixed.

### Manual Hook Execution

Run hooks manually without committing:

```bash
# Run all hooks on staged files
pre-commit run

# Run all hooks on all files
pre-commit run --all-files

# Run specific hook
pre-commit run cargo-check
pre-commit run clippy
```

### Using the Commit Template

When you run `git commit` without `-m`, your editor will open with the commit template (`.gitmessage`), which includes:

- Format guidelines
- Valid commit types
- Examples
- Helpful reminders

### Bypassing Hooks

**⚠️ Use sparingly!** Bypassing hooks should be rare and only in emergencies.

```bash
# Bypass all hooks
git commit --no-verify -m "emergency fix"

# Or
git commit -n -m "emergency fix"
```

Valid reasons to bypass:
- Emergency hotfix that must be deployed immediately
- Hooks are failing due to external issues (network, tools)
- You're making a WIP commit in a personal branch

Invalid reasons to bypass:
- "Hooks are annoying"
- "I'll fix it later"
- Avoiding legitimate code quality issues

## Troubleshooting

### Hook Execution Fails

**Problem**: Hooks fail with permission errors

```bash
# Make hooks executable
chmod +x .git/hooks/*
```

**Problem**: `pre-commit: command not found`

```bash
# Install pre-commit
pip install pre-commit

# Or add to PATH
export PATH="$HOME/.local/bin:$PATH"
```

### Cargo Hooks Fail

**Problem**: `cargo fmt` or `cargo clippy` not found

```bash
# Install Rust toolchain components
rustup component add rustfmt clippy
```

**Problem**: Clippy warnings treated as errors

Fix the warnings or temporarily adjust settings in `.pre-commit-config.yaml`:

```yaml
- id: clippy
  entry: cargo clippy --all-features --all-targets -- -W clippy::all
```

### Commit Message Validation Fails

**Problem**: "Invalid commit message format"

Check your commit message against the format:
- Does it start with a valid type?
- Is the description lowercase?
- Is it at least 10 characters?

Use the commit template for guidance:
```bash
git commit  # Without -m flag to see template
```

### Pre-commit Hook Hangs

**Problem**: Hook runs too long or appears frozen

```bash
# Kill the process
Ctrl+C

# Check which hook is slow
pre-commit run --verbose

# Temporarily bypass to commit
git commit --no-verify
```

If a hook consistently takes too long (>10 seconds), report it as an issue.

### Clean Up Hook State

If hooks are in a bad state:

```bash
# Uninstall hooks
pre-commit uninstall
pre-commit uninstall --hook-type commit-msg

# Reinstall
pre-commit install
pre-commit install --hook-type commit-msg
```

## Configuration Files

### `.pre-commit-config.yaml`

Main configuration for the pre-commit framework. Defines:
- Which hooks to run
- Hook parameters and arguments
- File patterns to include/exclude
- When hooks should run (commit, push, etc.)

### `.github/hooks/commit-msg`

Custom shell script that validates commit messages. Can be modified to adjust:
- Valid commit types
- Description length limits
- Scope validation rules

### `.gitmessage`

Commit message template shown in your editor. Customize to:
- Add project-specific commit types
- Include additional examples
- Update guidelines for your team

### `scripts/setup-hooks.sh`

Installation script. Modify if you need to:
- Add additional hooks
- Change installation process
- Add environment checks

## Best Practices

### Writing Commit Messages

1. **Be Descriptive**: Explain what and why, not how
   ```bash
   ✅ feat(auth): add rate limiting to login endpoint
   ❌ feat(auth): change code
   ```

2. **Use Present Tense**: Describe what the commit does
   ```bash
   ✅ fix: resolve memory leak
   ❌ fix: resolved memory leak
   ```

3. **Keep Subject Short**: Aim for under 72 characters
   ```bash
   ✅ feat(api): add pagination support
   ❌ feat(api): add pagination support for menu items with configurable page size
   ```

4. **Add Body for Complex Changes**: Explain the reasoning
   ```bash
   feat(api): add caching layer
   
   Implement Redis caching to reduce database load during peak hours.
   Cache TTL is set to 5 minutes for menu items.
   ```

### Working with Hooks

1. **Run Hooks Before Pushing**: Catch issues early
   ```bash
   pre-commit run --all-files
   ```

2. **Fix Issues Incrementally**: Don't let formatting issues pile up

3. **Keep Hooks Fast**: Report slow hooks (>10 seconds)

4. **Update Hooks Regularly**: Keep pre-commit config up to date
   ```bash
   pre-commit autoupdate
   ```

## Integration with CI/CD

The pre-commit hooks mirror checks that run in CI/CD:

| Local Hook | CI/CD Equivalent |
|------------|------------------|
| `cargo fmt` | Format check in CI |
| `cargo clippy` | Clippy check in CI |
| `cargo check` | Build verification |
| YAML validation | Configuration checks |
| Commit message | PR title validation |

Running hooks locally ensures CI/CD passes on first attempt, speeding up development.

## Maintenance

### Updating Hook Versions

```bash
# Check for updates
pre-commit autoupdate

# Review changes in .pre-commit-config.yaml
git diff .pre-commit-config.yaml

# Test with all files
pre-commit run --all-files
```

### Adding New Hooks

1. Edit `.pre-commit-config.yaml`
2. Add new hook configuration
3. Test thoroughly
4. Update this documentation
5. Notify team members to reinstall

### Removing Hooks

1. Remove from `.pre-commit-config.yaml`
2. Update documentation
3. Commit changes
4. Team members will automatically use new config

## Additional Resources

- [Pre-commit Documentation](https://pre-commit.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Git Hooks Documentation](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)

## Support

If you encounter issues with Git hooks:

1. Check this troubleshooting section
2. Run `pre-commit run --verbose` for detailed output
3. Check `.git/hooks/` files are executable
4. Verify all dependencies are installed
5. Open an issue in the project repository

---

**Remember**: Hooks are here to help maintain code quality and consistency. They catch issues early and save time in the long run. If a hook seems overly strict or causes problems, let's discuss adjusting it rather than bypassing it.