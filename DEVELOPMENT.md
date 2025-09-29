# Development Guide

This guide provides comprehensive information for developers contributing to the Platter project.

## 📋 Table of Contents

- [Prerequisites](#prerequisites)
- [Initial Setup](#initial-setup)
- [Development Workflow](#development-workflow)
- [Project Structure](#project-structure)
- [Code Style](#code-style)
- [Testing](#testing)
- [Building](#building)
- [Data Management](#data-management)
- [Troubleshooting](#troubleshooting)

## Prerequisites

Before starting development, ensure you have:

### Rust Installation
Install Rust using rustup for all platforms:

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

**Windows:**
1. Download and run [rustup-init.exe](https://win.rustup.rs/)
2. Select "1) Proceed with installation"
3. Add to PATH when prompted

Verify installation:
```bash
rustc --version
cargo --version
```

### Version Requirements
- Rust 1.80+
- Cargo (shipped with Rust)

### Git
Install Git from [git-scm.com](https://git-scm.com/)

### IDE Configuration (VSCodium Recommended)
Install these essential extensions:
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - Real-time compiler feedback
- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens) - Inline error highlighting
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) - Debugging support
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) - Cargo.toml support
- [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) - Dependency management

## Recommended Tools

Enhance your development experience with these cargo extensions:

```bash
# Install all recommended tools
cargo install cargo-watch cargo-audit cargo-deny
```

- **cargo-watch**: Automatic rebuild on file changes
  ```bash
  cargo watch -x run
  ```

- **cargo-audit**: Security vulnerability scanning
  ```bash
  cargo audit
  ```

- **cargo-deny**: License compliance checks
  ```bash
  cargo deny check licenses
  ```

## Initial Setup

### 1. Clone the Repository

```bash
git clone https://github.com/Ruthin-School/Platter.git
cd Platter
```

### 2. Install Dependencies

```bash
cargo build
```

This command will download and compile all required dependencies.

### 3. Directory Permissions

The application needs write access to the `data/` directory to store its data files. Ensure proper permissions are set:

On Linux/macOS:
```bash
chmod 755 data/
```

On Windows (PowerShell):
```powershell
icacls "data" /grant "$($env:USERNAME):(F)" /T
```

### 4. Run the Application

```bash
cargo run
```

The application will start on `http://localhost:8080`.

## Development Workflow

### Terminal Configuration Tips

Improve your terminal experience with these tips:

1. **Shell Aliases** (add to ~/.bashrc or ~/.zshrc):
```bash
alias cr='cargo run'
alias ct='cargo test'
alias cc='cargo check'
alias cw='cargo watch -x run'
```

2. **Prompt Customization**:
Show Rust version in your prompt:
```bash
# Add to shell config
export PS1="\[\e[32m\]\u@\h \[\e[33m\]\w \[\e[31m\](\$(rustc --version | cut -d' ' -f2))\[\e[0m\] $ "
```

3. **Terminal Multiplexers**:
Use tmux/screen for session management:
```bash
# Create named session
tmux new -s platter-dev
```

### Local Development

For active development, use cargo watch for automatic rebuilds:

```bash
# Install cargo watch if not already installed
cargo install cargo-watch

# Run with automatic rebuild
cargo watch -x run
```

### Feature Development

1. Create a feature branch: `git checkout -b feature/your-feature-name`
2. Make your changes
3. Add tests if applicable
4. Run tests: `cargo test`
5. Format your code: `cargo fmt`
6. Check for errors: `cargo check`
7. Commit your changes with a descriptive message
8. Push and create a pull request

## Project Structure

```
Platter/
├── Cargo.toml
├── README.md
├── API.md
├── DESIGN.md
├── DEVELOPMENT.md
├── SECURITY.md
├── DEPLOYMENT.md
├── LICENSE
├── src/
│   ├── main.rs
│   ├── auth.rs
│   ├── error_handler.rs
│   ├── handlers.rs
│   ├── storage.rs
│   └── scheduler.rs
├── data/
│   ├── admin_users.json
│   ├── menu_items.json
│   ├── menu_presets.json
│   ├── menu_schedules.json
│   └── notices.json
├── templates/
│   ├── base.html
│   ├── menu.html
│   ├── item_detail.html
│   ├── admin/
│   │   ├── dashboard.html
│   │   ├── login.html
│   │   ├── presets.html
│   │   └── schedules.html
│   └── partials/
│       ├── footer.html
│       └── header.html
├── static/
│   ├── css/
│   │   └── style.css
│   └── images/
│       ├── logo-white-no-text.svg
│       └── logo-white-text.png
├── assets/
│   └── branding/
│       ├── logo-coloured-text.svg
│       ├── logo-white-no-text.svg
│       └── logo-white-text.png
└── target/
```

## Code Style

### Rust Style Guidelines

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` to format code before committing
- Use `cargo clippy` for linting and best practices
- Write documentation for public functions and modules

### Commit Messages

Use the conventional commit format:
```
type(scope): description

More detailed explanation if needed
```

Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
- `feat(auth): add password reset functionality`
- `fix(storage): resolve data corruption issue`
- `docs: update API documentation`

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_function_name

# Run tests with coverage (if using cargo-tarpaulin)
cargo tarpaulin
```

### Writing Tests

- Place unit tests in the same file as the code they test
- Use `#[cfg(test)]` for test modules
- Write integration tests in the `tests/` directory
- Follow the Given-When-Then pattern for test organization

## Building

### Development Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

The optimized binary will be located at `target/release/platter`.

### Cross-compilation

To build for different platforms:

```bash
# Install target (example: Linux to Windows)
rustup target add x86_64-pc-windows-gnu

# Build for Windows
cargo build --target x86_64-pc-windows-gnu --release
```

## Data Management

> **⚠️ SECURITY WARNING:** The default administrator credentials are for DEVELOPMENT USE ONLY. Always change these before deploying to production.

The application uses a JSON-based file system for data storage, located in the `data/` directory:

- `admin_users.json`: Administrator accounts and credentials
- `menu_items.json`: Menu items with pricing and descriptions
- `notices.json`: Announcements and notices
- `menu_presets.json`: Reusable menu combinations
- `menu_schedules.json`: Scheduled menu assignments

> ⚠️ **Warning**: Direct editing of these files should only be done when the application is not running to prevent data corruption.

### Data Backup

Regularly backup the `data/` directory to prevent data loss:

```bash
# Create a backup
tar -czf backup-$(date +%Y%m%d-%H%M%S).tar.gz data/
```

## Troubleshooting

### Common Issues

#### Permission Errors
If you encounter permission errors with the `data/` directory:
- On Unix systems: `chmod -R 755 data/`
- On Windows: Run the command prompt as administrator and execute the icacls command

#### Dependency Issues
If you encounter dependency issues:
```bash
# Clean the project
cargo clean

# Reinstall dependencies
cargo build
```

#### Build Errors
For build errors, try:
```bash
# Update dependencies
cargo update

# Check for errors without building
cargo check

# Build with verbose output
cargo build -v
```

### Development Tools

- **rust-analyzer**: Language server for better IDE support
- **cargo-watch**: Automatic rebuild on file changes
- **cargo-edit**: Add/remove dependencies from command line
- **cargo-outdated**: Check for outdated dependencies
- **cargo-deny**: Audit dependencies for security issues

### Useful Commands

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for errors without building
cargo check

# Update dependencies
cargo update

# Audit dependencies for security issues
cargo install cargo-deny
cargo deny check advisories
```

## Screenshot Suggestions

Capture these useful screenshots for documentation:

1. **IDE Setup**:
   - VS Code with rust-analyzer active
   - Extension manager showing installed Rust extensions

2. **Terminal Configuration**:
   - Customized prompt showing Rust version
   - Aliases in action (cw, ct, etc.)

3. **Tool Output**:
   - cargo audit results
   - cargo deny license check
   - cargo watch in operation

## Getting Help

- Check the existing documentation in this repository
- Search existing GitHub issues
- Create a new issue if you encounter a bug or need a feature
- Join discussions in pull requests
