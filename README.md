# 📋 Platter

[![CI](https://github.com/Ruthin-School/Platter/workflows/CI/badge.svg)](https://github.com/Ruthin-School/Platter/actions/workflows/ci.yml)
[![Security Audit](https://github.com/Ruthin-School/Platter/workflows/Security%20Audit/badge.svg)](https://github.com/Ruthin-School/Platter/actions/workflows/security.yml)
[![Release](https://github.com/Ruthin-School/Platter/workflows/Release/badge.svg)](https://github.com/Ruthin-School/Platter/actions/workflows/release.yml)
[![Licence](https://img.shields.io/badge/licence-AGPLv3-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80+-orange.svg)](https://www.rust-lang.org/)

**Reading Time:** Approximately 5 minutes  
**Complexity:** Beginner-friendly

## 📖 What Is Platter?

Platter is a modern, secure, and efficient web application for managing dining operations. The application is built with Rust and Actix-web. Administrators can manage menus, notices, and schedules through an intuitive interface.

### ✨ Core Features

The application provides the following capabilities:

1. **Menu Management** – Create, update, and organise menu items with ease
2. **Notice System** – Display real-time notices and announcements
3. **Scheduling** – Automate menu scheduling functionality
4. **Admin Interface** – Access comprehensive administrative controls
5. **OAuth 2.0 Authentication** – Secure login with Microsoft Entra ID integration
6. **RESTful API** – Programmatic access to all features
7. **Security First** – Built-in security measures including Argon2 password hashing and PKCE (Proof Key for Code Exchange)
8. **Accessibility Validation** – Compile-time WCAG 2.1 Level AA compliance enforcement

---

## 📑 Table of Contents

Navigate to specific sections using these links:

- [Quick Start](#-quick-start)
- [Documentation](#-documentation)
- [Key Features](#-key-features)
- [Technology Stack](#-technology-stack)
- [Project Status](#-project-status)
- [Contributing](#-contributing)
- [Licence](#-licence)

---

## 🚀 Quick Start

**Time Required:** 10 minutes  
**Prerequisite Knowledge:** Basic command-line usage

### Step 1: Check Prerequisites

Before beginning, ensure you have the following installed:

1. [Rust](https://www.rust-lang.org/) version 1.80 or higher with Cargo
2. Git version control system

### Step 2: Clone the Repository

Execute the following commands in your terminal:

```bash
# Clone the repository
git clone https://github.com/Ruthin-School/Platter.git

# Navigate into the directory
cd Platter
```

### Step 3: Build and Run

Execute the following command:

```bash
# Build and run the application (default - fastest)
cargo run
```

The system will automatically download and compile dependencies on first run. This process may take several minutes.

> 💡 **Build Optimization:** By default, optional features are disabled for faster builds. See the [Build Optimization Guide](docs/guides/build-optimization.md) to enable OAuth or accessibility validation when needed.

### Step 4: Access the Application

Open your web browser and navigate to:

```
http://localhost:8080
```

> 📘 **Note:** For detailed setup instructions, refer to the [Getting Started Guide](docs/guides/getting-started.md).

---

## 📚 Documentation

The documentation is organised into focused sections. Each section addresses specific tasks and use cases.

### 🎯 Getting Started Guides

Begin here if you are new to Platter:

- **[Quick Start Guide](docs/guides/getting-started.md)** – Installation and initial setup procedures
- **[Configuration Guide](docs/guides/configuration.md)** – Environment variables and system settings
- **[Build Optimization Guide](docs/guides/build-optimization.md)** – Reduce compile times with feature flags
- **[OAuth Setup](docs/guides/oauth-setup.md)** – Microsoft Entra ID authentication configuration
- **[Accessibility Guide](ACCESSIBILITY.md)** – WCAG 2.1 Level AA compliance validation system

### 🔧 Development Resources

Use these guides if you are contributing to the project:

- **[Development Setup](docs/development/setup.md)** – Configure your development environment
- **[Build Optimization Guide](docs/guides/build-optimization.md)** – Faster compilation with feature flags
- **[Contributing Guidelines](docs/development/contributing.md)** – Contribution procedures and standards
- **[Testing Guide](docs/development/testing.md)** – Testing practices and methodologies

### 📡 API Reference

Access complete REST API documentation:

- **[API Documentation](docs/api/reference.md)** – Complete REST API reference with endpoints and examples

### 🚢 Deployment Guides

Deploy Platter to production environments:

- **[Production Deployment](docs/deployment/production.md)** – Production deployment procedures
- **[Docker Deployment](docs/deployment/docker.md)** – Containerised deployment instructions

### 🏗️ Architecture Documentation

Understand the system design:

- **[Design Documentation](docs/architecture/design.md)** – System architecture and technical design
- **[Security Documentation](docs/architecture/security.md)** – Security features and best practices
- **[Accessibility System](docs/accessibility/README.md)** – Compile-time accessibility validation

### 🔍 Troubleshooting Resources

Resolve common issues:

- **[OAuth Troubleshooting](docs/troubleshooting/oauth.md)** – Authentication issue resolution

### 📖 Complete Documentation Index

Browse all documentation sections: **[Documentation Index](docs/README.md)**

---

## 🔐 Key Features

### Security Capabilities

The application implements industry-standard security measures:

1. **Argon2 Password Hashing** – Industry-standard password security algorithm
2. **OAuth 2.0 with PKCE** – Enhanced security for authentication flows
3. **Secure Session Management** – Encrypted sessions with HttpOnly cookies
4. **CSRF Protection** – Token-based Cross-Site Request Forgery protection
5. **Rate Limiting** – Protection against brute force attacks

> 📘 **Reference:** Complete security details are available in the [Security Documentation](docs/architecture/security.md).

### Menu Management Features

Manage dining menus efficiently:

1. Create and organise menu items with categories and tags
2. Manage real-time availability status
3. Configure reusable menu presets
4. Implement automated scheduling systems

### Administrative Features

Access comprehensive administrative tools:

1. Administrative dashboard for system management
2. Notice and announcement system
3. User access control mechanisms
4. OAuth email whitelist configuration

### API Access Capabilities

Programmatic access to system features:

- Complete RESTful API for all functionality
- Detailed documentation with examples
- Authentication and authorisation support

> 📘 **Reference:** API details are available in the [API Documentation](docs/api/reference.md).

---

## 🛠️ Technology Stack

### Backend Technologies

| Technology | Purpose | Version |
|-----------|---------|---------|
| [Rust](https://www.rust-lang.org/) | Programming language | 1.80+ |
| [Actix-web](https://actix.rs/) | Web framework | 4.11 |
| [Tera](https://keats.github.io/tera/) | Template engine | 1.20 |
| [Tokio](https://tokio.rs/) | Async runtime | Latest |
| [Serde](https://serde.rs/) | JSON serialisation | Latest |

### Authentication Technologies

1. Argon2 password hashing algorithm
2. OAuth 2.0 and OIDC (OpenID Connect) protocols

> 📘 **Reference:** Complete architecture details are available in the [Design Documentation](docs/architecture/design.md).

---

## 📊 Project Status

### Version Information

Current version details and update history are maintained in the [CHANGELOG.md](CHANGELOG.md) file.

### Default Development Credentials

> ⚠️ **Security Warning:** These credentials are for development environments ONLY. You MUST change these before production deployment.

**Default Admin Credentials:**
- Username: `admin`
- Password: `admin123`

**Production Security Requirements:**

Before deploying to production, complete these mandatory steps:

1. Change default admin credentials
2. Generate a strong SESSION_SECRET
3. Configure CORS_ALLOWED_ORIGINS
4. Enable PRODUCTION mode
5. Configure HTTPS

> 📘 **Reference:** Complete production security procedures are detailed in the [Security Best Practices](docs/architecture/security.md#best-practices) section.

---

## 🤝 Contributing

Contributions to this project are welcome. Follow these steps to contribute.

### Quick Contribution Steps

Execute these steps in sequence:

1. **Fork the repository** – Create your own copy
2. **Create a feature branch** – Use command: `git checkout -b feature/amazing-feature`
3. **Make your changes** – Follow the [Code Style Guide](docs/development/contributing.md#code-style)
4. **Run tests** – Execute command: `cargo test`
5. **Commit your changes** – Use command: `git commit -m 'feat: add amazing feature'`
6. **Push to the branch** – Execute command: `git push origin feature/amazing-feature`
7. **Open a Pull Request** – Submit your changes for review

### Development Resources

Access these resources for detailed guidance:

- [Development Setup](docs/development/setup.md) – Configure your development environment
- [Testing Guide](docs/development/testing.md) – Execute and create tests
- [API Reference](docs/api/reference.md) – Understand API structure
- [Contributing Guidelines](docs/development/contributing.md) – Complete contribution procedures

> 📘 **Full Details:** Complete contributing guidelines are available in the [Contributing Documentation](docs/development/contributing.md).

---

## 📄 Licence

This project is licenced under the AGPLv3 Licence. Complete licence terms are available in the [LICENCE](LICENSE) file.

---

## 🙏 Acknowledgements

This project is built using:

- [Rust programming language](https://www.rust-lang.org/)
- [Actix-web framework](https://actix.rs/)
- Modern web technologies for fast and secure user experience

---

## 🔗 Additional Resources

### Project Resources

- **Documentation Hub** – [Complete Documentation](docs/README.md)
- **Issue Tracker** – [GitHub Issues](https://github.com/Ruthin-School/Platter/issues)
- **Change Log** – [Version History](CHANGELOG.md)
- **Licence Terms** – [AGPLv3 Licence](LICENSE)

### Getting Help

If you encounter issues or need assistance:

1. **First Step** – Check the [Troubleshooting Guide](docs/troubleshooting/README.md)
2. **Second Step** – Search existing [GitHub Issues](https://github.com/Ruthin-School/Platter/issues)
3. **Third Step** – [Open a new issue](https://github.com/Ruthin-School/Platter/issues) with a detailed description

---

**Project maintained by:** Ruthin School  
**Last updated:** 2025
