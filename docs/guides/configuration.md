# ⚙️ Configuration Guide

> **Navigation:** [Documentation Home](../README.md) → [Getting Started](README.md) → Configuration

**Reading Time:** Approximately 15 minutes  
**Complexity:** Intermediate  
**Prerequisites:** Basic understanding of environment variables

## 📖 What This Guide Covers

This guide provides complete configuration reference for Platter. You will learn how to:

1. Set environment variables for development and production
2. Configure security settings
3. Manage CORS (Cross-Origin Resource Sharing) policies
4. Configure data file locations
5. Troubleshoot common configuration issues

---

## 📑 Table of Contents

Navigate to specific sections:

- [Section 1: Environment Variables](#-section-1-environment-variables)
- [Section 2: Security Configuration](#-section-2-security-configuration)
- [Section 3: CORS Configuration](#-section-3-cors-configuration)
- [Section 4: Production vs Development](#-section-4-production-vs-development)
- [Section 5: Configuration Files](#-section-5-configuration-files)

---

## 🔧 Section 1: Environment Variables

Environment variables control how Platter behaves. This section explains all available variables.

### Core Configuration Variables

| Variable Name | Purpose | Default Value | Required? | Example Value |
|--------------|---------|--------------|-----------|---------------|
| `PORT` | Port number for the web server | `8080` | No | `3000` |
| `HOST` | IP address to bind the server to | `0.0.0.0` | No | `127.0.0.1` |
| `RUST_LOG` | Logging verbosity level | `info` | No | `debug`, `warn`, `error` |

**Explanation of values:**
- **PORT** – The network port number (1-65535) where the application listens for connections
- **HOST** – The IP address; `0.0.0.0` means "listen on all network interfaces", `127.0.0.1` means "listen only on localhost"
- **RUST_LOG** – Controls detail level of log messages: `error` (least verbose), `warn`, `info`, `debug` (most verbose)

### Security Configuration Variables

| Variable Name | Purpose | Default Value | Required for Production? | Example Value |
|--------------|---------|--------------|-------------------------|---------------|
| `SESSION_SECRET` | Encryption key for session cookies (64 bytes) | Insecure development key | **Yes** | Generated using `openssl rand -base64 64` |
| `PRODUCTION` | Enables production security mode | Not set (development mode) | **Recommended** | `true` |
| `CORS_ALLOWED_ORIGINS` | Comma-separated list of allowed origins | `http://localhost:8080,http://127.0.0.1:8080` | No | `https://yourdomain.com` |

> ⚠️ **Critical Security Warning:** You MUST set a strong `SESSION_SECRET` in production environments. Using the default development key creates a severe security vulnerability.

### How to Set Environment Variables

Choose the method for your operating system:

#### Method A: Linux or macOS

Execute these commands in your terminal:

```bash
# Set individual variables
export PORT=3000
export PRODUCTION=true
export SESSION_SECRET=$(openssl rand -base64 64)

# Run the application
./platter
```

**What this does:**
- Line 1: Sets the port to 3000
- Line 2: Enables production mode
- Line 3: Generates a secure random session secret
- Line 4: Starts the application with these settings

#### Method B: Windows PowerShell

Execute these commands in PowerShell:

```powershell
# Set individual variables
$env:PORT="3000"
$env:PRODUCTION="true"
$env:SESSION_SECRET=$(openssl rand -base64 64 | Out-String).Trim()

# Run the application
.\platter.exe
```

#### Method C: Windows Command Prompt

Execute these commands in Command Prompt:

```cmd
REM Set individual variables
set PORT=3000
set PRODUCTION=true

REM Run the application
platter.exe
```

> 📘 **Note:** Command Prompt cannot generate session secrets directly. Use PowerShell or generate the secret externally.

---

## 🔐 Section 2: Security Configuration

This section explains critical security settings.

### Session Secret Configuration

The session secret encrypts user session data. A compromised secret allows attackers to forge sessions.

#### Requirement: Strong Random Secret

Follow these requirements:

1. **Minimum Length** – 32 bytes (64 bytes recommended)
2. **Randomness** – Must be cryptographically random
3. **Confidentiality** – Never commit to version control
4. **Rotation** – Change periodically in production (every 90 days recommended)

#### Step 2.1: Generate a Secure Session Secret

Execute this command on Linux or macOS:

```bash
# Generate and display a secure secret
openssl rand -base64 64 | tr -d '\n'
```

Execute this command on Windows PowerShell:

```powershell
# Generate and display a secure secret
openssl rand -base64 64
```

#### Step 2.2: Set the Session Secret

Execute this command to set the secret:

**Linux/macOS:**
```bash
export SESSION_SECRET=$(openssl rand -base64 64 | tr -d '\n')
```

**Windows PowerShell:**
```powershell
$env:SESSION_SECRET=$(openssl rand -base64 64 | Out-String).Trim()
```

> 📘 **Important:** The `tr -d '\n'` command on Linux/macOS removes newline characters. On Windows, `.Trim()` achieves the same result.

### Production Mode Configuration

Production mode enables enhanced security features.

#### Step 2.3: Enable Production Mode

Set this environment variable:

```bash
export PRODUCTION=true
```

#### What Production Mode Enables

When `PRODUCTION=true`, the application activates:

1. **Secure Cookie Flags** – Sets HttpOnly and Secure flags on cookies
2. **Stricter CORS Policies** – Restricts cross-origin requests to allowed origins only
3. **Enhanced Session Security** – Implements additional session validation
4. **Production-Optimised Logging** – Reduces log verbosity to warnings and errors

#### Example: Complete Production Configuration

```bash
# Enable production mode
export PRODUCTION=true

# Set secure session secret
export SESSION_SECRET=$(openssl rand -base64 64 | tr -d '\n')

# Configure CORS
export CORS_ALLOWED_ORIGINS=https://yourdomain.com,https://admin.yourdomain.com

# Run the application
./platter
```

---

## 🌐 Section 3: CORS Configuration

CORS (Cross-Origin Resource Sharing) controls which websites can access your API.

### Development Mode CORS

**Default behaviour:** Allows all origins for development convenience

**Risk level:** High (acceptable only for local development)

**When to use:** Local development environments only

### Production Mode CORS

**Default behaviour:** Restricts to specified origins

**Risk level:** Low (when configured correctly)

**When to use:** All production environments

#### Step 3.1: Configure Allowed Origins

Set the CORS_ALLOWED_ORIGINS variable with comma-separated origins:

```bash
export CORS_ALLOWED_ORIGINS="https://yourdomain.com,https://admin.yourdomain.com"
```

**Important formatting rules:**
1. Use full URLs including protocol (`https://`)
2. Separate multiple origins with commas (no spaces)
3. Do not include trailing slashes
4. Match the exact domains users will access

#### CORS Best Practices

Follow these security practices:

1. ✅ **Allow Only Trusted Domains** – Never use wildcard (`*`) in production
2. ✅ **Use HTTPS in Production** – Always use `https://` protocol
3. ✅ **Avoid Wildcards** – Specify exact domains rather than patterns
4. ✅ **Test Thoroughly** – Verify CORS configuration before deployment
5. ✅ **Audit Regularly** – Review allowed origins periodically

---

## 🔄 Section 4: Production vs Development

This section explains the differences between development and production configurations.

### Development Configuration

**Purpose:** Local development and testing

**Characteristics:**
- Permissive CORS (allows all origins)
- Detailed logging (debug level)
- Uses insecure default session secret
- Supports hot-reload and rapid iteration

**Example configuration:**

```bash
# Development settings
PORT=8080
RUST_LOG=debug
# SESSION_SECRET uses insecure default (development only)
```

**When to use:** Local development environments only

### Production Configuration

**Purpose:** Live deployment serving real users

**Characteristics:**
- Restricted CORS (specified origins only)
- Reduced logging (warnings and errors only)
- Requires secure session secret
- Enables all security features

**Example configuration:**

```bash
# Production settings
PRODUCTION=true
SESSION_SECRET=$(openssl rand -base64 64 | tr -d '\n')
CORS_ALLOWED_ORIGINS=https://yourdomain.com
PORT=8080
RUST_LOG=warn
```

**When to use:** All production and staging environments

### Production Deployment Checklist

Complete these requirements before production deployment:

- [ ] **Step 1:** Set `PRODUCTION=true`
- [ ] **Step 2:** Generate and set strong `SESSION_SECRET`
- [ ] **Step 3:** Configure `CORS_ALLOWED_ORIGINS` with exact domains
- [ ] **Step 4:** Configure HTTPS/TLS certificates
- [ ] **Step 5:** Change default admin credentials
- [ ] **Step 6:** Set appropriate `RUST_LOG` level (`warn` or `error`)
- [ ] **Step 7:** Verify all configuration before launch

---

## 📁 Section 5: Configuration Files

This section explains the hybrid TOML/JSON configuration system.

### Configuration Directory Structure

TOML configuration files (human-editable):

```
config/
├── admin.toml              # Admin users (❌ DO NOT COMMIT)
├── admin.toml.example      # Admin template (✅ safe to commit)
├── validation.toml         # Validation rules (✅ safe to commit)
└── settings.toml           # App settings (✅ safe to commit)
```

### Data Directory Structure

JSON data files (application-managed):

```
data/
├── menu_items.json         # Menu items (❌ DO NOT COMMIT)
├── menu_items.json.example # Menu template (✅ safe to commit)
├── menu_presets.json       # Menu presets (❌ DO NOT COMMIT)
├── menu_schedules.json     # Menu schedules (❌ DO NOT COMMIT)
└── notices.json            # Announcements (❌ DO NOT COMMIT)
```

> 📘 **Note:** v0.8.0 uses enhanced JSON format with metadata wrappers. See [`STORAGE_ARCHITECTURE.md`](../../STORAGE_ARCHITECTURE.md) for details.

#### Required File Permissions

Set these permissions on Linux/macOS:

```bash
# Directory permissions (rwxr-xr-x)
chmod 755 data/

# File permissions (rw-r--r--)
chmod 644 data/*.json
```

**Permission explanation:**
- **755 (directory)** – Owner can read/write/execute, others can read/execute
- **644 (files)** – Owner can read/write, others can read only

**Application requirements:**
- The application must have write access to the data directory
- Files must be readable by the application user

---

## 🔍 Section 6: Configuration Validation

This section explains how to verify your configuration.

### Step 6.1: Check Current Configuration

Execute this command to view environment variables:

```bash
# View all Platter-related environment variables
env | grep -E "(PORT|HOST|RUST_LOG|PRODUCTION|CORS_ALLOWED_ORIGINS|SESSION_SECRET)"
```

### Step 6.2: Validate JSON Configuration Files

Execute this command to check JSON syntax:

```bash
# Validate admin users file
jq '.' data/admin_users.json
```

**Expected output:** Formatted JSON with no error messages

**If jq is not installed:**
```bash
# Alternative: Use Python
python3 -m json.tool data/admin_users.json
```

### Common Configuration Issues

#### Issue 6.1: Missing SESSION_SECRET in Production

**Symptom:** Error message "Production mode requires SESSION_SECRET"

**Cause:** `PRODUCTION=true` is set but `SESSION_SECRET` is not defined

**Solution:**

```bash
# Generate and set session secret
export SESSION_SECRET=$(openssl rand -base64 64 | tr -d '\n')
```

#### Issue 6.2: CORS Errors

**Symptom:** Browser console shows "CORS policy" errors

**Diagnosis:** Check current CORS configuration:

```bash
echo $CORS_ALLOWED_ORIGINS
```

**Solution:** Ensure your domain is in the allowed list:

```bash
export CORS_ALLOWED_ORIGINS="https://yourdomain.com,https://app.yourdomain.com"
```

#### Issue 6.3: Port Already in Use

**Symptom:** Error "Address already in use" or "Port 8080 is already in use"

**Diagnosis:** Another process is using the port

**Solution:** Use a different port:

```bash
export PORT=3000
./platter
```

---

## 🎯 Section 7: Configuration Examples

This section provides complete configuration examples for different environments.

### Example 7.1: Local Development

Create file `.env.development` (do not commit to version control):

```bash
# Development configuration
PORT=8080
HOST=127.0.0.1
RUST_LOG=debug
```

**Use case:** Local development and testing

### Example 7.2: Staging Environment

Create file `.env.staging`:

```bash
# Staging configuration
PRODUCTION=true
PORT=8080
SESSION_SECRET=<generated-secret>
CORS_ALLOWED_ORIGINS=https://staging.yourdomain.com
RUST_LOG=info
```

**Use case:** Pre-production testing environment

### Example 7.3: Production Environment

Create file `.env.production`:

```bash
# Production configuration
PRODUCTION=true
PORT=8080
SESSION_SECRET=<generated-secret>
CORS_ALLOWED_ORIGINS=https://yourdomain.com,https://admin.yourdomain.com
RUST_LOG=warn
```

**Use case:** Live production environment

---

## 📖 Related Documentation

Access these resources for additional information:

- **[Getting Started Guide](getting-started.md)** – Initial installation and setup
- **[Security Documentation](../architecture/security.md)** – Security best practices and guidelines
- **[Production Deployment](../deployment/production.md)** – Production deployment procedures
- **[Troubleshooting Guide](../troubleshooting/README.md)** – Resolve configuration issues

---

---

[← Back to Guides](README.md) | [Documentation Home](../README.md)