# 🚀 Production Deployment Guide

> **Navigation:** [Documentation Home](../README.md) → [Deployment](README.md) → Production

**Reading Time:** Approximately 25 minutes  
**Complexity:** Intermediate to Advanced  
**Prerequisites:** System administration knowledge and understanding of web deployment concepts

## 📖 What This Guide Covers

This guide provides comprehensive instructions for deploying Platter to production environments. By the end of this guide, you will understand:

1. System prerequisites and resource requirements
2. How to configure production environment variables
3. Multiple deployment methods (native, Docker, systemd)
4. Reverse proxy configuration for HTTPS
5. Security best practices for production
6. Monitoring, maintenance, and backup strategies
7. Common troubleshooting scenarios

---

## 📋 Section 1: Prerequisites

### 1.1: System Requirements

Your target production system must have:

| Requirement | Specification | Purpose |
|-------------|--------------|---------|
| **Operating System** | Linux (recommended), Windows, or macOS | Application runtime environment |
| **RAM (Random Access Memory)** | Minimum 256 MB, recommended 512 MB | Application memory requirements |
| **Disk Space** | Minimum 500 MB for application and data | Storage for binary and data files |
| **Network** | Stable internet connection | API access and updates |
| **Domain Name** | Recommended for production | Professional access and SSL/TLS certificates |

### 1.2: Network Requirements

**Required ports:**

| Port | Protocol | Purpose | Access Level |
|------|----------|---------|-------------|
| 8080 | TCP | Application HTTP port | Internal (behind reverse proxy) |
| 80 | TCP | HTTP traffic | Public (redirects to HTTPS) |
| 443 | TCP | HTTPS traffic | Public (primary access) |

**Firewall configuration:**
- Allow inbound traffic on ports 80 and 443
- Block direct access to port 8080 (use reverse proxy)
- Allow outbound traffic for dependency updates

---

## ⚙️ Section 2: Environment Configuration

### 2.1: Required Environment Variables

These variables MUST be set for production deployment:

**Variable 2.1.1: SESSION_SECRET**

```bash
# Generate secure session secret
SESSION_SECRET=$(openssl rand -base64 64 | tr -d '\n')
```

**Purpose:** Encrypts session data to protect user authentication.

**Requirements:**
- Minimum 32 bytes (64 base64 characters recommended)
- Cryptographically secure random value
- Never committed to version control

**Variable 2.1.2: PRODUCTION**

```bash
PRODUCTION=true
```

**Purpose:** Enables production security settings.

**What this enables:**
- Strict CORS (Cross-Origin Resource Sharing) policies
- Secure cookie flags (HTTPS-only)
- Enhanced security headers
- Production logging levels

### 2.2: Recommended Environment Variables

These variables should be configured for optimal production operation:

| Variable | Default | Production Example | Purpose |
|----------|---------|-------------------|---------|
| `PORT` | `8080` | `8080` | Application listen port |
| `HOST` | `0.0.0.0` | `127.0.0.1` | Bind address (use localhost behind proxy) |
| `RUST_LOG` | `info` | `warn` | Logging verbosity level |
| `CORS_ALLOWED_ORIGINS` | `http://localhost:8080` | `https://yourdomain.com` | Comma-separated allowed origins |

### 2.3: Environment File Configuration

**Step 2.3.1: Create Environment File**

Create `.env` file (never commit to Git):

```bash
# Production environment configuration
PRODUCTION=true
SESSION_SECRET=your-generated-secret-here
PORT=8080
HOST=127.0.0.1
RUST_LOG=warn
CORS_ALLOWED_ORIGINS=https://yourdomain.com,https://admin.yourdomain.com
```

**Step 2.3.2: Load Environment Variables**

```bash
# Load from .env file
export $(grep -v '^#' .env | xargs)

# Verify variables are set
echo $PRODUCTION
echo $SESSION_SECRET
```

> ⚠️ **Critical Security Warning:** Never commit `SESSION_SECRET` to version control. Use environment files excluded from Git or secret management services.

---

## 🔨 Section 3: Building for Production

### 3.1: Method A – Build on Target System

Execute these commands on your production server:

**Step 3.1.1: Clone Repository**

```bash
# Clone the repository
git clone https://github.com/Ruthin-School/Platter.git
cd Platter
```

**Step 3.1.2: Build Optimised Binary**

```bash
# Build with release optimisations
cargo build --release
```

**What happens during release build:**
1. Code compiled with maximum optimisations
2. Debug symbols removed
3. Dead code eliminated
4. Result: Smaller, faster binary

**Step 3.1.3: Locate Binary**

```bash
# Binary location
ls -lh target/release/platter
```

**Expected output:** Binary approximately 5-15 MB depending on features.

### 3.2: Method B – Cross-Compilation

Build on development machine for target platform:

**Step 3.2.1: Add Target Platform**

```bash
# Example: Build for Linux from any platform
rustup target add x86_64-unknown-linux-musl
```

**Common targets:**
- `x86_64-unknown-linux-musl` – Linux 64-bit (static binary)
- `x86_64-pc-windows-gnu` – Windows 64-bit
- `aarch64-unknown-linux-musl` – ARM64 Linux (Raspberry Pi)

**Step 3.2.2: Cross-Compile**

```bash
# Build for target platform
cargo build --target x86_64-unknown-linux-musl --release
```

**Step 3.2.3: Transfer Binary**

```bash
# Copy to production server
scp target/x86_64-unknown-linux-musl/release/platter user@server:/opt/platter/
```

---

## 🐳 Section 4: Docker Deployment

### 4.1: Using Docker Container

**Step 4.1.1: Build Docker Image**

```bash
# Build the Docker image
docker build -t platter:production .
```

**Step 4.1.2: Run Container**

```bash
# Run with environment variables
docker run -d \
  --name platter-app \
  -p 8080:8080 \
  -e PRODUCTION=true \
  -e SESSION_SECRET="$(openssl rand -base64 64)" \
  -e CORS_ALLOWED_ORIGINS="https://yourdomain.com" \
  -v platter-data:/app/data \
  platter:production
```

**Parameter explanation:**
- `-d`: Run in detached mode (background)
- `--name`: Container name for easy reference
- `-p 8080:8080`: Map port 8080 to host
- `-e`: Set environment variables
- `-v`: Mount volume for data persistence

**Step 4.1.3: Verify Deployment**

```bash
# Check container status
docker ps

# View logs
docker logs platter-app

# Test application
curl http://localhost:8080
```

### 4.2: Docker Compose Deployment

**Step 4.2.1: Create docker-compose.yml**

```yaml
version: '3.8'

services:
  platter:
    build: .
    image: platter:production
    container_name: platter-production
    ports:
      - "8080:8080"
    environment:
      - PRODUCTION=true
      - SESSION_SECRET=${SESSION_SECRET}
      - CORS_ALLOWED_ORIGINS=${CORS_ALLOWED_ORIGINS:-https://yourdomain.com}
      - RUST_LOG=${RUST_LOG:-warn}
    volumes:
      - platter-data:/app/data
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

volumes:
  platter-data:
    driver: local
```

**Step 4.2.2: Deploy with Compose**

```bash
# Generate session secret
echo "SESSION_SECRET=$(openssl rand -base64 64 | tr -d '\n')" > .env

# Add production variables
echo "PRODUCTION=true" >> .env
echo "CORS_ALLOWED_ORIGINS=https://yourdomain.com" >> .env

# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

Detailed Docker deployment: [Docker Deployment Guide](docker.md)

---

## 🔧 Section 5: Systemd Service (Linux)

### 5.1: Service Configuration

**Step 5.1.1: Create Service File**

Create `/etc/systemd/system/platter.service`:

```ini
[Unit]
Description=Platter Menu Dashboard
After=network.target
Documentation=https://github.com/Ruthin-School/Platter

[Service]
Type=simple
User=platter
Group=platter
WorkingDirectory=/opt/platter
ExecStart=/opt/platter/platter
Restart=always
RestartSec=10

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ReadOnlyPaths=/usr /lib /lib64
ReadWritePaths=/opt/platter/data
ProtectSystem=strict
ProtectHome=true

# Environment variables
Environment=PRODUCTION=true
Environment=PORT=8080
Environment=HOST=127.0.0.1
Environment=RUST_LOG=warn

[Install]
WantedBy=multi-user.target
```

**Security directives explanation:**
- `NoNewPrivileges`: Prevents privilege escalation
- `PrivateTmp`: Provides isolated temporary directory
- `ReadOnlyPaths`: Makes system directories read-only
- `ReadWritePaths`: Allows writing only to data directory
- `ProtectSystem`: Additional system protection
- `ProtectHome`: Protects user home directories

### 5.2: Service Installation

**Step 5.2.1: Create Application User**

```bash
# Create dedicated user for application
sudo useradd -r -s /bin/false platter
```

**What this does:**
- `-r`: Creates system user (no login shell by default)
- `-s /bin/false`: Prevents interactive login
- `platter`: Username for the service

**Step 5.2.2: Create Application Directory**

```bash
# Create directory structure
sudo mkdir -p /opt/platter/{data,templates,static}

# Set ownership
sudo chown -R platter:platter /opt/platter
```

**Step 5.2.3: Deploy Application Files**

```bash
# Copy binary
sudo cp target/release/platter /opt/platter/

# Copy assets
sudo cp -r templates static /opt/platter/

# Copy data files (if migrating)
sudo cp -r data/* /opt/platter/data/

# Set permissions
sudo chown -R platter:platter /opt/platter
sudo chmod 755 /opt/platter
sudo chmod 644 /opt/platter/data/*.json
```

**Step 5.2.4: Enable and Start Service**

```bash
# Reload systemd daemon
sudo systemctl daemon-reload

# Enable service (start on boot)
sudo systemctl enable platter

# Start service now
sudo systemctl start platter

# Check status
sudo systemctl status platter
```

**Expected status output:**
```
● platter.service - Platter Menu Dashboard
     Loaded: loaded (/etc/systemd/system/platter.service; enabled)
     Active: active (running) since [timestamp]
```

### 5.3: Service Management Commands

```bash
# View service status
sudo systemctl status platter

# Restart service
sudo systemctl restart platter

# Stop service
sudo systemctl stop platter

# View logs
sudo journalctl -u platter -f

# View logs from last boot
sudo journalctl -u platter -b
```

---

## 🔒 Section 6: Reverse Proxy Setup

### 6.1: Nginx Configuration

**Purpose:** Nginx handles HTTPS termination and forwards requests to the application.

**Step 6.1.1: Install Nginx**

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install nginx

# CentOS/RHEL
sudo yum install nginx
```

**Step 6.1.2: Create Nginx Configuration**

Create `/etc/nginx/sites-available/platter`:

```nginx
# HTTP to HTTPS redirect
server {
    listen 80;
    listen [::]:80;
    server_name yourdomain.com;
    
    # Redirect all HTTP to HTTPS
    return 301 https://$server_name$request_uri;
}

# HTTPS server
server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name yourdomain.com;

    # SSL certificate configuration
    ssl_certificate /etc/letsencrypt/live/yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/yourdomain.com/privkey.pem;
    
    # SSL security settings
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    # Proxy to application
    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Port $server_port;

        # Session cookie security
        proxy_cookie_path / "/; secure; HttpOnly; SameSite=Strict";
    }

    # Static file caching
    location /static/ {
        proxy_pass http://127.0.0.1:8080/static/;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

**Step 6.1.3: Enable Configuration**

```bash
# Create symbolic link
sudo ln -s /etc/nginx/sites-available/platter /etc/nginx/sites-enabled/

# Test configuration
sudo nginx -t

# Reload Nginx
sudo systemctl reload nginx
```

**Step 6.1.4: Obtain SSL/TLS Certificate**

```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx

# Obtain certificate
sudo certbot --nginx -d yourdomain.com

# Test automatic renewal
sudo certbot renew --dry-run
```

### 6.2: Apache Configuration

**Step 6.2.1: Enable Required Modules**

```bash
sudo a2enmod proxy proxy_http ssl headers
```

**Step 6.2.2: Create Virtual Host Configuration**

Create `/etc/apache2/sites-available/platter.conf`:

```apache
<VirtualHost *:80>
    ServerName yourdomain.com
    Redirect permanent / https://yourdomain.com/
</VirtualHost>

<VirtualHost *:443>
    ServerName yourdomain.com

    # SSL configuration
    SSLEngine on
    SSLCertificateFile /etc/letsencrypt/live/yourdomain.com/fullchain.pem
    SSLCertificateKeyFile /etc/letsencrypt/live/yourdomain.com/privkey.pem

    # Proxy configuration
    ProxyPreserveHost On
    ProxyPass / http://127.0.0.1:8080/
    ProxyPassReverse / http://127.0.0.1:8080/

    # Security headers
    Header always set Strict-Transport-Security "max-age=31536000; includeSubDomains"
    Header always set X-Content-Type-Options "nosniff"
    Header always set X-Frame-Options "DENY"
    
    # Cookie security
    Header edit Set-Cookie ^(.*)$ $1;HttpOnly;Secure;SameSite=Strict
</VirtualHost>
```

**Step 6.2.3: Enable Site**

```bash
# Enable site
sudo a2ensite platter

# Test configuration
sudo apache2ctl configtest

# Reload Apache
sudo systemctl reload apache2
```

---

## 🛡️ Section 7: Security Best Practices

### 7.1: HTTPS Configuration

**Why HTTPS is essential:**
1. Encrypts data in transit
2. Prevents session hijacking
3. Enables secure cookie flag
4. Required for modern browser features
5. Builds user trust

**Implementation checklist:**
- [ ] Valid SSL/TLS certificate installed
- [ ] HTTP redirects to HTTPS
- [ ] HSTS (HTTP Strict Transport Security) header enabled
- [ ] Secure cookie flags configured
- [ ] TLS 1.2+ protocols only

### 7.2: Environment Security

**Step 7.2.1: Generate Strong Session Secret**

```bash
# Generate cryptographically secure secret
openssl rand -base64 64 | tr -d '\n'
```

**Step 7.2.2: Configure CORS Properly**

```bash
# Restrict to your domains only
CORS_ALLOWED_ORIGINS=https://yourdomain.com,https://admin.yourdomain.com
```

**What to avoid:**
- ❌ Using `*` (wildcard) for CORS
- ❌ Including `http://` URLs in production
- ❌ Allowing untrusted domains

**Step 7.2.3: Set Production Mode**

```bash
# Enable production security
PRODUCTION=true
```

**What this enables:**
- Strict CORS validation
- Secure-only cookies (HTTPS required)
- Enhanced security headers
- Production-appropriate logging

### 7.3: System Security

**File system permissions:**

```bash
# Application binary: executable by owner only
chmod 750 /opt/platter/platter

# Data files: read/write by owner only
chmod 640 /opt/platter/data/*.json

# Directory: accessible by owner only
chmod 750 /opt/platter
```

**Process isolation:**

```bash
# Run as dedicated user (not root)
User=platter
Group=platter

# Limit capabilities
CapabilityBoundingSet=
```

**Network security:**

```bash
# Configure firewall (UFW example)
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw deny 8080/tcp  # Block direct access
sudo ufw enable
```

### 7.4: Data Protection

**Backup strategy:**
1. Daily automated backups of data directory
2. Store backups in separate location
3. Test restoration procedures regularly
4. Encrypt backups containing sensitive data

**Access control:**
1. Limit administrative access
2. Use strong passwords
3. Enable two-factor authentication (if available)
4. Audit access logs regularly

---

## 📊 Section 8: Monitoring and Maintenance

### 8.1: Log Management

**View application logs:**

```bash
# Systemd service logs
sudo journalctl -u platter -f

# Last 100 lines
sudo journalctl -u platter -n 100

# Logs since yesterday
sudo journalctl -u platter --since yesterday
```

**Configure log rotation:**

Create `/etc/logrotate.d/platter`:

```
/var/log/platter/*.log {
    daily
    rotate 14
    compress
    delaycompress
    missingok
    notifempty
    create 640 platter platter
}
```

### 8.2: Health Checks

**Manual health check:**

```bash
# Test application responsiveness
curl -f http://localhost:8080/ || exit 1
```

**Automated monitoring script:**

Create `/usr/local/bin/platter-healthcheck.sh`:

```bash
#!/bin/bash
ENDPOINT="http://localhost:8080/"
TIMEOUT=5

if ! curl -sf --max-time $TIMEOUT "$ENDPOINT" > /dev/null; then
    echo "Health check failed"
    systemctl restart platter
    # Send alert notification here
fi
```

**Schedule with cron:**

```bash
# Add to crontab (every 5 minutes)
*/5 * * * * /usr/local/bin/platter-healthcheck.sh
```

### 8.3: Update Process

**Step 8.3.1: Download New Version**

```bash
cd /opt/platter
git pull origin main
```

**Step 8.3.2: Stop Service**

```bash
sudo systemctl stop platter
```

**Step 8.3.3: Backup Current Data**

```bash
tar -czf backup-$(date +%Y%m%d-%H%M%S).tar.gz data/
```

**Step 8.3.4: Deploy New Binary**

```bash
cargo build --release
sudo cp target/release/platter /opt/platter/
sudo chown platter:platter /opt/platter/platter
```

**Step 8.3.5: Start Service**

```bash
sudo systemctl start platter
```

**Step 8.3.6: Verify Functionality**

```bash
sudo systemctl status platter
curl http://localhost:8080
```

---

## 💾 Section 9: Backup and Recovery

### 9.1: Automated Backup Script

Create `/usr/local/bin/platter-backup.sh`:

```bash
#!/bin/bash
# Platter backup script

DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/backup/platter"
APP_DATA="/opt/platter/data"

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Create compressed backup
tar -czf "$BACKUP_DIR/backup_$DATE.tar.gz" \
    -C "$(dirname "$APP_DATA")" \
    "$(basename "$APP_DATA")"

# Keep only last 7 days of backups
find "$BACKUP_DIR" -type f -name "backup_*" -mtime +7 -delete

# Log backup completion
echo "Backup completed: backup_$DATE.tar.gz"
```

**Make executable:**

```bash
sudo chmod +x /usr/local/bin/platter-backup.sh
```

### 9.2: Schedule Automated Backups

**Add to crontab:**

```bash
# Edit root crontab
sudo crontab -e

# Add daily backup at 2 AM
0 2 * * * /usr/local/bin/platter-backup.sh
```

### 9.3: Restoration Procedure

**Step 9.3.1: Stop Application**

```bash
sudo systemctl stop platter
```

**Step 9.3.2: Restore from Backup**

```bash
# Extract backup
tar -xzf /backup/platter/backup_20250103_020000.tar.gz \
    -C /opt/platter/

# Set permissions
sudo chown -R platter:platter /opt/platter/data
```

**Step 9.3.3: Start Application**

```bash
sudo systemctl start platter
sudo systemctl status platter
```

---

## 🔧 Section 10: Troubleshooting

### 10.1: Application Won't Start

**Symptom:** Service fails to start or immediately stops.

**Diagnostic commands:**

```bash
# Check service status
sudo systemctl status platter

# View recent logs
sudo journalctl -u platter -n 50

# Check environment variables
sudo systemctl show platter --property=Environment
```

**Common solutions:**

1. **Missing environment variables:**
   ```bash
   # Add to service file
   Environment=SESSION_SECRET=your-secret-here
   ```

2. **Permission errors:**
   ```bash
   # Fix file permissions
   sudo chown -R platter:platter /opt/platter
   ```

3. **Port already in use:**
   ```bash
   # Find process using port
   sudo lsof -i :8080
   # Stop conflicting process or change PORT
   ```

### 10.2: HTTPS Not Working

**Symptom:** Cannot access application via HTTPS.

**Diagnostic steps:**

1. **Verify certificate:**
   ```bash
   sudo certbot certificates
   ```

2. **Test Nginx configuration:**
   ```bash
   sudo nginx -t
   ```

3. **Check certificate expiry:**
   ```bash
   openssl s_client -connect yourdomain.com:443 -servername yourdomain.com
   ```

**Solutions:**

1. **Renew expired certificate:**
   ```bash
   sudo certbot renew
   sudo systemctl reload nginx
   ```

2. **Fix Nginx configuration errors:**
   ```bash
   # Review error output
   sudo nginx -t
   # Correct syntax errors
   sudo nano /etc/nginx/sites-available/platter
   ```

### 10.3: Data Not Persisting

**Symptom:** Changes disappear after restart.

**Possible causes:**
1. Docker volume not mounted correctly
2. Incorrect file permissions
3. Application writing to wrong directory

**Solutions:**

1. **Verify Docker volume:**
   ```bash
   docker volume inspect platter-data
   docker run --rm -v platter-data:/data alpine ls -la /data
   ```

2. **Check file permissions:**
   ```bash
   ls -la /opt/platter/data/
   sudo chmod 755 /opt/platter/data
   sudo chmod 644 /opt/platter/data/*.json
   ```

3. **Verify working directory:**
   ```bash
   # In systemd service
   WorkingDirectory=/opt/platter
   ```

### 10.4: Performance Issues

**Symptoms:** Slow response times or high resource usage.

**Diagnostic commands:**

```bash
# Check resource usage
top -p $(pgrep platter)

# Monitor network connections
netstat -tulpn | grep :8080

# View system metrics
htop
```

**Solutions:**

1. **Increase resources:**
   - Add more RAM if memory usage is high
   - Allocate more CPU if CPU usage is at 100%

2. **Optimise logging:**
   ```bash
   # Reduce log verbosity
   RUST_LOG=error
   ```

3. **Review proxy configuration:**
   - Enable caching for static assets
   - Configure connection pooling

---

## 🎯 Section 11: Next Steps

After completing production deployment, proceed with these tasks:

### For System Administrators

Complete these tasks in sequence:

1. **Configure Monitoring** – Set up health checks and alerting
2. **Test Backup Restoration** – Verify backup procedures work correctly
3. **Document Runbooks** – Create incident response procedures
4. **Plan Scaling** – Prepare for increased traffic if needed

### For Security Teams

Complete these tasks in sequence:

1. **Security Audit** – Review all security configurations
2. **Penetration Testing** – Test for vulnerabilities
3. **Compliance Check** – Verify regulatory compliance
4. **Incident Response Plan** – Prepare security incident procedures

---

## 📖 Related Documentation

Access these resources for additional information:

- **[Docker Deployment Guide](docker.md)** – Containerised deployment instructions
- **[Configuration Guide](../guides/configuration.md)** – Environment variables and settings reference
- **[Security Documentation](../architecture/security.md)** – Production security best practices
- **[Getting Started Guide](../guides/getting-started.md)** – Initial setup instructions
- **[Troubleshooting Guide](../troubleshooting/README.md)** – Common deployment issues and solutions

---

[← Back to Deployment](README.md) | [Documentation Home](../README.md)
