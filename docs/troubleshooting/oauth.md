# 🔐 OAuth Troubleshooting Guide

> **Navigation:** [Documentation Home](../README.md) → [Troubleshooting](README.md) → OAuth

**Reading Time:** Approximately 15 minutes  
**Complexity:** Intermediate  
**Prerequisites:** Understanding of OAuth 2.0 authentication flow and Microsoft Entra ID

## 📖 What This Guide Covers

This guide provides comprehensive troubleshooting information for OAuth authentication issues with Microsoft Entra ID. By the end of this guide, you will understand:

1. Quick fixes for common OAuth problems
2. How to interpret error messages
3. Configuration verification procedures
4. Advanced debugging techniques
5. Log message references

---

## 📋 Section 1: Quick Fixes

### 1.1: Login Button Not Appearing

**Symptom:** The "Login with Microsoft" button does not display on the login page.

**Diagnostic steps:**

**Step 1.1.1: Verify Configuration File Exists**

```bash
# Check if config file exists
ls -la data/oauth_config.json

# View file contents
cat data/oauth_config.json
```

**Step 1.1.2: Check OAuth is Enabled**

Verify `oauth_config.json` contains:
```json
{
  "enabled": true
}
```

**Step 1.1.3: Restart Application**

```bash
# Restart to load configuration
cargo run
# or
systemctl restart platter
```

**Step 1.1.4: Clear Browser Cache**

1. Open browser developer tools (F12)
2. Right-click refresh button
3. Select "Empty Cache and Hard Reload"

### 1.2: Authentication Fails After Microsoft Redirect

**Symptom:** Login redirects to Microsoft successfully, but returns to login page or shows an error.

**Common causes:**

| Cause | Symptom | Solution |
|-------|---------|----------|
| Email not authorised | "Email not authorized" error | Add email to `allowed_emails` array |
| CSRF token mismatch | "CSRF validation failed" error | Clear cookies and try again |
| Invalid redirect URI | Microsoft shows URI error | Verify redirect URI configuration |
| Session expired | Returns to login page | Try in incognito window |

**Solution 1.2.1: Verify Email in Allowed List**

Edit `data/oauth_config.json`:
```json
{
  "allowed_emails": [
    "your-email@domain.com",
    "admin@domain.com"
  ]
}
```

**Important:** Email comparison is case-insensitive.

**Solution 1.2.2: Clear Browser Data**

1. Press `Ctrl+Shift+Delete` (Windows/Linux) or `Cmd+Shift+Delete` (macOS)
2. Select "Cookies and other site data"
3. Select "All time" time range
4. Click "Clear data"

**Solution 1.2.3: Test in Incognito Mode**

1. Open new incognito/private window
2. Navigate to application
3. Attempt login again

### 1.3: Automatically Logged Back In After Logout

**Symptom:** After logging out, the application immediately logs you back in.

**Cause:** Microsoft Entra ID session remains active.

**Solution steps:**

**Step 1.3.1: Use OIDC Logout**

Ensure you are using the latest application version with OIDC (OpenID Connect) logout support.

**Step 1.3.2: Clear All Cookies**

1. Clear cookies for your application domain
2. Clear cookies for `login.microsoftonline.com`
3. Close all browser tabs
4. Restart browser

**Step 1.3.3: Force Complete Logout**

1. Log out from application
2. Navigate to `https://login.microsoftonline.com/common/oauth2/v2.0/logout`
3. Clear browser cache
4. Return to application

---

## ⚠️ Section 2: Common Error Messages

### 2.1: "Email not authorised for admin access"

**Error meaning:** Your email address is not in the allowed emails list.

**Solution:**

**Step 2.1.1: Add Email to Configuration**

Edit `data/oauth_config.json`:

```json
{
  "allowed_emails": [
    "your-email@domain.com",
    "admin@domain.com"
  ]
}
```

**Step 2.1.2: Restart Application**

```bash
# Reload configuration
systemctl restart platter
# or
cargo run
```

**Step 2.1.3: Verify Case Sensitivity**

Email comparison is case-insensitive, so these are equivalent:
- `Admin@Domain.com`
- `admin@domain.com`
- `ADMIN@DOMAIN.COM`

### 2.2: "CSRF token validation failed"

**Error meaning:** Security token mismatch, usually from expired session or browser issue.

**What CSRF protection does:**
- Generates unique token for each login attempt
- Stores token in session
- Validates token on callback
- Prevents cross-site request forgery attacks

**Solutions:**

**Solution 2.2.1: Clear Browser Cookies**

```bash
# In browser developer tools (F12)
# Navigate to: Application → Cookies
# Delete all cookies for your domain
```

**Solution 2.2.2: Ensure Cookies Enabled**

1. Open browser settings
2. Navigate to Privacy and Security
3. Ensure "Allow all cookies" or "Block third-party cookies" only
4. Restart browser

**Solution 2.2.3: Try Different Browser**

1. Open different browser (Chrome, Firefox, Edge)
2. Or use incognito/private mode
3. Attempt login again

**Solution 2.2.4: Check Reverse Proxy Configuration**

If using reverse proxy, verify it preserves sessions:

```nginx
# Nginx configuration
proxy_pass http://127.0.0.1:8080;
proxy_cookie_path / "/; secure; HttpOnly; SameSite=Strict";
```

### 2.3: "OAuth is not enabled"

**Error meaning:** OAuth configuration is disabled or not loaded.

**Solutions:**

**Solution 2.3.1: Enable OAuth in Configuration**

Verify `data/oauth_config.json` contains:
```json
{
  "enabled": true
}
```

**Solution 2.3.2: Check Configuration Loading**

View application startup logs:
```bash
# Check for configuration loading messages
journalctl -u platter -n 50 | grep -i "oauth"
```

**Expected log output:**
```
OAuth config loaded successfully
```

**Solution 2.3.3: Validate JSON Syntax**

```bash
# Validate JSON syntax
jq empty data/oauth_config.json

# Or using Python
python3 -m json.tool data/oauth_config.json
```

**If valid:** No output or formatted JSON  
**If invalid:** Error message with line number

**Solution 2.3.4: Restart Application**

```bash
# Restart to reload configuration
systemctl restart platter
```

### 2.4: "OAuth configuration not found"

**Error meaning:** Configuration file is missing or cannot be loaded.

**Solutions:**

**Solution 2.4.1: Create Configuration File**

```bash
# Create configuration file
cat > data/oauth_config.json << 'EOF'
{
  "client_id": "your-client-id",
  "client_secret": "your-client-secret",
  "issuer_url": "https://login.microsoftonline.com/your-tenant-id/v2.0",
  "redirect_url": "https://yourdomain.com/auth_callback",
  "allowed_emails": ["admin@yourdomain.com"],
  "enabled": true
}
EOF
```

**Solution 2.4.2: Verify File Permissions**

```bash
# Check file permissions
ls -la data/oauth_config.json

# Fix permissions if needed
chmod 644 data/oauth_config.json
```

**Solution 2.4.3: Check File Path**

Ensure configuration file is in correct location:
```
/path/to/application/data/oauth_config.json
```

**Solution 2.4.4: Review Application Logs**

```bash
# Check for file system errors
journalctl -u platter | grep -i "oauth_config"
```

### 2.5: Redirect URI Mismatch Errors

**Symptom:** Error from Microsoft stating redirect URI does not match.

**Error example:**
```
AADSTS50011: The redirect URI specified in the request does not match
the redirect URIs configured for the application
```

**Solutions:**

**Solution 2.5.1: Verify Exact Match**

Redirect URI in `oauth_config.json` must EXACTLY match Azure Portal configuration.

**Check for common mismatches:**
- Trailing slash: `https://domain.com/callback` vs `https://domain.com/callback/`
- Protocol: `http://` vs `https://`
- Port number: `https://domain.com` vs `https://domain.com:443`
- Path: `/auth_callback` vs `/callback`

**Solution 2.5.2: Update Azure Portal Configuration**

1. Navigate to [Azure Portal](https://portal.azure.com/)
2. Go to "App registrations"
3. Select your application
4. Click "Authentication" in left menu
5. Under "Redirect URIs", verify or add your URI
6. Click "Save"

**Solution 2.5.3: Update Application Configuration**

Ensure `oauth_config.json` matches Azure Portal:
```json
{
  "redirect_url": "https://yourdomain.com/auth_callback"
}
```

---

## ✅ Section 3: Configuration Verification

### 3.1: Local Configuration Checks

**Check 3.1.1: Verify Configuration File**

```bash
# Check file exists
ls -la data/oauth_config.json

# View contents
cat data/oauth_config.json
```

**Check 3.1.2: Validate JSON Syntax**

```bash
# Using jq (if installed)
jq '.' data/oauth_config.json

# Using Python
python3 -m json.tool data/oauth_config.json
```

**Check 3.1.3: Verify Required Fields**

Configuration must include:

| Field | Example | Source |
|-------|---------|--------|
| `client_id` | `12345678-1234-1234-1234-123456789abc` | Azure Portal → App registrations → Application (client) ID |
| `client_secret` | `abc123...` | Azure Portal → Certificates & secrets → Value (not ID) |
| `issuer_url` | `https://login.microsoftonline.com/{tenant-id}/v2.0` | Use your tenant ID |
| `redirect_url` | `https://yourdomain.com/auth_callback` | Must match Azure Portal exactly |
| `allowed_emails` | `["admin@domain.com"]` | Array of authorised email addresses |
| `enabled` | `true` | Must be `true` to enable OAuth |

### 3.2: Azure Portal Verification

**Step 3.2.1: Verify App Registration**

1. Navigate to [Azure Portal](https://portal.azure.com/)
2. Go to "Microsoft Entra ID" (formerly Azure Active Directory)
3. Select "App registrations" from left menu
4. Find your application
5. Note the "Application (client) ID" (use as `client_id`)
6. Note the "Directory (tenant) ID" (use in `issuer_url`)

**Step 3.2.2: Check Redirect URI**

1. In your app registration, click "Authentication"
2. Under "Platform configurations" → "Web"
3. Verify redirect URI matches your `redirect_url` exactly
4. Format: `https://your-domain.com/auth_callback`

**Step 3.2.3: Verify API Permissions**

1. Click "API permissions" in your app registration
2. Ensure these Microsoft Graph permissions are granted:
   - ✅ `openid` (Sign users in)
   - ✅ `email` (View users' email address)
   - ✅ `profile` (View users' basic profile)
3. Click "Grant admin consent" if permissions show "Not granted"

**Step 3.2.4: Validate Client Secret**

1. Go to "Certificates & secrets" in your app registration
2. Verify the secret hasn't expired
3. If expired, create new secret:
   - Click "New client secret"
   - Set description and expiration
   - Copy the **Value** (not the Secret ID)
   - Update `client_secret` in `oauth_config.json`

> ⚠️ **Critical:** Always use the secret **Value**, not the Secret ID.

**Step 3.2.5: Verify Tenant ID**

1. From app Overview page, copy "Directory (tenant) ID"
2. Your `issuer_url` should be:
   ```
   https://login.microsoftonline.com/{tenant-id}/v2.0
   ```
3. Replace `{tenant-id}` with actual tenant ID

---

## 🔬 Section 4: Advanced Debugging

### 4.1: Enable Detailed Logging

**Step 4.1.1: Set Log Level**

```bash
# Linux/macOS
export RUST_LOG=debug
./platter

# Windows PowerShell
$env:RUST_LOG="debug"
.\platter.exe
```

**Log levels (from most to least verbose):**
- `trace`: Extremely detailed (includes all debug info)
- `debug`: Detailed debugging information
- `info`: General informational messages (default)
- `warn`: Warning messages only
- `error`: Error messages only

**Step 4.1.2: View Logs in Real-Time**

```bash
# Systemd service
journalctl -u platter -f

# Docker container
docker logs -f platter

# Direct execution
./platter 2>&1 | tee oauth-debug.log
```

### 4.2: Analyse Log Patterns

**Successful OAuth flow logs:**

```
[INFO] OAuth config loaded successfully
[INFO] OAuth login initiated, CSRF token stored in session
[DEBUG] Redirecting to Microsoft: https://login.microsoftonline.com/...
[INFO] CSRF token validated successfully
[DEBUG] Token exchange successful
[INFO] Token subject validated: a1b2c3d4-...
[INFO] Token email claim validated: user@domain.com
[INFO] Email authorisation successful for: user@domain.com
[INFO] OAuth authentication and session creation successful
```

**Failed authentication logs:**

```
[ERROR] CSRF validation failed: token mismatch
[ERROR] Token validation failed: missing or empty subject identifier
[ERROR] Email not in allowed list: user@domain.com
[ERROR] Authorization failed: email not authorized
```

### 4.3: Debug Session Issues

**Step 4.3.1: Inspect Browser Cookies**

1. Open browser developer tools (F12)
2. Navigate to "Application" tab
3. Select "Cookies" in left sidebar
4. Check for session cookie
5. Verify cookie attributes:
   - `HttpOnly`: ✅ Should be set
   - `Secure`: ✅ Should be set (HTTPS only)
   - `SameSite`: ✅ Should be `Strict` or `Lax`

**Step 4.3.2: Verify Environment Variables**

```bash
# Check SESSION_SECRET is set
echo $SESSION_SECRET

# Verify PRODUCTION mode
echo $PRODUCTION
```

**Step 4.3.3: Check HTTPS Requirements**

Secure cookies require HTTPS in production:
- ✅ Development: HTTP allowed with `PRODUCTION=false`
- ❌ Production: HTTP not allowed with `PRODUCTION=true`

**Step 4.3.4: Verify Proxy Configuration**

If using reverse proxy, ensure it doesn't strip session cookies:

```nginx
# Nginx configuration
proxy_pass http://127.0.0.1:8080;
proxy_set_header Host $host;
proxy_set_header X-Forwarded-Proto $scheme;
proxy_cookie_path / "/; secure; HttpOnly; SameSite=Strict";
```

### 4.4: Network Analysis

**Step 4.4.1: Use Browser Network Tab**

1. Open developer tools (F12)
2. Go to "Network" tab
3. Attempt login
4. Examine requests:
   - `/oauth/login` - Should redirect to Microsoft
   - Microsoft authentication flow
   - `/auth_callback` - Should include `code` and `state` parameters

**Step 4.4.2: Verify Request Parameters**

Check `/auth_callback` request includes:
- `code`: Authorisation code from Microsoft
- `state`: CSRF protection token

**Step 4.4.3: Check Response Status Codes**

| Status Code | Meaning | Action |
|-------------|---------|--------|
| 302 | Redirect (normal) | Follow redirect chain |
| 401 | Unauthorised | Check authentication |
| 403 | Forbidden | Verify permissions |
| 500 | Server error | Check application logs |

### 4.5: Isolated Testing

**Step 4.5.1: Create Minimal Test Configuration**

```json
{
  "client_id": "your-client-id",
  "client_secret": "your-client-secret",
  "issuer_url": "https://login.microsoftonline.com/your-tenant-id/v2.0",
  "redirect_url": "http://localhost:8080/auth_callback",
  "allowed_emails": ["test@yourdomain.com"],
  "enabled": true
}
```

**Step 4.5.2: Test Locally First**

1. Use `http://localhost:8080/auth_callback` for local testing
2. Add this URI to Azure Portal redirect URIs
3. Test authentication locally
4. Once working, update to production URL

**Step 4.5.3: Test in Isolation**

```bash
# Stop production instance
systemctl stop platter

# Run with debug logging
RUST_LOG=debug ./platter

# Monitor output for errors
```

---

## 📊 Section 5: Log Messages Reference

### 5.1: Successful OAuth Flow

| Log Message | Stage | Meaning |
|-------------|-------|---------|
| `OAuth config loaded successfully` | Startup | Configuration file loaded correctly |
| `OAuth login initiated, CSRF token stored in session` | Login start | User clicked "Login with Microsoft" |
| `CSRF token validated successfully` | Callback | Security token matched (callback successful) |
| `Token subject validated: <id>` | Token verification | User identity verified from Microsoft |
| `Token email claim validated: <email>` | Token verification | Email extracted from token |
| `Email authorisation successful for: <email>` | Authorisation | User's email is in allowed list |
| `OAuth authentication and session creation successful` | Login complete | User authenticated successfully |
| `OAuth user logout, redirecting to OIDC logout endpoint` | Logout | Proper logout flow initiated |

### 5.2: Error Patterns

| Log Message | Issue | Solution |
|-------------|-------|----------|
| `Failed to load OAuth config: <error>` | Config file missing or invalid | Verify file exists with valid JSON |
| `CSRF validation failed: no token in session` | Session expired or cookies disabled | Clear cookies, enable cookies, retry |
| `CSRF validation failed: token mismatch` | Possible replay attack or session issue | Clear browser data, try incognito |
| `Token validation failed: missing or empty subject identifier` | Invalid token from Microsoft | Check Azure AD configuration |
| `Token validation failed: invalid email format` | Email claim missing or malformed | Verify email permission in Azure AD |
| `Authorization failed: email <email> not in allowed list` | User not authorised | Add email to `allowed_emails` |
| `Invalid auth URL: <error>` | Malformed issuer URL | Check `issuer_url` format |

---

## 📝 Section 6: Configuration Reference

### 6.1: Complete Configuration Example

**File:** `data/oauth_config.json`

```json
{
  "client_id": "12345678-1234-1234-1234-123456789abc",
  "client_secret": "your-client-secret-value-here",
  "issuer_url": "https://login.microsoftonline.com/your-tenant-id-here/v2.0",
  "redirect_url": "https://your-domain.com/auth_callback",
  "allowed_emails": [
    "admin@yourdomain.com",
    "user@yourdomain.com"
  ],
  "enabled": true
}
```

### 6.2: Field Descriptions

**`client_id`** *(required)*

- **Source:** Azure Portal → App registrations → Overview → Application (client) ID
- **Format:** UUID (Universally Unique Identifier) format `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`
- **Example:** `a1b2c3d4-e5f6-7890-abcd-ef1234567890`

**`client_secret`** *(required)*

- **Source:** Azure Portal → App registrations → Certificates & secrets → Client secrets → **Value**
- **Critical:** Use the secret **Value**, not the Secret ID
- **Security:** Never commit to version control
- **Expiration:** Check expiry date regularly

**`issuer_url`** *(required)*

- **Format:** `https://login.microsoftonline.com/{tenant-id}/v2.0`
- **Tenant ID source:** Azure Portal → App registrations → Overview → Directory (tenant) ID
- **Critical:** Must include `/v2.0` suffix
- **Example:** `https://login.microsoftonline.com/a1b2c3d4-e5f6-7890-abcd-ef1234567890/v2.0`

**`redirect_url`** *(required)*

- **Format:** `https://your-domain.com/auth_callback`
- **Critical:** Must exactly match Azure Portal configuration (including trailing slash)
- **Protocol:** Use `https://` in production, `http://localhost` for local testing
- **Example:** `https://platter.example.com/auth_callback`

**`allowed_emails`** *(required)*

- **Format:** Array of email addresses as strings
- **Comparison:** Case-insensitive
- **Purpose:** Controls who can access admin functions
- **Example:** `["admin@example.com", "user@example.com"]`

**`enabled`** *(required)*

- **Format:** Boolean (`true` or `false`)
- **Purpose:** Enable/disable OAuth without deleting configuration
- **Value:** Must be `true` to enable OAuth authentication

### 6.3: Common Configuration Mistakes

| Mistake | Problem | Correct Approach |
|---------|---------|------------------|
| Using client secret ID instead of value | Authentication fails | Copy the **Value** column, not the ID |
| Missing `/v2.0` in issuer URL | Token validation fails | Always include `/v2.0` at end |
| Redirect URL trailing slash mismatch | Redirect URI error | Match Azure Portal exactly |
| Wrong protocol (http vs https) | Security error or mismatch | Use https in production |
| Tenant ID in wrong position | Authentication fails | Place in issuer URL, not redirect URL |

---

## 🎯 Section 7: Next Steps

After resolving OAuth issues, proceed with these tasks:

### For System Administrators

Complete these tasks in sequence:

1. **Document Configuration** – Record all OAuth settings securely
2. **Set Up Monitoring** – Monitor for authentication failures
3. **Plan Secret Rotation** – Schedule client secret renewal
4. **Review Access List** – Regularly audit allowed emails

### For Security Teams

Complete these tasks in sequence:

1. **Audit Permissions** – Review Azure AD permissions granted
2. **Implement MFA** – Enable multi-factor authentication in Azure AD
3. **Monitor Logs** – Set up alerts for failed authentication attempts
4. **Test Disaster Recovery** – Verify OAuth recovery procedures

---

## 📖 Related Documentation

Access these resources for additional information:

- **[OAuth Setup Guide](../guides/oauth-setup.md)** – Initial OAuth configuration instructions
- **[Configuration Guide](../guides/configuration.md)** – Environment variables and settings
- **[Security Documentation](../architecture/security.md)** – Security settings and best practices
- **[Production Deployment Guide](../deployment/production.md)** – Production troubleshooting procedures

---

[← Back to Troubleshooting](README.md) | [Documentation Home](../README.md)