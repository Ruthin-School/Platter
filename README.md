# Platter

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/Ruthin-School/Platter/actions)
[![Dependencies](https://img.shields.io/badge/dependencies-up%20to%20date-brightgreen.svg)](Cargo.toml)
[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80+-orange.svg)](https://www.rust-lang.org/)

A modern, secure, and efficient web application for managing dining operations, built with Rust and Actix-web. Designed for administrators to manage menus, notices, and schedules with an intuitive interface.

## 🚀 Features

- **Menu Management**: Create, update, and organize menu items with ease
- **Notice System**: Real-time notices and announcements
- **Scheduling**: Automated menu scheduling functionality
- **Admin Interface**: Comprehensive administrative controls
- **RESTful API**: Programmatic access to all features
- **Security First**: Built-in security measures including Argon2 password hashing

## 📋 Table of Contents

- [Features](#-features)
- [Quick Start](#-quick-start)
- [Installation](#-installation)
- [Configuration](#-configuration)
- [API Documentation](#-api-documentation)
- [Security](#-security)
- [Development](#-development)
- [Deployment](#-deployment)
- [Contributing](#-contributing)
- [License](#-license)

## 💻 Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/) 1.80+ with Cargo
- Git

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Ruthin-School/Platter.git
   cd Platter
   ```

2. Build and run the application:

   ```bash
   cargo run
   ```

3. Access the application at `http://localhost:8080`

## ⚙️ Configuration

### Environment Variables

| Variable              | Description                                                                 | Default Value                    |
|-----------------------|-----------------------------------------------------------------------------|----------------------------------|
| `PORT`                | Port to run the server on                                                  | `8080`                           |
| `HOST`                | Host address to bind to                                                    | `0.0.0.0`                        |
| `RUST_LOG`            | Logging level (debug, info, warn, error)                                   | `info`                           |
| `PRODUCTION`          | Enable production mode with enhanced security                              | Not set (development mode)       |
| `SESSION_SECRET`      | Secret key for session encryption (required in production)                 | Development key (insecure)       |
| `CORS_ALLOWED_ORIGINS`| Comma-separated allowed origins when in production mode                     | `http://localhost:8080,http://127.0.0.1:8080` |

### Default Admin Account

> **⚠️ Security Warning:** Default credentials are for development only. Never use in production.

- **Username**: `admin`
- **Password**: `admin123`

## 📡 API Documentation

Complete API documentation can be found in [API.md](API.md). The application provides comprehensive RESTful endpoints for:
- Menu items management
- Notices system
- Menu presets
- Scheduling functionality

## 🔐 Security

Security is a core priority for this application. See [SECURITY.md](SECURITY.md) for detailed security measures including:
- Argon2 password hashing
- Secure session management
- CORS configuration
- Input validation

## 🛠️ Development

For detailed development setup and guidelines, see [DEVELOPMENT.md](DEVELOPMENT.md).

### Building for Production

```bash
cargo build --release
```

The optimized binary will be located at `target/release/platter`.

### Running Tests

```bash
cargo test
```

## 🚢 Deployment

For deployment instructions and best practices, see [DEPLOYMENT.md](DEPLOYMENT.md).

## 🤝 Contributing

We welcome contributions from the community! Please see our [DEVELOPMENT.md](DEVELOPMENT.md) for guidelines on:

1. Setting up your development environment
2. Code style and conventions
3. Testing practices
4. Submitting pull requests

## 📄 License

This project is licensed under the [AGPLv3 License](LICENSE). See the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [Actix-web](https://actix.rs/)
- Uses modern web technologies for a fast and secure user experience
