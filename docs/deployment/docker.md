# 🐳 Docker Deployment Guide

> **Navigation:** [Documentation Home](../README.md) → [Deployment](README.md) → Docker

**Reading Time:** Approximately 20 minutes  
**Complexity:** Intermediate  
**Prerequisites:** Understanding of Docker containers and basic container concepts

## 📖 What This Guide Covers

This guide provides comprehensive instructions for deploying Platter using Docker and Docker Compose. By the end of this guide, you will understand:

1. Docker prerequisites and installation
2. How to build and run Docker containers
3. Docker Compose configuration and deployment
4. Data persistence strategies with volumes
5. Production best practices for Docker
6. Container monitoring and maintenance

---

## 📋 Section 1: Prerequisites

### 1.1: Required Software

**Docker Engine:** Version 20.10 or higher

**Docker Compose:** Version 2.0 or higher (optional, for compose deployment)

**System requirements:**
- 512 MB RAM (Random Access Memory) minimum
- 1 GB disk space for images and containers
- Linux, macOS, or Windows operating system

### 1.2: Docker Installation

#### Step 1.2.1: Install Docker on Linux

```bash
# Download and run official installation script
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add current user to docker group (optional)
sudo usermod -aG docker $USER

# Log out and back in for group changes to take effect
```

**What this does:**
- Downloads Docker installation script
- Installs Docker Engine and CLI (Command-Line Interface)
- Adds user to docker group for non-root access

#### Step 1.2.2: Install Docker on macOS or Windows

1. Download [Docker Desktop](https://www.docker.com/products/docker-desktop)
2. Run the installer
3. Follow the installation wizard
4. Start Docker Desktop application

#### Step 1.2.3: Verify Docker Installation

```bash
# Check Docker version
docker --version

# Verify Docker is running
docker ps

# Test Docker installation
docker run hello-world
```

**Expected output:** Version information and successful test container execution.

---

## 🐳 Section 2: Basic Docker Deployment

### 2.1: Dockerfile Configuration

**Step 2.1.1: Create Dockerfile**

Create `Dockerfile` in project root:

```dockerfile
# Build stage
FROM rust:1.80 as builder

WORKDIR /app

# Copy project files
COPY . .

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary and assets from build stage
COPY --from=builder /app/target/release/platter .
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/static ./static

# Create data directory
RUN mkdir -p data

# Expose application port
EXPOSE 8080

# Run application
CMD ["./platter"]
```

**Dockerfile explanation:**

**Build stage:**
- `FROM rust:1.80 as builder`: Uses Rust image for compilation
- `WORKDIR /app`: Sets working directory
- `COPY . .`: Copies source code
- `RUN cargo build --release`: Builds optimised binary

**Runtime stage:**
- `FROM debian:bullseye-slim`: Uses minimal Debian image
- `RUN apt-get ...`: Installs required system libraries
- `COPY --from=builder ...`: Copies built binary and assets
- `EXPOSE 8080`: Documents port usage
- `CMD ["./platter"]`: Defines container startup command

### 2.2: Build Docker Image

**Step 2.2.1: Build Image**

```bash
# Build image with tag
docker build -t platter:latest .
```

**What happens during build:**
1. Docker reads Dockerfile instructions
2. Downloads base images
3. Compiles application in build stage
4. Creates minimal runtime image
5. Tags final image as `platter:latest`

**Build process duration:** 5-15 minutes on first build (faster subsequent builds due to caching)

**Step 2.2.2: Verify Image**

```bash
# List Docker images
docker images platter

# View image details
docker inspect platter:latest
```

### 2.3: Run Docker Container

**Step 2.3.1: Basic Container Run**

```bash
# Run container with basic configuration
docker run -d \
  --name platter \
  -p 8080:8080 \
  platter:latest
```

**Parameter explanation:**
- `-d`: Detached mode (runs in background)
- `--name platter`: Names container "platter"
- `-p 8080:8080`: Maps host port 8080 to container port 8080
- `platter:latest`: Image to run

**Step 2.3.2: Run with Environment Variables**

```bash
# Run with production configuration
docker run -d \
  --name platter \
  -p 8080:8080 \
  -e PRODUCTION=true \
  -e SESSION_SECRET="$(openssl rand -base64 64)" \
  -e CORS_ALLOWED_ORIGINS="https://yourdomain.com" \
  -v platter-data:/app/data \
  platter:latest
```

**Additional parameters:**
- `-e`: Sets environment variables
- `-v platter-data:/app/data`: Mounts named volume for data persistence

**Step 2.3.3: Verify Deployment**

```bash
# Check container status
docker ps

# View container logs
docker logs platter

# Follow logs in real-time
docker logs -f platter

# Test application
curl http://localhost:8080
```

**Expected log output:**
```
Server running at http://0.0.0.0:8080
```

---

## 🎼 Section 3: Docker Compose Deployment

### 3.1: Basic Docker Compose Configuration

**Step 3.1.1: Create docker-compose.yml**

Create `docker-compose.yml` in project root:

```yaml
version: '3.8'

services:
  platter:
    build: .
    image: platter:latest
    container_name: platter
    ports:
      - "8080:8080"
    environment:
      - PRODUCTION=true
      - SESSION_SECRET=${SESSION_SECRET}
      - CORS_ALLOWED_ORIGINS=${CORS_ALLOWED_ORIGINS:-https://yourdomain.com}
      - RUST_LOG=${RUST_LOG:-info}
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

**Configuration explanation:**

**services.platter:**
- `build: .`: Builds image from current directory
- `ports`: Maps ports between host and container
- `environment`: Sets environment variables (uses `.env` file)
- `volumes`: Mounts data volume
- `restart: unless-stopped`: Automatic restart policy
- `healthcheck`: Monitors container health

**volumes:**
- `platter-data`: Named volume for persistent data storage

### 3.2: Environment File Setup

**Step 3.2.1: Create .env File**

Create `.env` file (exclude from Git):

```bash
# Production configuration
PRODUCTION=true
SESSION_SECRET=your-generated-secret-here
CORS_ALLOWED_ORIGINS=https://yourdomain.com,https://admin.yourdomain.com
RUST_LOG=info
PORT=8080
```

**Step 3.2.2: Generate Secure Session Secret**

```bash
# Generate and save to .env file
echo "SESSION_SECRET=$(openssl rand -base64 64 | tr -d '\n')" > .env

# Add other required variables
echo "PRODUCTION=true" >> .env
echo "CORS_ALLOWED_ORIGINS=https://yourdomain.com" >> .env
```

> ⚠️ **Security Warning:** Never commit `.env` files to version control. Add `.env` to `.gitignore`.

### 3.3: Deploy with Docker Compose

**Step 3.3.1: Start Services**

```bash
# Start all services in background
docker-compose up -d
```

**What happens:**
1. Docker Compose reads `docker-compose.yml`
2. Loads environment variables from `.env`
3. Builds images if needed
4. Creates networks and volumes
5. Starts containers

**Step 3.3.2: View Logs**

```bash
# View all service logs
docker-compose logs

# Follow logs in real-time
docker-compose logs -f

# View specific service logs
docker-compose logs platter
```

**Step 3.3.3: Stop Services**

```bash
# Stop all services
docker-compose down

# Stop and remove volumes (⚠️ deletes data)
docker-compose down -v
```

---

## ⚙️ Section 4: Advanced Configuration

### 4.1: Custom Network Configuration

**Step 4.2.1: Create Custom Network**

```bash
# Create custom bridge network
docker network create platter-network

# Run container on custom network
docker run -d \
  --name platter \
  --network platter-network \
  -p 8080:8080 \
  platter:latest
```

**Benefits:**
- Isolated network for related containers
- Service discovery by container name
- Better security through network isolation

**Step 4.2.2: Docker Compose with Networks**

```yaml
services:
  platter:
    # ... other configuration ...
    networks:
      - platter-net

networks:
  platter-net:
    driver: bridge
```

---

## 💾 Section 5: Data Persistence

### 5.1: Named Volumes (Recommended)

**Purpose:** Managed by Docker, survives container deletion.

**Step 5.1.1: Create Named Volume**

```bash
# Create volume
docker volume create platter-data

# Inspect volume
docker volume inspect platter-data
```

**Step 5.1.2: Use Named Volume**

```bash
# Mount named volume
docker run -d \
  --name platter \
  -v platter-data:/app/data \
  platter:latest
```

**Benefits:**
- ✅ Docker manages volume lifecycle
- ✅ Easy to backup and restore
- ✅ Works across container recreations
- ✅ Better performance on Docker Desktop

### 5.2: Bind Mounts (Development)

**Purpose:** Direct mapping to host filesystem, useful for development.

**Step 5.2.1: Use Bind Mount**

```bash
# Mount local directory
docker run -d \
  --name platter \
  -v $(pwd)/data:/app/data \
  platter:latest
```

**Step 5.2.2: Docker Compose Bind Mount**

```yaml
services:
  platter:
    # ... other configuration ...
    volumes:
      - ./data:/app/data  # Relative path to current directory
```

**Benefits:**
- ✅ Direct file access from host
- ✅ Easy to edit configuration files
- ✅ Useful for development and debugging

**Considerations:**
- ❌ Path must exist on host
- ❌ Slower performance on Docker Desktop (macOS/Windows)
- ❌ Permission issues possible on Linux

### 5.3: Backup and Restore

**Step 5.3.1: Backup Volume Data**

```bash
# Create timestamped backup
docker run --rm \
  -v platter-data:/data \
  -v $(pwd):/backup \
  alpine tar czf /backup/platter-backup-$(date +%Y%m%d).tar.gz -C /data .
```

**What this does:**
1. Runs temporary Alpine Linux container
2. Mounts platter-data volume to `/data`
3. Mounts current directory to `/backup`
4. Creates compressed archive of volume contents
5. Removes container after completion

**Step 5.3.2: Restore from Backup**

```bash
# Restore backup to volume
docker run --rm \
  -v platter-data:/data \
  -v $(pwd):/backup \
  alpine tar xzf /backup/platter-backup-20250103.tar.gz -C /data
```

**What this does:**
1. Runs temporary Alpine Linux container
2. Extracts backup archive to volume
3. Removes container after completion

**Step 5.3.3: Automated Backup Script**

Create `backup-docker.sh`:

```bash
#!/bin/bash
# Docker volume backup script

DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="./backups"
VOLUME_NAME="platter-data"

mkdir -p "$BACKUP_DIR"

docker run --rm \
  -v $VOLUME_NAME:/data \
  -v $(pwd)/$BACKUP_DIR:/backup \
  alpine tar czf /backup/backup_$DATE.tar.gz -C /data .

echo "Backup created: $BACKUP_DIR/backup_$DATE.tar.gz"

# Keep only last 7 backups
ls -t $BACKUP_DIR/backup_*.tar.gz | tail -n +8 | xargs -r rm

chmod +x backup-docker.sh
```

---

## 🔒 Section 6: Production Best Practices

### 6.1: Security Hardening

**Step 6.1.1: Use Specific Image Versions**

```dockerfile
# Good: Specific versions
FROM rust:1.80-slim as builder
FROM debian:bullseye-slim

# Avoid: Latest tags
FROM rust:latest as builder
FROM debian:latest
```

**Why:** Specific versions ensure reproducible builds and prevent unexpected changes.

**Step 6.1.2: Run as Non-Root User**

```dockerfile
# Create non-root user
RUN useradd -m -u 1000 platter

# Set ownership
RUN chown -R platter:platter /app

# Switch to non-root user
USER platter
```

**Why:** Reduces security risk by limiting container privileges.

**Step 6.1.3: Minimise Image Size**

```dockerfile
# Multi-stage build with Alpine
FROM rust:1.80-alpine as builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Minimal runtime image
FROM alpine:latest

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/platter .
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/static ./static

RUN mkdir -p data

EXPOSE 8080

CMD ["./platter"]
```

**Benefits:**
- Smaller image size (typically 20-50 MB)
- Faster downloads and deployments
- Reduced attack surface

### 6.2: Production Dockerfile

Complete production-ready Dockerfile:

```dockerfile
# Build stage
FROM rust:1.80-alpine as builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev

# Copy and build
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates curl

# Create non-root user
RUN addgroup -g 1000 platter && \
    adduser -D -u 1000 -G platter platter

WORKDIR /app

# Copy binary and assets
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/platter .
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/static ./static

# Set ownership
RUN chown -R platter:platter /app && \
    mkdir -p data && \
    chown -R platter:platter data

# Switch to non-root user
USER platter

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
  CMD curl -f http://localhost:8080/ || exit 1

EXPOSE 8080

CMD ["./platter"]
```

### 6.3: Production Docker Compose

Complete production configuration:

```yaml
version: '3.8'

services:
  platter:
    build:
      context: .
      dockerfile: Dockerfile.prod
    image: platter:production
    container_name: platter-prod
    ports:
      - "127.0.0.1:8080:8080"  # Bind to localhost only
    environment:
      - PRODUCTION=true
      - SESSION_SECRET=${SESSION_SECRET}
      - CORS_ALLOWED_ORIGINS=${CORS_ALLOWED_ORIGINS}
      - RUST_LOG=warn
    volumes:
      - platter-data:/app/data:rw
    restart: always
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
    networks:
      - platter-net
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 512M
        reservations:
          cpus: '0.5'
          memory: 256M

  # Optional: Nginx reverse proxy
  nginx:
    image: nginx:alpine
    container_name: platter-nginx
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/nginx/ssl:ro
    depends_on:
      - platter
    restart: always
    networks:
      - platter-net

networks:
  platter-net:
    driver: bridge

volumes:
  platter-data:
    driver: local
```

**Configuration highlights:**
- Port binding to localhost only (accessed via reverse proxy)
- Resource limits prevent excessive consumption
- Structured logging with rotation
- Health checks for automatic recovery
- Network isolation

### 6.4: Resource Management

**Set resource limits:**

```yaml
services:
  platter:
    # ... other configuration ...
    deploy:
      resources:
        limits:
          cpus: '1.0'      # Maximum 1 CPU core
          memory: 512M     # Maximum 512 MB RAM
        reservations:
          cpus: '0.5'      # Reserve 0.5 CPU core
          memory: 256M     # Reserve 256 MB RAM
```

**Why set limits:**
- Prevents resource exhaustion
- Ensures predictable performance
- Enables better capacity planning
- Protects other services on same host

---

## 🔄 Section 7: Updates and Maintenance

### 7.1: Update Container

**Step 7.1.1: Pull Latest Changes**

```bash
# Pull latest code
git pull origin main
```

**Step 7.1.2: Rebuild and Deploy**

```bash
# Rebuild image
docker-compose build

# Recreate container with new image
docker-compose up -d

# View logs
docker-compose logs -f platter
```

**What happens:**
1. Docker Compose rebuilds image with new code
2. Stops old container
3. Starts new container with updated image
4. Old container removed automatically

### 7.2: Cleanup Commands

**Remove stopped containers:**

```bash
# Remove all stopped containers
docker container prune

# Confirm with 'y'
```

**Remove unused images:**

```bash
# Remove dangling images
docker image prune

# Remove all unused images
docker image prune -a
```

**Remove unused volumes:**

```bash
# Remove unused volumes (⚠️ deletes data)
docker volume prune
```

**Complete system cleanup:**

```bash
# Remove all unused resources
docker system prune -a

# Include volumes (⚠️ deletes all unused data)
docker system prune -a --volumes
```

> ⚠️ **Warning:** Volume cleanup commands permanently delete data. Always backup before running.

---

## 📊 Section 8: Monitoring

### 8.1: View Logs

**Docker logs:**

```bash
# View logs
docker logs platter

# Follow logs in real-time
docker logs -f platter

# Show last 100 lines
docker logs --tail 100 platter

# Show logs with timestamps
docker logs -t platter
```

**Docker Compose logs:**

```bash
# View all service logs
docker-compose logs

# Follow specific service
docker-compose logs -f platter

# Show timestamps
docker-compose logs -t
```

### 8.2: Container Statistics

**Real-time statistics:**

```bash
# Monitor single container
docker stats platter

# Monitor all containers
docker stats

# Display once (non-streaming)
docker stats --no-stream
```

**What statistics show:**
- CPU usage percentage
- Memory usage and limit
- Network I/O (Input/Output)
- Disk I/O
- Process count

**Docker Compose statistics:**

```bash
# View resource usage
docker-compose top
```

### 8.3: Health Checks

**Check container health:**

```bash
# View health status
docker inspect --format='{{.State.Health.Status}}' platter

# View health check logs
docker inspect --format='{{json .State.Health}}' platter | jq
```

**Health status values:**
- `healthy`: Container is healthy
- `unhealthy`: Health check failing
- `starting`: Waiting for initial health check

---

## 🎯 Section 9: Next Steps

After completing Docker deployment, proceed with these tasks:

### For System Administrators

Complete these tasks in sequence:

1. **Configure Reverse Proxy** – Set up Nginx or Apache with SSL/TLS
2. **Implement Monitoring** – Configure health checks and alerting
3. **Schedule Backups** – Automate daily volume backups
4. **Document Procedures** – Create runbooks for common operations

### For Development Teams

Complete these tasks in sequence:

1. **Optimise Build Process** – Improve Docker layer caching
2. **Add Development Compose** – Create `docker-compose.dev.yml` for development
3. **Configure CI/CD** – Automate image building and deployment
4. **Security Scanning** – Implement container vulnerability scanning

---

## 📖 Related Documentation

Access these resources for additional information:

- **[Production Deployment Guide](production.md)** – Non-containerised deployment instructions
- **[Configuration Guide](../guides/configuration.md)** – Environment variables and settings reference
- **[Security Documentation](../architecture/security.md)** – Security best practices
- **[Getting Started Guide](../guides/getting-started.md)** – Initial setup instructions

---

[← Back to Deployment](README.md) | [Documentation Home](../README.md)