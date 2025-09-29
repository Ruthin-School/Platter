# Deployment Guide

This guide provides instructions for deploying the Dining Hall Dashboard to production environments.

## 📋 Table of Contents

- [Prerequisites](#prerequisites)
- [Environment Configuration](#environment-configuration)
- [Building for Production](#building-for-production)
- [Docker Deployment](#docker-deployment)
- [Systemd Service (Linux)](#systemd-service-linux)
- [Reverse Proxy Setup](#reverse-proxy-setup)
- [Security Best Practices](#security-best-practices)
- [Monitoring and Maintenance](#monitoring-and-maintenance)
- [Backup and Recovery](#backup-and-recovery)
- [Troubleshooting](#troubleshooting)

## Prerequisites

Before deploying, ensure your target system has:

- A compatible operating system (Linux, Windows, or macOS)
- At least 256MB of RAM
- Sufficient disk space for the application and data
- Network connectivity
- A domain name pointing to your server (recommended)

## Environment Configuration

### Required Environment Variables

| Variable         | Description | Example |
|------------------|-------------|---------|
| `SESSION_SECRET` | 64-byte secret key for session encryption (required in production) | `$(openssl rand -base64 64 \| tr -d '\n')` |
| `PRODUCTION`     | Enable production mode with enhanced security | `true` |

### Recommended Environment Variables

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `PORT` | Port to run the application on | `8080` | `3000` |
| `HOST` | Host address to bind to | `0.0.0.0` | `127.0.0.1` |
| `RUST_LOG` | Logging level | `info` | `warn` |
| `CORS_ALLOWED_ORIGINS` | Comma-separated allowed origins | `http://localhost:8080,http://127.0.0.1:8080` | `https://yourdomain.com,https://admin.yourdomain.com` |

> ⚠️ **Important**: Always use a strong, randomly generated `SESSION_SECRET` in production. Never commit this to version control.

## Building for Production

### Method 1: Build on Target System

On your production server:

```bash
# Clone the repository
git clone https://github.com/Ruthin-School/Platter.git
cd Platter

# Build the optimized binary
cargo build --release

# The optimized binary will be located at:
# target/release/platter
```

### Method 2: Cross-compilation

Build on your development machine for the target platform:

```bash
# Add the target (example: x86_64 Linux)
rustup target add x86_64-unknown-linux-musl

# Build for the target
cargo build --target x86_64-unknown-linux-musl --release

# The binary will be in:
# target/x86_64-unknown-linux-musl/release/platter
```

## Docker Deployment

### Using Pre-built Images

The application can be containerized using Docker:

```Dockerfile
FROM rust:1.80 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/platter .
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/static ./static
RUN mkdir -p data
EXPOSE 8080
CMD ["./platter"]
```

### Building and Running with Docker

```bash
# Build the image
docker build -t platter .

# Run with environment variables
docker run -d \
  --name dining-hall-app \
  -p 8080:8080 \
  -e PRODUCTION=true \
  -e SESSION_SECRET="$(openssl rand -base64 64)" \
  -v dining-data:/app/data \
  platter
```

### Docker Compose

Create a `docker-compose.yml` file:

```yaml
version: '3.8'

services:
  dining-hall:
    build: .
    ports:
      - "8080:8080"
    environment:
      - PRODUCTION=true
      - SESSION_SECRET=${SESSION_SECRET}
      - CORS_ALLOWED_ORIGINS=https://yourdomain.com
    volumes:
      - dining_data:/app/data
    restart: unless-stopped

volumes:
  dining_data:
```

Run with:
```bash
SESSION_SECRET=$(openssl rand -base64 64) docker-compose up -d
```

## Systemd Service (Linux)

Create a systemd service file at `/etc/systemd/system/dining-hall.service`:

```ini
[Unit]
Description=Dining Hall Dashboard
After=network.target

[Service]
Type=simple
User=appuser
WorkingDirectory=/opt/dining-hall
ExecStart=/opt/dining-hall/platter
Restart=always
RestartSec=10

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ReadWritePaths=/opt/dining-hall/data

# Environment variables
Environment=PRODUCTION=true
Environment=PORT=8080
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

Set up the service:

```bash
# Create app user
sudo useradd -r -s /bin/false appuser

# Create application directory
sudo mkdir -p /opt/dining-hall
sudo chown appuser:appuser /opt/dining-hall

# Copy application files
sudo cp target/release/platter /opt/dining-hall/
sudo cp -r templates static /opt/dining-hall/
sudo chown -R appuser:appuser /opt/dining-hall

# Copy the service file
sudo cp dining-hall.service /etc/systemd/system/

# Enable and start the service
sudo systemctl daemon-reload
sudo systemctl enable dining-hall
sudo systemctl start dining-hall

# Check status
sudo systemctl status dining-hall
```

## Reverse Proxy Setup

### Nginx Configuration

Add this to your Nginx configuration:

```nginx
server {
    listen 80;
    server_name yourdomain.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name yourdomain.com;

    ssl_certificate /path/to/your/certificate.crt;
    ssl_certificate_key /path/to/your/private.key;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Port $server_port;

        # Session and security headers
        proxy_cookie_path / "/; secure; HttpOnly; SameSite=Lax";
    }
}
```

### Apache Configuration

```apache
<VirtualHost *:443>
    ServerName yourdomain.com

    SSLEngine on
    SSLCertificateFile /path/to/your/certificate.crt
    SSLCertificateKeyFile /path/to/your/private.key

    ProxyPreserveHost On
    ProxyPass / http://127.0.0.1:8080/
    ProxyPassReverse / http://127.0.0.1:8080/

    # Headers for security
    Header edit Set-Cookie ^(.*)$ $1;HttpOnly;Secure
</VirtualHost>
```

## Security Best Practices

### 1. HTTPS Configuration
- Always use HTTPS in production
- Obtain SSL/TLS certificates (e.g., from Let's Encrypt)
- Configure proper security headers

### 2. Environmental Security
- Use strong `SESSION_SECRET` (minimum 32 bytes)
- Restrict `CORS_ALLOWED_ORIGINS` to trusted domains only
- Set `PRODUCTION=true` to enable production security settings

### 3. System Security
- Run the application as a non-privileged user
- Limit file system permissions
- Use network-level protection (firewalls, etc.)

### 4. Data Protection
- Regularly backup the `data/` directory
- Encrypt sensitive data at rest if required
- Implement proper access controls

## Monitoring and Maintenance

### Log Management
The application logs to standard output and standard error. Configure log rotation:

```bash
# Example logrotate configuration for systemd service
# /etc/logrotate.d/dining-hall
/var/log/journal/*dining-hall*.journal {
    daily
    rotate 14
    compress
    delaycompress
    missingok
    notifempty
    copytruncate
}
```

### Health Checks
Implement health checks in your load balancer or monitoring system:

```bash
# Simple health check
curl -f http://localhost:8080/ || exit 1
```

### Update Process
1. Download new version
2. Stop the service
3. Backup current data
4. Deploy new binary
5. Start the service
6. Verify functionality

## Backup and Recovery

### Regular Backups
Create a backup script:

```bash
#!/bin/bash
# backup.sh
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/backup/dining-hall"
APP_DATA="/opt/dining-hall/data"

mkdir -p "$BACKUP_DIR"
tar -czf "$BACKUP_DIR/backup_$DATE.tar.gz" -C "$(dirname "$APP_DATA")" "$(basename "$APP_DATA")"

# Keep only last 7 days of backups
find "$BACKUP_DIR" -type f -name "backup_*" -mtime +7 -delete
```

### Automated Backups
Add to crontab:
```bash
# Daily backup at 2 AM
0 2 * * * /path/to/backup.sh
```

## Troubleshooting

### Common Issues

#### Application Won't Start
Check if required environment variables are set:
```bash
env | grep -E "(SESSION_SECRET|PRODUCTION)"
```

#### Permission Errors
Ensure the data directory has correct permissions:
```bash
ls -la /opt/dining-hall/data/
```

#### No Data Persistence
Verify that the data directory is mounted correctly in Docker or has proper write permissions.

### Debugging
Get logs from systemd service:
```bash
sudo journalctl -u dining-hall -f
```

### Performance Monitoring
Monitor resource usage:
```bash
# Check resource usage
top -p $(pgrep dining-hall)

# Monitor network connections
netstat -tulpn | grep :8080
```
