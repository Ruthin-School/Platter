# 💻 Development Environment Setup Guide

> **Navigation:** [Documentation Home](../README.md) → [Development](README.md) → Setup

**Reading Time:** Approximately 20 minutes  
**Complexity:** Beginner to Intermediate  
**Prerequisites:** Basic command-line knowledge and text editor familiarity

## 📖 What This Guide Covers

This guide provides comprehensive instructions for setting up your development environment for the Platter project. By the end of this guide, you will have:

1. Installed all required prerequisites
2. Configured your development tools and IDE (Integrated Development Environment)
3. Cloned and built the project successfully
4. Understood the project structure and organisation
5. Learned the development workflow and best practices

---

## 📋 Section 1: Prerequisites Installation

### 1.1: Rust Programming Language

**Purpose:** Rust is the programming language used to build Platter.

**Required version:** Rust 1.80 or higher

#### Step 1.1.1: Install Rust on Linux or macOS

Execute these commands in your terminal:

```bash
# Download and run the Rust installer
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Load Rust into your current terminal session
source "$HOME/.cargo/env"
```

**What these commands do:**
- Line 1: Downloads the official Rust installer using secure HTTPS and executes it
- Line 2: Adds Rust tools to your PATH environment variable in the current terminal

#### Step 1.1.2: Install Rust on Windows

Follow these steps in sequence:

1. Download the installer from [rustup-init.exe](https://win.rustup.rs/)
2. Run the downloaded file
3. When prompted, select option "1) Proceed with installation"
4. When prompted, allow the installer to modify your PATH environment variable
5. Restart your terminal to apply PATH changes

#### Step 1.1.3: Verify Rust Installation

Execute these commands to confirm installation:

```bash
# Check Rust compiler version
rustc --version

# Check Cargo package manager version
cargo --version
```

**Expected output format:**
```
rustc 1.80.0 (or higher)
cargo 1.80.0 (or higher)
```

> ⚠️ **Troubleshooting:** If commands are not recognized, restart your terminal and try again. The PATH environment variable requires a fresh terminal session to take effect.

### 1.2: Git Version Control

**Purpose:** Git manages source code changes and enables collaboration.

**Installation:**

Visit [git-scm.com](https://git-scm.com/) and download the installer for your operating system.

**Verify installation:**

```bash
git --version
```

**Expected output:** `git version 2.x.x` (any recent version)

---

## 🔧 Section 2: IDE Configuration

### 2.1: Recommended IDE: VSCodium

**Purpose:** VSCodium provides a free, open-source code editor optimised for development.

**Download:** [vscodium.com](https://vscodium.com/)

### 2.2: Essential Extensions

Install these extensions for optimal Rust development:

#### Extension 2.2.1: rust-analyzer

**Purpose:** Provides real-time compiler feedback and code intelligence.

**Installation:**
1. Open VSCodium
2. Press `Ctrl+Shift+X` (Windows/Linux) or `Cmd+Shift+X` (macOS)
3. Search for "rust-analyzer"
4. Click "Install"

**Features provided:**
- Code completion
- Inline error messages
- Go to definition
- Find references

#### Extension 2.2.2: Error Lens

**Purpose:** Displays error messages inline in the code editor.

**Installation:** Search for "Error Lens" in the extensions marketplace.

**Benefits:**
- Immediate visual error feedback
- Reduced need to check Problems panel
- Clearer error context

#### Extension 2.2.3: CodeLLDB

**Purpose:** Enables debugging support for Rust applications.

**Installation:** Search for "CodeLLDB" in the extensions marketplace.

**Usage:** Set breakpoints and debug applications directly in VSCodium.

#### Extension 2.2.4: Even Better TOML

**Purpose:** Provides syntax highlighting and validation for `Cargo.toml` files.

**Installation:** Search for "Even Better TOML" in the extensions marketplace.

#### Extension 2.2.5: Crates

**Purpose:** Displays available versions for dependencies in `Cargo.toml`.

**Installation:** Search for "Crates" in the extensions marketplace.

**Features:**
- Shows latest version numbers
- Highlights outdated dependencies
- One-click updates

---

## 🛠️ Section 3: Recommended Development Tools

### 3.1: Cargo Extensions

Install these extensions to enhance your development workflow:

```bash
# Install all recommended tools in one command
cargo install cargo-watch cargo-audit cargo-deny
```

#### Tool 3.1.1: cargo-watch

**Purpose:** Automatically rebuilds your project when files change.

**Usage:**

```bash
# Run application with automatic rebuild
cargo watch -x run

# Run tests with automatic rebuild
cargo watch -x test

# Run custom command with automatic rebuild
cargo watch -x "build --release"
```

**Benefits:**
- Faster development iteration
- Immediate feedback on code changes
- Reduced manual command execution

#### Tool 3.1.2: cargo-audit

**Purpose:** Scans dependencies for known security vulnerabilities.

**Usage:**

```bash
# Check for security vulnerabilities
cargo audit

# Check and display detailed information
cargo audit --color always
```

**When to use:**
- Before production deployment
- Monthly as part of maintenance
- After updating dependencies

#### Tool 3.1.3: cargo-deny

**Purpose:** Checks licences and security of dependencies.

**Usage:**

```bash
# Check licence compliance
cargo deny check licenses

# Check for security advisories
cargo deny check advisories

# Check for banned dependencies
cargo deny check bans
```

**Benefits:**
- Ensures licence compliance
- Identifies security issues
- Enforces dependency policies

---

## 📥 Section 4: Project Setup

### 4.1: Clone the Repository

Execute these commands in your terminal:

```bash
# Download the Platter source code
git clone https://github.com/Ruthin-School/Platter.git

# Navigate into the downloaded directory
cd Platter
```

**What happens:**
1. Git downloads the complete project history and files
2. A new directory named `Platter` is created
3. Your current directory changes to the project folder

### 4.2: Build Dependencies

Execute this command:

```bash
# Download and compile all dependencies
cargo build
```

**What happens during first build:**

1. **Dependency Resolution** (10-30 seconds)
   - Cargo reads `Cargo.toml`
   - Cargo resolves all dependency versions
   - Cargo creates `Cargo.lock` file

2. **Dependency Download** (2-5 minutes)
   - Cargo downloads all required libraries
   - Downloads are cached in `~/.cargo/registry`

3. **Compilation** (3-10 minutes)
   - Cargo compiles all dependencies
   - Compiled libraries cached in `target/debug/deps`

4. **Application Build** (30-60 seconds)
   - Cargo compiles Platter source code
   - Creates executable in `target/debug/platter`

> 📘 **Note:** Subsequent builds will be much faster because dependencies are already compiled and cached.

### 4.3: Configure Data Directory Permissions

The application requires write access to the `data/` directory.

#### On Linux or macOS:

```bash
# Set appropriate permissions
chmod 755 data/
```

**What this command does:**
- `7` (owner): Read, write, execute permissions
- `5` (group): Read and execute permissions
- `5` (others): Read and execute permissions

#### On Windows PowerShell:

```powershell
# Grant full control to current user
icacls "data" /grant "$($env:USERNAME):(F)" /T
```

**What this command does:**
- Grants full control permissions to current user
- Applies recursively to all files in data directory

### 4.4: Run the Application

Execute this command:

```bash
# Build and run the application
cargo run
```

**Expected output:**

```
   Compiling platter v1.0.0
    Finished dev [unoptimized + debuginfo] target(s) in 2.34s
     Running `target/debug/platter`
Server running at http://localhost:8080
```

**Access the application:**

Open your web browser and navigate to:
```
http://localhost:8080
```

**What you should see:**
- Platter homepage displaying the current menu or login page

---

## 🔄 Section 5: Development Workflow

### 5.1: Terminal Configuration Tips

Improve your development experience with these terminal optimisations:

#### Tip 5.1.1: Create Shell Aliases

Add these aliases to your shell configuration file (`~/.bashrc`, `~/.zshrc`, or equivalent):

```bash
# Rust development aliases
alias cr='cargo run'
alias ct='cargo test'
alias cc='cargo check'
alias cw='cargo watch -x run'
alias cb='cargo build'
alias cbr='cargo build --release'
```

**Usage after reloading shell:**
- `cr` instead of `cargo run`
- `ct` instead of `cargo test`
- `cw` instead of `cargo watch -x run`

**Reload shell configuration:**
```bash
source ~/.bashrc  # or ~/.zshrc
```

#### Tip 5.1.2: Customise Your Prompt

Display Rust version in your terminal prompt:

Add to your shell configuration:

```bash
# Show Rust version in prompt
export PS1="\[\e[32m\]\u@\h \[\e[33m\]\w \[\e[31m\](\$(rustc --version | cut -d' ' -f2))\[\e[0m\] $ "
```

**Result:** Your prompt will display: `user@host /path (1.80.0) $`

#### Tip 5.1.3: Use Terminal Multiplexers

**tmux (Terminal Multiplexer) usage:**

```bash
# Create a named session for Platter development
tmux new -s platter-dev

# Split terminal horizontally
Ctrl+B then "

# Split terminal vertically
Ctrl+B then %

# Switch between panes
Ctrl+B then arrow keys

# Detach from session
Ctrl+B then D

# Reattach to session
tmux attach -t platter-dev
```

**Benefits:**
- Multiple terminal panes in one window
- Sessions persist after terminal close
- Improved workflow organisation

### 5.2: Active Development Workflow

Use cargo-watch for automatic rebuilds during development:

```bash
# Install cargo-watch if not already installed
cargo install cargo-watch

# Run with automatic rebuild on file changes
cargo watch -x run
```

**What happens:**
1. cargo-watch monitors project files for changes
2. When a file is saved, cargo-watch automatically runs `cargo run`
3. Application restarts with new changes
4. You see immediate feedback in the terminal

**Alternative commands:**

```bash
# Run tests automatically
cargo watch -x test

# Check code without running
cargo watch -x check
```

### 5.3: Feature Development Process

Follow these steps when developing a new feature:

**Step 5.3.1: Create Feature Branch**

```bash
# Create and switch to new branch
git checkout -b feature/your-feature-name
```

**Naming conventions:**
- `feature/` prefix for new features
- `fix/` prefix for bug fixes
- `docs/` prefix for documentation changes

**Step 5.3.2: Make Changes**

1. Edit code files as needed
2. Follow Rust coding standards
3. Add tests for new functionality
4. Update documentation

**Step 5.3.3: Test Changes**

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

**Step 5.3.4: Format and Lint Code**

```bash
# Format code automatically
cargo fmt

# Run linter
cargo clippy

# Check for errors without building
cargo check
```

**Step 5.3.5: Commit Changes**

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "feat(module): add new feature description"
```

**Commit message format:**
```
type(scope): description

[optional body]
```

Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Step 5.3.6: Push and Create Pull Request**

```bash
# Push branch to remote
git push origin feature/your-feature-name
```

Then create a pull request on GitHub.

---

## 📁 Section 6: Project Structure

### 6.1: Directory Organisation

```
Platter/
├── Cargo.toml                 # Project configuration and dependencies
├── README.md                  # Project overview
├── API.md                     # API documentation
├── DESIGN.md                  # Design documentation
├── DEVELOPMENT.md             # Development guide
├── SECURITY.md                # Security documentation
├── DEPLOYMENT.md              # Deployment guide
├── LICENSE                    # Project licence
├── src/                       # Source code directory
│   ├── main.rs               # Application entry point
│   ├── auth.rs               # Authentication logic
│   ├── error_handler.rs      # Error handling
│   ├── handlers.rs           # Request handlers
│   ├── storage.rs            # Data storage logic
│   └── scheduler.rs          # Scheduling logic
├── data/                      # Data storage directory
│   ├── admin_users.json      # Administrator accounts
│   ├── menu_items.json       # Menu items
│   ├── menu_presets.json     # Menu presets
│   ├── menu_schedules.json   # Menu schedules
│   └── notices.json          # Notices and announcements
├── templates/                 # HTML templates
│   ├── base.html             # Base template
│   ├── menu.html             # Menu display template
│   ├── item_detail.html      # Item details template
│   ├── admin/                # Admin templates
│   │   ├── dashboard.html    # Admin dashboard
│   │   ├── login.html        # Login page
│   │   ├── presets.html      # Preset management
│   │   └── schedules.html    # Schedule management
│   └── partials/             # Reusable template components
│       ├── footer.html       # Footer component
│       └── header.html       # Header component
├── static/                    # Static assets
│   ├── css/
│   │   └── style.css         # Application styles
│   └── images/
│       ├── logo-white-no-text.svg
│       └── logo-white-text.png
├── assets/                    # Source assets
│   └── branding/
│       ├── logo-coloured-text.svg
│       ├── logo-white-no-text.svg
│       └── logo-white-text.png
└── target/                    # Build output (not committed to Git)
```

### 6.2: Key Files Explained

| File | Purpose | When to Edit |
|------|---------|--------------|
| `Cargo.toml` | Dependencies and project metadata | Adding dependencies, changing version |
| `src/main.rs` | Application entry point | Changing server configuration |
| `src/auth.rs` | Authentication logic | Modifying authentication behaviour |
| `src/handlers.rs` | HTTP request handlers | Adding new routes or endpoints |
| `src/storage.rs` | Data persistence | Changing data storage logic |
| `data/*.json` | Application data | Via admin interface (not directly) |

---

## 🎨 Section 7: Code Style Guidelines

### 7.1: Rust Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

**Naming conventions:**

| Element | Style | Example |
|---------|-------|---------|
| Functions and variables | `snake_case` | `get_menu_item`, `user_name` |
| Types and traits | `CamelCase` | `MenuItem`, `Authenticate` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_ITEMS`, `DEFAULT_PORT` |
| Modules | `snake_case` | `auth`, `storage` |

**Formatting rules:**
- Maximum line length: 100 characters
- Indentation: 4 spaces (no tabs)
- Trailing commas in multi-line lists
- One statement per line

**Use automated formatting:**

```bash
# Format all code
cargo fmt

# Check formatting without changes
cargo fmt -- --check
```

### 7.2: Documentation Standards

Add documentation comments for public items:

```rust
/// Retrieves a menu item by its unique identifier.
///
/// # Arguments
///
/// * `id` - The unique identifier of the menu item
///
/// # Returns
///
/// Returns `Some(MenuItem)` if found, `None` otherwise
///
/// # Example
///
/// ```
/// let item = get_item("abc123");
/// if let Some(item) = item {
///     println!("Found: {}", item.name);
/// }
/// ```
pub fn get_item(id: &str) -> Option<MenuItem> {
    // implementation
}
```

---

## 🧪 Section 8: Testing

### 8.1: Running Tests

```bash
# Run all tests
cargo test

# Run tests with detailed output
cargo test -- --nocapture

# Run specific test by name
cargo test test_function_name

# Run tests in specific module
cargo test auth::tests
```

### 8.2: Writing Tests

**Unit test structure:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_creation() {
        // Given: test setup
        let item = MenuItem::new("Test Item", "Description");
        
        // When: action performed
        let name = item.get_name();
        
        // Then: assertion
        assert_eq!(name, "Test Item");
    }
}
```

**Test organisation principles:**
- Place unit tests in same file as code being tested
- Use `#[cfg(test)]` to exclude from production builds
- Follow Given-When-Then pattern for clarity
- Test one behaviour per test function

---

## 🔨 Section 9: Building

### 9.1: Development Build

```bash
# Build with debug information
cargo build
```

**Output location:** `target/debug/platter`

**Characteristics:**
- Includes debug symbols
- Not optimised for speed
- Larger binary size
- Faster compilation

### 9.2: Release Build

```bash
# Build with optimisations
cargo build --release
```

**Output location:** `target/release/platter`

**Characteristics:**
- Optimised for speed and size
- No debug symbols
- Smaller binary size
- Slower compilation
- **Use this for production deployment**

### 9.3: Cross-Compilation

Build for different operating systems:

```bash
# Install target (example: Linux to Windows)
rustup target add x86_64-pc-windows-gnu

# Build for Windows from Linux
cargo build --target x86_64-pc-windows-gnu --release
```

**Common targets:**
- `x86_64-pc-windows-gnu` - Windows 64-bit
- `x86_64-apple-darwin` - macOS 64-bit
- `x86_64-unknown-linux-gnu` - Linux 64-bit

---

## 💾 Section 10: Data Management

### 10.1: Data File Structure

The application uses JSON-based file storage in the `data/` directory:

| File | Contains | Edit Method |
|------|----------|-------------|
| `admin_users.json` | Administrator accounts | Admin interface only |
| `menu_items.json` | Menu items | Admin interface only |
| `notices.json` | Announcements | Admin interface only |
| `menu_presets.json` | Menu combinations | Admin interface only |
| `menu_schedules.json` | Scheduled menus | Admin interface only |

> ⚠️ **Critical Warning:** Never edit data files whilst the application is running. This can cause data corruption. Always stop the application first.

### 10.2: Default Administrator Credentials

**Development credentials:**
- Username: `admin`
- Password: `admin123`

> ⚠️ **Security Warning:** These credentials are for DEVELOPMENT ONLY. Change them before deploying to production. See [Security Documentation](../architecture/security.md#best-practices) for instructions.

### 10.3: Data Backup

Create regular backups of the `data/` directory:

```bash
# Create timestamped backup
tar -czf backup-$(date +%Y%m%d-%H%M%S).tar.gz data/

# Restore from backup
tar -xzf backup-20250103-120000.tar.gz
```

**Backup recommendations:**
- Create backups before major changes
- Store backups in separate location
- Test backup restoration regularly
- Automate backups in production

---

## 🔧 Section 11: Troubleshooting

### 11.1: Common Build Issues

#### Issue 11.1.1: Dependency Resolution Fails

**Symptom:** Error messages about conflicting dependency versions

**Solution:**

```bash
# Clean project
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build
```

#### Issue 11.1.2: Compilation Errors

**Symptom:** Rust compiler errors during build

**Solution:**

```bash
# Check for errors without full build
cargo check

# View detailed error information
cargo build --verbose
```

#### Issue 11.1.3: Out of Memory During Build

**Symptom:** Build process killed due to insufficient memory

**Solution:**

```bash
# Build with reduced parallelism
cargo build -j 1
```

### 11.2: Runtime Issues

#### Issue 11.2.1: Permission Errors (Linux/macOS)

**Symptom:** "Permission denied" errors accessing `data/` directory

**Solution:**

```bash
# Fix directory permissions
chmod -R 755 data/
```

#### Issue 11.2.2: Port Already in Use

**Symptom:** "Address already in use" error on startup

**Solution:**

```bash
# Run on different port
PORT=3000 cargo run

# Or find and stop conflicting process
lsof -i :8080  # Find process using port 8080
kill <PID>     # Stop the process
```

#### Issue 11.2.3: Data File Corruption

**Symptom:** Application fails to load data files

**Solution:**

1. Stop the application
2. Restore from latest backup
3. Verify JSON file syntax using `jq`:

```bash
# Validate JSON syntax
jq empty data/menu_items.json
```

If valid, `jq` outputs nothing. If invalid, `jq` displays error details.

---

## 🛠️ Section 12: Useful Development Commands

### 12.1: Code Quality Commands

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check code without building
cargo check

# Run linter with all warnings
cargo clippy -- -W clippy::all

# Fix automatically fixable issues
cargo fix
```

### 12.2: Dependency Management Commands

```bash
# Update dependencies
cargo update

# Audit for security issues
cargo audit

# Check licences
cargo deny check licenses

# Show dependency tree
cargo tree
```

### 12.3: Performance Commands

```bash
# Build with optimisations
cargo build --release

# Run benchmarks (if configured)
cargo bench

# Profile binary size
cargo bloat --release
```

---

## 🎯 Section 13: Next Steps

After completing development environment setup, proceed with these tasks:

### For New Contributors

Complete these tasks in sequence:

1. **Review Contributing Guidelines** – Read [Contributing Guide](contributing.md)
2. **Understand Testing Practices** – Study [Testing Guide](testing.md)
3. **Explore the Codebase** – Examine source files in `src/` directory
4. **Make First Contribution** – Start with a "good first issue" from GitHub

### For Feature Development

Complete these tasks in sequence:

1. **Study API Documentation** – Review [API Reference](../api/reference.md)
2. **Understand Architecture** – Read [Design Documentation](../architecture/design.md)
3. **Review Security Guidelines** – Study [Security Documentation](../architecture/security.md)
4. **Plan Feature Implementation** – Create design document for complex features

### For Deployment Preparation

Complete these tasks in sequence:

1. **Configure Environment** – Follow [Configuration Guide](../guides/configuration.md)
2. **Review Production Requirements** – Study [Production Deployment](../deployment/production.md)
3. **Test Security Settings** – Verify [Security Checklist](../architecture/security.md#production-security-checklist)

---

## 📖 Related Documentation

Access these resources for additional information:

- **[Contributing Guidelines](contributing.md)** – How to contribute to the project
- **[Testing Guide](testing.md)** – Testing practices and procedures
- **[API Reference](../api/reference.md)** – API documentation for development
- **[Design Documentation](../architecture/design.md)** – System architecture overview
- **[Configuration Guide](../guides/configuration.md)** – Environment configuration options

---

[← Back to Development](README.md) | [Documentation Home](../README.md)
