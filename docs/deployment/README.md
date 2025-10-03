# 🚀 Deployment Guide

> **Navigation:** [Documentation Home](../README.md) → Deployment

Production deployment guides for Platter.

---

## 📚 Deployment Documentation

### [🏭 Production Deployment Guide](production.md)

**What this covers:** Deploy Platter to production environments

**Topics included:**
- Server requirements and prerequisites
- Building optimised production binaries
- Systemd service configuration (Linux)
- Reverse proxy setup (Nginx and Apache)
- Monitoring and maintenance procedures
- Backup and recovery strategies

**Reading time:** Approximately 25 minutes

---

### [🐳 Docker Deployment Guide](docker.md)

**What this covers:** Containerised deployment with Docker

**Topics included:**
- Docker installation and setup
- Building Docker images
- Docker Compose configuration
- Container volume management
- Production Docker best practices
- Container monitoring and updates

**Reading time:** Approximately 20 minutes

---

## 🚀 Deployment Options

### Option 1: Standalone Server Deployment

Deploy directly on Linux, Windows, or macOS servers.

**Best for:**
- Traditional server environments
- Direct system control requirements
- Non-containerised infrastructure

**Methods:**
- Systemd service (Linux)
- Windows Service
- Direct binary execution

---

### Option 2: Docker Container Deployment

Run in isolated containers using Docker or Docker Compose.

**Best for:**
- Containerised infrastructure
- Easy scaling and updates
- Consistent deployment across environments

**Methods:**
- Docker standalone containers
- Docker Compose multi-container setups
- Container orchestration platforms

---

### Option 3: Cloud Platform Deployment

Deploy to cloud providers with container support.

**Best for:**
- Cloud-native infrastructure
- Automatic scaling requirements
- Managed service preferences

**Compatible platforms:**
- Any platform supporting Docker containers
- Platforms with Rust runtime support

---

## 🔒 Production Security Checklist

Complete these tasks before deploying to production:

**Essential security configuration:**
- [ ] **Generate secure SESSION_SECRET** using `openssl rand -base64 64`
- [ ] **Configure CORS_ALLOWED_ORIGINS** with actual domain names (no wildcards)
- [ ] **Enable HTTPS** with valid SSL/TLS (Secure Sockets Layer/Transport Layer Security) certificate
- [ ] **Change default admin credentials** from `admin`/`admin123`
- [ ] **Configure OAuth settings** with proper client credentials (if using OAuth)
- [ ] **Set PRODUCTION=true** to enable production security mode
- [ ] **Configure logging and monitoring** for security events
- [ ] **Set up automated backups** for data directory

---

## 🎯 Quick Navigation Links

**Core documentation:**
- [Main README](../../README.md) – Project overview
- [Configuration Guide](../guides/configuration.md) – Environment variables reference
- [Security Best Practices](../architecture/security.md) – Security guidelines
- [Getting Started Guide](../guides/getting-started.md) – Initial setup

---

## 🔍 Common Deployment Tasks

Find quick links to frequently performed operations:

- [Deploy with systemd](production.md#systemd-service-linux) – Linux service configuration
- [Configure Nginx reverse proxy](production.md#nginx-configuration) – HTTPS and proxy setup
- [Set up Docker deployment](docker.md#docker-deployment) – Container deployment
- [Configure backups](production.md#backup-and-recovery) – Data protection
- [Monitor application](production.md#monitoring-and-maintenance) – Health monitoring

---

## 📖 Related Documentation

Access additional resources for deployment:

- **[Security Documentation](../architecture/security.md)** – Production security considerations
- **[Configuration Guide](../guides/configuration.md)** – Environment variables and settings
- **[Troubleshooting Guide](../troubleshooting/README.md)** – Common deployment issues and solutions

---

[← Back to Documentation Home](../README.md)