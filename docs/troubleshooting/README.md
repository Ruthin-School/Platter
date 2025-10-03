# 🔧 Troubleshooting Guide

> **Navigation:** [Documentation Home](../README.md) → Troubleshooting

Solutions to common issues and debugging guides for Platter.

---

## 📚 Troubleshooting Guides

### [🔐 OAuth Troubleshooting Guide](oauth.md)

**What this covers:** Resolve OAuth authentication issues with Microsoft Entra ID

**Topics included:**
- Quick fixes for common OAuth problems
- Configuration verification procedures
- Error message interpretation and solutions
- Advanced debugging techniques
- Log message reference guide

**Reading time:** Approximately 15 minutes

---

## 🔍 Quick Diagnostics

### Common Issues Overview

**Authentication problems:**
- OAuth login button not appearing on login page
- Authentication fails after Microsoft redirect
- Session persistence problems
- Automatic re-login after logout

**Configuration problems:**
- Configuration file errors or missing files
- CSRF (Cross-Site Request Forgery) token validation failures
- Environment variable issues
- Redirect URI (Uniform Resource Identifier) mismatch errors

**Deployment problems:**
- Application fails to start
- Permission errors accessing data directory
- Port already in use conflicts
- HTTPS certificate issues

---

### Diagnostic Process

Execute these steps before detailed debugging:

**Step 1: Review Configuration**

Read the [Configuration Guide](../guides/configuration.md) to verify all settings are correct.

**Step 2: Examine Application Logs**

```bash
# View logs for systemd service
sudo journalctl -u platter -n 100

# View logs for Docker container
docker logs platter

# View logs for direct execution
./platter 2>&1 | tee debug.log
```

**Step 3: Verify Environment Variables**

```bash
# Check required variables are set
echo $PRODUCTION
echo $SESSION_SECRET
echo $CORS_ALLOWED_ORIGINS
```

**Step 4: Test in Isolation**

```bash
# Try in incognito/private browser window
# This eliminates cached data and cookies
```

---

## 🎯 Quick Navigation Links

**Core documentation:**
- [Main README](../../README.md) – Project overview
- [OAuth Setup Guide](../guides/oauth-setup.md) – Initial OAuth configuration
- [Configuration Guide](../guides/configuration.md) – Environment variables reference
- [Security Documentation](../architecture/security.md) – Security settings

---

## 🔧 Debugging Tools and Commands

### Enable Debug Logging

**Linux or macOS:**
```bash
# Set debug log level
export RUST_LOG=debug
./platter
```

**Windows PowerShell:**
```bash
# Set debug log level
$env:RUST_LOG="debug"
.\platter.exe
```

**Log levels available:**
- `error`: Error messages only (minimal logging)
- `warn`: Warnings and errors
- `info`: General information, warnings, and errors (default)
- `debug`: Detailed debugging information
- `trace`: Extremely verbose logging (includes all debug information)

---

### Validate Configuration Files

**Validate JSON syntax:**

```bash
# Using jq (if installed)
jq '.' data/oauth_config.json

# Using Python
python3 -m json.tool data/oauth_config.json

# Using Node.js
node -e "console.log(JSON.parse(require('fs').readFileSync('data/oauth_config.json')))"
```

**Expected result:** Formatted JSON output (file is valid)  
**Error result:** Syntax error with line number (file has errors)

---

### Network Debugging

**Browser Developer Tools (F12):**

**Network tab usage:**
1. Open developer tools (press F12)
2. Navigate to "Network" tab
3. Attempt the failing operation
4. Examine requests and responses
5. Look for HTTP status codes (4xx or 5xx indicate errors)

**Console tab usage:**
1. Check for JavaScript errors
2. Review security warnings
3. Examine CORS errors

**Application tab usage:**
1. Inspect cookies (verify session cookie exists)
2. Check local storage
3. Review cookie attributes (HttpOnly, Secure, SameSite)

---

## 📖 Related Documentation

Access additional troubleshooting resources:

- **[OAuth Setup Guide](../guides/oauth-setup.md)** – Initial OAuth configuration instructions
- **[Configuration Guide](../guides/configuration.md)** – Environment variables and settings
- **[Security Documentation](../architecture/security.md)** – Security settings and requirements
- **[Production Deployment Guide](../deployment/production.md)** – Production troubleshooting procedures

---

[← Back to Documentation Home](../README.md)