# 🏗️ Architecture Documentation

> **Navigation:** [Documentation Home](../README.md) → Architecture

Technical architecture, design decisions, and security documentation.

---

## 📚 Architecture Documentation

### [📐 Design Documentation](design.md)

**What this covers:** System architecture and design principles

**Topics included:**
- Architecture overview and component relationships
- Core design principles and philosophies
- Technology stack with justification for each choice
- Data management strategies and structures
- Performance optimisation techniques
- Scalability planning and future expansion options

**Reading time:** Approximately 20 minutes

---

### [🛡️ Security Documentation](security.md)

**What this covers:** Security features and implementation best practices

**Topics included:**
- Authentication and authorisation mechanisms
- Data protection strategies
- Network security configuration
- Security configuration procedures
- Production security checklist
- Vulnerability reporting process

**Reading time:** Approximately 15 minutes

---

## 🏗️ System Architecture Overview

Platter is built on a modern, secure architecture designed for performance and maintainability:

**Backend components:**
- **Programming language:** Rust (version 1.80+)
- **Web framework:** Actix-web (high-performance async framework)
- **Async runtime:** Tokio (asynchronous I/O runtime)
- **Serialisation:** Serde (efficient JSON processing)

**Frontend components:**
- **Rendering:** Server-side rendering with Tera template engine
- **Markup:** HTML5 (Hypertext Markup Language version 5) with semantic structure
- **Styling:** CSS3 (Cascading Style Sheets version 3) with responsive design
- **Scripting:** Vanilla JavaScript (progressive enhancement only)

**Data storage:**
- **Format:** JSON (JavaScript Object Notation) file-based storage
- **Location:** Local file system in `data/` directory
- **Benefits:** No external database required, easy backup and version control

**Authentication:**
- **Session management:** Encrypted session cookies with secure flags
- **Password security:** Argon2 hashing algorithm

---

## 🔐 Security Features

### Authentication and Authorisation

**Implemented security measures:**
- **Argon2 password hashing** – Memory-hard algorithm resistant to attacks
- **Secure session management** – Encrypted cookies with HttpOnly and Secure flags
- **Role-based access control** – Administrator and user permission levels

### Data Protection

**Security layers:**
- **Input validation** – Server-side validation of all user inputs
- **Output encoding** – Automatic escaping to prevent XSS (Cross-Site Scripting)
- **CSRF protection** – Token-based protection against cross-site request forgery

### Network Security

**Network-level protection:**
- **CORS configuration** – Configurable cross-origin resource sharing policies
- **Security headers** – HTTP headers for additional protection
- **HTTPS enforcement** – SSL/TLS encryption in production mode

---

## 🎯 Quick Navigation Links

**Core documentation:**
- [Main README](../../README.md) – Project overview
- [Getting Started Guide](../guides/getting-started.md) – Installation and setup
- [Development Guide](../development/README.md) – Developer resources
- [API Reference](../api/reference.md) – API documentation

---

## 🔍 Architecture Topics

Find detailed information about specific architecture aspects:

- [System architecture overview](design.md#architecture-overview) – High-level system design
- [Technology stack details](design.md#technology-stack) – Technologies and rationale
- [Security architecture](security.md#security-features) – Security implementation
- [Data model structures](design.md#data-management) – Data organisation
- [Performance optimisation](design.md#performance-considerations) – Speed and efficiency

---

## 📖 Related Documentation

Access additional technical resources:

- **[Development Setup Guide](../development/setup.md)** – Technical environment setup
- **[API Reference](../api/reference.md)** – API design and endpoints
- **[Deployment Guide](../deployment/README.md)** – Production architecture considerations

---

[← Back to Documentation Home](../README.md)