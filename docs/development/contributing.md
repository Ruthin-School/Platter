# 🤝 Contributing Guidelines

> **Navigation:** [Documentation Home](../README.md) → [Development](README.md) → Contributing

**Reading Time:** Approximately 20 minutes  
**Complexity:** Beginner to Intermediate  
**Prerequisites:** Basic Git knowledge and understanding of open-source contribution

## 📖 What This Guide Covers

This guide explains how to contribute to the Platter project. By the end of this guide, you will understand:

1. The code of conduct and community expectations
2. How to set up your contribution environment
3. The development workflow and branching strategy
4. Code style requirements and quality standards
5. The pull request process from submission to merge
6. Documentation and testing requirements

---

## 📋 Section 1: Community Standards

By participating in this project, you agree to maintain a professional, respectful, and collaborative environment.

### 1.1: Our Values

All contributors are expected to uphold these core values:

| Value | Meaning | Example |
|-------|---------|---------|
| **Respect** | Treat all contributors with courtesy and professionalism | Use polite language and acknowledge others' contributions |
| **Inclusivity** | Welcome diverse perspectives and backgrounds | Consider accessibility and different experience levels |
| **Constructive Collaboration** | Provide helpful, actionable feedback | Suggest improvements with specific examples and reasoning |
| **Knowledge Sharing** | Help others learn and grow | Share expertise and assist fellow contributors |
| **Professionalism** | Maintain high standards of conduct | Focus on technical merit and project improvements |

### 1.2: Professional Conduct

We foster a positive environment where all contributors:

- ✅ Communicate respectfully and professionally
- ✅ Focus on constructive technical discussions
- ✅ Welcome questions and help others learn
- ✅ Respect privacy and confidentiality
- ✅ Assume good intentions and give benefit of the doubt

### 1.3: Standards Enforcement

To maintain our community standards, we will:

1. Address concerns privately and professionally
2. Provide clear guidance on expected behaviour
3. Take appropriate action to protect the community when needed

Our goal is to create an environment where everyone can contribute effectively and professionally.

---

## 🚀 Section 2: Getting Started

### 2.1: Fork the Repository

**Step 2.1.1: Create Your Fork**

1. Visit the [Platter repository](https://github.com/Ruthin-School/Platter)
2. Click the "Fork" button in the top-right corner
3. Select your GitHub account as the destination
4. Wait for GitHub to complete the fork

**What happens:**
- GitHub creates a complete copy of the repository under your account
- You receive full read-write access to your fork
- Your fork remains linked to the original repository

**Step 2.1.2: Clone Your Fork Locally**

Execute these commands in your terminal:

```bash
# Replace YOUR-USERNAME with your GitHub username
git clone https://github.com/YOUR-USERNAME/Platter.git

# Navigate into the cloned directory
cd Platter
```

**Step 2.1.3: Add Upstream Remote**

Execute this command to track the original repository:

```bash
# Add the original repository as 'upstream'
git remote add upstream https://github.com/Ruthin-School/Platter.git

# Verify remotes are configured correctly
git remote -v
```

**Expected output:**
```
origin    https://github.com/YOUR-USERNAME/Platter.git (fetch)
origin    https://github.com/YOUR-USERNAME/Platter.git (push)
upstream  https://github.com/Ruthin-School/Platter.git (fetch)
upstream  https://github.com/Ruthin-School/Platter.git (push)
```

### 2.2: Set Up Development Environment

Follow the complete [Development Setup Guide](setup.md) to configure your environment.

**Quick setup checklist:**
- [ ] Install Rust 1.80 or higher
- [ ] Install Git version control
- [ ] Configure IDE with recommended extensions
- [ ] Clone repository and build project
- [ ] Verify application runs successfully

### 2.3: Create a Working Branch

**Step 2.3.1: Update Your Main Branch**

```bash
# Switch to main branch
git checkout main

# Fetch latest changes from upstream
git fetch upstream

# Merge upstream changes into your local main
git merge upstream/main

# Push updated main to your fork
git push origin main
```

**Step 2.3.2: Create Feature Branch**

```bash
# Create and switch to new branch
git checkout -b feature/your-feature-name
```

**Branch naming conventions:**

| Type | Prefix | Example | When to Use |
|------|--------|---------|-------------|
| New feature | `feature/` | `feature/add-dietary-filters` | Adding new functionality |
| Bug fix | `fix/` | `fix/menu-display-error` | Fixing defects |
| Documentation | `docs/` | `docs/update-api-reference` | Documentation changes only |
| Refactoring | `refactor/` | `refactor/auth-module` | Code improvements without behaviour changes |
| Testing | `test/` | `test/add-integration-tests` | Adding or updating tests |

---

## 🔄 Section 3: Development Workflow

### 3.1: Making Changes

**Step 3.1.1: Write Clear, Readable Code**

Follow these principles:

1. **Descriptive naming:** Use names that explain purpose
   ```rust
   // Good
   fn calculate_total_price(items: &[MenuItem]) -> f64
   
   // Avoid
   fn calc(x: &[MenuItem]) -> f64
   ```

2. **Small functions:** Each function should have one clear purpose
   ```rust
   // Good - single responsibility
   fn validate_email(email: &str) -> bool
   fn send_notification(email: &str, message: &str) -> Result<()>
   
   // Avoid - multiple responsibilities
   fn validate_and_send(email: &str, message: &str) -> Result<()>
   ```

3. **Clear comments:** Explain why, not what
   ```rust
   // Good - explains reasoning
   // Use Argon2 because it's memory-hard and resistant to GPU attacks
   let hash = argon2::hash_password(password)?;
   
   // Avoid - states the obvious
   // Hash the password
   let hash = argon2::hash_password(password)?;
   ```

**Step 3.1.2: Follow Code Style Guidelines**

Apply these rules consistently:

- Use 4 spaces for indentation (never tabs)
- Limit lines to 100 characters maximum
- Add trailing commas in multi-line lists
- Place opening braces on same line as declaration
- Use explicit types when clarity improves

**Automatic formatting:**

```bash
# Format all code before committing
cargo fmt

# Verify formatting without changes
cargo fmt -- --check
```

**Step 3.1.3: Add Tests for New Functionality**

Every new feature requires tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_creation() {
        // Given: Input data
        let name = "Test Item";
        let description = "Test Description";
        
        // When: Create menu item
        let item = MenuItem::new(name, description);
        
        // Then: Verify properties
        assert_eq!(item.name, name);
        assert_eq!(item.description, description);
    }
}
```

**Step 3.1.4: Update Documentation**

Update documentation when you:

- Add new features or endpoints
- Change existing functionality
- Modify configuration options
- Fix bugs that affect documented behaviour

### 3.2: Testing Your Changes

**Step 3.2.1: Run Complete Test Suite**

```bash
# Run all tests
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run specific test module
cargo test auth::tests
```

**All tests must pass before submitting a pull request.**

**Step 3.2.2: Run Code Quality Checks**

```bash
# Check for errors
cargo check

# Run linter (address all warnings)
cargo clippy

# Ensure code is formatted
cargo fmt -- --check
```

**Step 3.2.3: Test Manually**

1. Build and run the application
   ```bash
   cargo run
   ```

2. Access the application at `http://localhost:8080`

3. Test your changes through the user interface

4. Verify no regressions in existing functionality

### 3.3: Commit Your Changes

**Step 3.3.1: Stage Changes**

```bash
# Stage specific files
git add src/handlers.rs src/auth.rs

# Or stage all changes (use carefully)
git add .
```

**Step 3.3.2: Create Descriptive Commit**

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

**Commit message structure:**
```
type(scope): description

[optional body]

[optional footer]
```

**Commit types:**

| Type | Purpose | Example |
|------|---------|---------|
| `feat` | New feature | `feat(auth): add OAuth logout endpoint` |
| `fix` | Bug fix | `fix(storage): resolve data corruption on concurrent writes` |
| `docs` | Documentation only | `docs: update API documentation for menu endpoints` |
| `style` | Code style changes | `style: apply consistent formatting to auth module` |
| `refactor` | Code refactoring | `refactor(handlers): simplify error handling logic` |
| `perf` | Performance improvement | `perf(storage): optimise menu item retrieval` |
| `test` | Adding/updating tests | `test(auth): add integration tests for OAuth flow` |
| `chore` | Maintenance tasks | `chore: update dependencies to latest versions` |
| `ci` | CI/CD changes | `ci: add automated security scanning` |

**Example commits:**

```bash
# Feature addition
git commit -m "feat(menu): add dietary filter options

Implements filtering for vegetarian, vegan, and gluten-free items.
Users can now select multiple dietary requirements simultaneously.

Closes #123"

# Bug fix
git commit -m "fix(auth): prevent session fixation vulnerability

Session ID now regenerated after successful authentication.
This prevents attackers from hijacking user sessions.

Fixes #456"

# Documentation
git commit -m "docs: add OAuth setup guide

Provides step-by-step instructions for configuring
Microsoft Entra ID authentication."
```

**Commit best practices:**

1. **Present tense:** Use "add feature" not "added feature"
2. **Imperative mood:** Use "move cursor to" not "moves cursor to"
3. **Specific scope:** Reference the affected module
4. **Reference issues:** Include issue numbers (Closes #123, Fixes #456)
5. **Limit first line:** Maximum 72 characters
6. **Explain why:** Describe reasoning in commit body

---

## 🔀 Section 4: Pull Request Process

### 4.1: Prepare Your Pull Request

**Step 4.1.1: Update Your Branch with Latest Changes**

Before creating a pull request, synchronise with the main branch:

```bash
# Fetch latest upstream changes
git fetch upstream

# Rebase your branch on upstream/main
git rebase upstream/main

# Resolve any conflicts if they occur
# Edit conflicted files, then:
git add <resolved-files>
git rebase --continue

# Force push to update your fork (only after rebase)
git push origin feature/your-feature-name --force
```

**What rebase does:**
1. Temporarily removes your commits
2. Updates your branch with upstream changes
3. Reapplies your commits on top
4. Results in a clean, linear history

**Step 4.1.2: Run Final Quality Checks**

Execute these commands to verify quality:

```bash
# Run all tests
cargo test

# Check code quality
cargo clippy

# Verify formatting
cargo fmt -- --check

# Build release version
cargo build --release
```

**All checks must pass before proceeding.**

### 4.2: Create Pull Request

**Step 4.2.1: Push Your Branch**

```bash
# Push your branch to your fork
git push origin feature/your-feature-name
```

**Step 4.2.2: Open Pull Request on GitHub**

1. Navigate to your fork on GitHub
2. Click "Compare & pull request" button
3. Select base repository: `Ruthin-School/Platter`
4. Select base branch: `main`
5. Verify head repository is your fork
6. Verify compare branch is your feature branch

**Step 4.2.3: Complete Pull Request Template**

Provide comprehensive information:

```markdown
## Description
Clear explanation of what this PR does and why.

## Type of Change
- [ ] Bug fix (non-breaking change fixing an issue)
- [ ] New feature (non-breaking change adding functionality)
- [ ] Breaking change (fix or feature causing existing functionality to change)
- [ ] Documentation update

## Related Issues
Closes #123
Relates to #456

## Changes Made
- Implemented dietary filtering for menu items
- Added filter UI components to menu page
- Updated API to support filter parameters
- Added comprehensive tests for filtering logic

## Testing Performed
- [ ] All existing tests pass
- [ ] Added new tests for this feature
- [ ] Manually tested in browser
- [ ] Tested on mobile viewport
- [ ] Verified accessibility with screen reader

## Screenshots
(If applicable, add screenshots demonstrating the changes)

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review of code completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] No new warnings introduced
- [ ] Tests added and passing
```

### 4.3: Pull Request Requirements

Your PR must meet these requirements:

**Required checklist:**
- [ ] All CI (Continuous Integration) checks pass
- [ ] Tests included for new functionality
- [ ] Documentation updated where relevant
- [ ] Code follows style guidelines
- [ ] Clear description of changes provided
- [ ] Related issues referenced using keywords

**Quality standards:**
- [ ] No clippy warnings introduced
- [ ] Code coverage maintained or improved
- [ ] Breaking changes clearly documented
- [ ] Migration guide provided (if breaking changes)

### 4.4: Code Review Process

**Step 4.4.1: Respond to Reviewer Feedback**

When reviewers provide comments:

1. **Read carefully:** Understand the concern raised
2. **Ask questions:** Request clarification if needed
3. **Make changes:** Address feedback in new commits
4. **Explain decisions:** Justify choices when appropriate
5. **Be respectful:** Maintain professional tone

**Example response:**

```markdown
Thanks for the feedback! I've made the following changes:

1. Refactored the authentication logic as suggested
2. Added error handling for edge cases
3. Updated tests to cover the new scenarios

Regarding the caching approach, I chose to implement 
TTL-based expiration because [explanation]. However, 
I'm open to the LRU approach if you think it's better
for this use case.
```

**Step 4.4.2: Update Your Pull Request**

```bash
# Make requested changes
# ... edit files ...

# Commit changes
git add .
git commit -m "refactor: address code review feedback

- Improve error handling in auth module
- Add input validation tests
- Update documentation with examples"

# Push updates
git push origin feature/your-feature-name
```

**Your pull request updates automatically when you push to the branch.**

**Step 4.4.3: Request Re-review**

After addressing feedback:

1. Click "Re-request review" button on GitHub
2. Add comment summarising changes made
3. Tag reviewer if urgent

### 4.5: Merging Process

**Merge requirements:**
- ✅ At least one approval from maintainer
- ✅ All CI checks passing
- ✅ No merge conflicts
- ✅ Branch up to date with main
- ✅ All review comments resolved

**Merge strategies:**

| Strategy | When Used | Result |
|----------|-----------|--------|
| **Squash and merge** | Feature branches (preferred) | Single commit in main branch |
| **Merge commit** | Multiple logical commits | All commits preserved |
| **Rebase and merge** | Clean linear history desired | Commits rebased onto main |

> 📘 **Reference:** For detailed merge strategy guidance, see the [Merge Strategies Guide](merge-strategies.md)

**After merge:**

1. Delete your feature branch (GitHub offers this option)
2. Update your local repository:
   ```bash
   git checkout main
   git pull upstream main
   git push origin main
   ```

---

## 🎨 Section 5: Code Style Standards

### 5.1: Rust Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

**Naming conventions:**

```rust
// Functions and variables: snake_case
fn calculate_total_price(menu_items: &[MenuItem]) -> f64

// Types and traits: CamelCase
struct MenuItem { ... }
trait Authenticate { ... }

// Constants: SCREAMING_SNAKE_CASE
const MAX_ITEMS: usize = 100;
const DEFAULT_PORT: u16 = 8080;

// Modules: snake_case
mod menu_handler;
mod auth_service;
```

### 5.2: Code Organisation

**Import organisation:**

```rust
// 1. Standard library imports
use std::collections::HashMap;
use std::sync::Arc;

// 2. External crate imports (alphabetical)
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

// 3. Internal imports (alphabetical)
use crate::auth::AuthService;
use crate::models::MenuItem;
```

**Module structure:**

```rust
// 1. Imports
use std::collections::HashMap;

// 2. Constants
const MAX_RETRIES: u32 = 3;

// 3. Type definitions
struct Config {
    port: u16,
}

// 4. Implementations
impl Config {
    fn new(port: u16) -> Self {
        Self { port }
    }
}

// 5. Public functions
pub fn start_server(config: Config) -> Result<()> {
    // implementation
}

// 6. Private functions
fn validate_config(config: &Config) -> bool {
    // implementation
}

// 7. Tests
#[cfg(test)]
mod tests {
    use super::*;
    // test functions
}
```

### 5.3: Documentation Standards

**Function documentation:**

```rust
/// Retrieves a menu item by its unique identifier.
///
/// This function searches the in-memory cache first, falling back
/// to file storage if the item is not found in cache.
///
/// # Arguments
///
/// * `id` - The unique identifier of the menu item (UUID format)
///
/// # Returns
///
/// Returns `Some(MenuItem)` if the item exists, `None` otherwise.
///
/// # Examples
///
/// ```
/// let item = get_menu_item("550e8400-e29b-41d4-a716-446655440000");
/// match item {
///     Some(item) => println!("Found: {}", item.name),
///     None => println!("Item not found"),
/// }
/// ```
///
/// # Errors
///
/// This function does not return errors but logs warnings if
/// storage access fails.
pub fn get_menu_item(id: &str) -> Option<MenuItem> {
    // implementation
}
```

### 5.4: Error Handling Standards

**Use Result types for fallible operations:**

```rust
use anyhow::{Context, Result};

fn load_configuration() -> Result<Config> {
    let content = std::fs::read_to_string("config.json")
        .context("Failed to read configuration file")?;
    
    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse configuration JSON")?;
    
    validate_config(&config)
        .context("Configuration validation failed")?;
    
    Ok(config)
}
```

**Provide helpful error context:**

```rust
// Good - explains what failed and why
return Err(anyhow!("Failed to create menu item: name '{}' exceeds maximum length of {} characters", name, MAX_NAME_LENGTH));

// Avoid - generic error without context
return Err(anyhow!("Invalid input"));
```

---

## 📚 Section 6: Documentation Guidelines

### 6.1: When to Update Documentation

Update documentation when you:

| Change Type | Documentation Required |
|-------------|----------------------|
| Add new features | API reference, user guide, README |
| Change existing functionality | All affected documentation |
| Fix bugs affecting documented behaviour | Changelog, relevant guides |
| Add configuration options | Configuration reference |
| Modify API endpoints | API reference, examples |
| Change deployment process | Deployment guide |

### 6.2: Documentation Structure

**Standard document format:**

```markdown
# 📘 Document Title

> **Navigation:** [Documentation Home](../README.md) → [Section](README.md) → Document

**Reading Time:** Approximately X minutes  
**Complexity:** Beginner/Intermediate/Advanced  
**Prerequisites:** List of prerequisite knowledge

## 📖 What This Guide Covers

Brief introduction explaining document contents.

## 📋 Section 1: Topic

Content organised into logical sections...

## 📖 Related Documentation

- [Related Doc 1](link)
- [Related Doc 2](link)

---

[← Back to Section](README.md) | [Documentation Home](../README.md)
```

### 6.3: Writing Style

**Be clear and concise:**

```markdown
<!-- Good - direct and clear -->
Execute this command to install dependencies:
```bash
cargo build
```

<!-- Avoid - unnecessarily verbose -->
In order to install the dependencies that are required
for the project, you should execute the following command
in your terminal window:
```

**Use active voice:**

```markdown
<!-- Good -->
The server validates all inputs before processing.

<!-- Avoid -->
All inputs are validated by the server before they are processed.
```

**Provide concrete examples:**

```markdown
<!-- Good - specific example -->
Set the PORT environment variable to change the server port:
```bash
PORT=3000 cargo run
```

<!-- Avoid - abstract explanation -->
You can configure the port using environment variables.
```

### 6.4: Code Examples in Documentation

**Include complete, runnable examples:**

```rust
// Good - complete example
use platter::MenuItem;

fn main() {
    let item = MenuItem::new(
        "Veggie Burger",
        "Plant-based burger",
    );
    println!("Created: {}", item.name);
}

// Avoid - incomplete snippet
let item = MenuItem::new(/* ... */);
```

**Test all code examples:**

```bash
# Extract and test code examples
cargo test --doc
```

---

## 🧪 Section 7: Testing Guidelines

### 7.1: Test Coverage Requirements

**Minimum coverage expectations:**

| Code Type | Minimum Coverage | Target Coverage |
|-----------|-----------------|-----------------|
| New features | 80% | 90% |
| Bug fixes | Include regression test | N/A |
| Refactoring | Maintain existing | Improve if possible |

### 7.2: Writing Effective Tests

**Test naming convention:**

```rust
#[test]
fn test_<what>_<when>_<expected>() {
    // Given: Setup
    let input = "test@example.com";
    
    // When: Action
    let result = validate_email(input);
    
    // Then: Assertion
    assert!(result);
}
```

**Examples:**

```rust
#[test]
fn test_email_validation_with_valid_email_returns_true() {
    let email = "user@example.com";
    assert!(validate_email(email));
}

#[test]
fn test_email_validation_with_invalid_email_returns_false() {
    let email = "not-an-email";
    assert!(!validate_email(email));
}

#[test]
fn test_menu_item_creation_with_valid_data_succeeds() {
    let item = MenuItem::new("Test", "Description");
    assert_eq!(item.name, "Test");
}
```

### 7.3: Test Organisation

**Unit tests:**
- Place in same file as code being tested
- Use `#[cfg(test)]` module
- Test individual functions in isolation

**Integration tests:**
- Place in `tests/` directory
- Test complete workflows
- Verify component integration

---

## 🐛 Section 8: Reporting Issues

### 8.1: Before Reporting

Complete these checks:

1. **Search existing issues:** Your issue may already exist
2. **Verify in latest version:** Update and test again
3. **Isolate the problem:** Create minimal reproduction
4. **Gather information:** Collect relevant details

### 8.2: Bug Report Template

```markdown
**Bug Description**
Clear, concise description of the bug.

**Steps to Reproduce**
1. Start the application
2. Navigate to menu page
3. Click "Add Item" button
4. Observe error

**Expected Behaviour**
Item creation form should appear.

**Actual Behaviour**
Application displays "Internal Server Error" message.

**Environment**
- OS: Ubuntu 22.04
- Rust version: 1.80.0
- Platter version: 1.0.0
- Browser: Firefox 120

**Additional Context**
Error appears only when logged in as OAuth user,
not with admin credentials.

**Logs**
```
[ERROR] Failed to validate session: InvalidToken
Stack trace: ...
```
```

### 8.3: Feature Request Template

```markdown
**Feature Description**
Clear description of the proposed feature.

**Use Case**
Why is this feature needed? What problem does it solve?

**Proposed Solution**
How would you implement this feature?

**Alternatives Considered**
What other approaches did you consider?

**Additional Context**
Any other relevant information, screenshots, or examples.
```

---

## 🎯 Section 9: Next Steps

After reading the contributing guidelines, proceed with these tasks:

### For New Contributors

Complete these tasks in sequence:

1. **Set Up Development Environment** – Follow [Development Setup Guide](setup.md)
2. **Find Beginner-Friendly Issues** – Look for "good first issue" label on GitHub
3. **Ask Questions** – Join discussions, request clarification when needed
4. **Make First Contribution** – Start with documentation or small bug fixes

### For Experienced Contributors

Complete these tasks in sequence:

1. **Review Architecture** – Study [Design Documentation](../architecture/design.md)
2. **Understand Security Requirements** – Read [Security Guide](../architecture/security.md)
3. **Explore Testing Practices** – Review [Testing Guide](testing.md)
4. **Tackle Complex Issues** – Work on features requiring deeper knowledge

---

## 📖 Related Documentation

Access these resources for additional information:

- **[Development Setup Guide](setup.md)** – Set up your development environment
- **[Branch Protection Rules](branch-protection.md)** – GitHub branch protection configuration
- **[Merge Strategies Guide](merge-strategies.md)** – Detailed merge strategy documentation
- **[Git Workflow Guide](git-workflow.md)** – Git workflow and release tagging
- **[Git Hooks Guide](git-hooks.md)** – Pre-commit and commit-msg hooks
- **[Testing Guide](testing.md)** – Testing practices and procedures
- **[API Reference](../api/reference.md)** – API documentation for development
- **[Design Documentation](../architecture/design.md)** – Architecture overview and patterns

---

[← Back to Development](README.md) | [Documentation Home](../README.md)