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
- **OAuth 2.0 Authentication**: Secure login with Microsoft Entra ID integration
- **RESTful API**: Programmatic access to all features
- **Security First**: Built-in security measures including Argon2 password hashing and PKCE

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

### OAuth 2.0 Authentication

The application supports OAuth 2.0 authentication with Microsoft Entra ID (Azure AD) for enhanced security. OAuth users are validated against an allowlist of email addresses.

#### Configuration

Create a `data/oauth_config.json` file with the following structure:

```json
{
  "client_id": "your-azure-app-client-id",
  "client_secret": "your-azure-app-client-secret",
  "issuer_url": "https://login.microsoftonline.com/your-tenant-id/v2.0",
  "redirect_url": "https://your-domain.com/auth_callback",
  "allowed_emails": ["admin@yourdomain.com", "user@yourdomain.com"],
  "enabled": true
}
```

#### Microsoft Entra ID Setup

1. Register an application in the [Azure Portal](https://portal.azure.com/)
2. Add `https://your-domain.com/auth_callback` as a redirect URI
3. Note the Application (client) ID and create a client secret
4. Use your tenant ID in the issuer URL

#### Features

- **PKCE Support**: Enhanced security for public clients
- **Email Whitelist**: Only specified emails can access admin functions
- **Fallback Authentication**: Username/password login remains available
- **Session Management**: OAuth tokens stored securely in sessions

### OAuth Troubleshooting

Having issues with OAuth authentication? This guide covers common problems and their solutions, organized from simple fixes to advanced debugging.

#### Quick Fixes

##### Login with Microsoft Button Not Appearing

**Symptoms**: The OAuth login button doesn't show on the login page.

**Solutions**:
1. Verify [`data/oauth_config.json`](data/oauth_config.json) exists in your project
2. Check that `"enabled": true` in the configuration file
3. Restart the application after making configuration changes
4. Clear browser cache and reload the page

##### Authentication Fails After Microsoft Redirect

**Symptoms**: Login redirects to Microsoft, but returns to login page or shows an error.

**Common Causes**:
- Email not in the allowed list
- CSRF token mismatch (session expired)
- Invalid redirect URI configuration

**Solutions**:
1. Verify your email is in the `allowed_emails` array in [`oauth_config.json`](data/oauth_config.json)
2. Try clearing browser cookies and sessions
3. Test in an incognito/private browsing window
4. Check application logs for specific error messages

##### Automatically Logged Back In After Logout

**Symptoms**: After logging out, you're immediately logged back in.

**Cause**: Microsoft Entra ID session is still active.

**Solution**: The application implements proper OIDC logout. If you're still experiencing this:
1. Ensure you're using the latest version with OIDC logout support
2. Clear all browser cookies for both your application and `login.microsoftonline.com`
3. Close all browser tabs and restart the browser

#### Common Error Messages

##### "Email not authorized for admin access"

**Cause**: Your email address is not in the allowed emails list.

**Solution**:
1. Add your email to the `allowed_emails` array in [`data/oauth_config.json`](data/oauth_config.json):
```json
{
  "allowed_emails": ["your-email@domain.com", "admin@domain.com"]
}
```
2. Restart the application
3. **Note**: Email comparison is case-insensitive

##### "CSRF token validation failed"

**Cause**: Security token mismatch, usually from an expired session or browser issue.

**Solutions**:
1. Clear browser cookies and try again
2. Ensure cookies are enabled in your browser
3. Try a different browser or incognito mode
4. Check if your reverse proxy/load balancer preserves sessions correctly

##### "OAuth is not enabled"

**Cause**: OAuth configuration is disabled or not loaded.

**Solutions**:
1. Verify `"enabled": true` in [`data/oauth_config.json`](data/oauth_config.json)
2. Check application logs for configuration loading errors
3. Ensure the configuration file has valid JSON syntax
4. Restart the application after enabling OAuth

##### "OAuth configuration not found"

**Cause**: The configuration file is missing or couldn't be loaded.

**Solutions**:
1. Create [`data/oauth_config.json`](data/oauth_config.json) if it doesn't exist
2. Verify file permissions (application must be able to read the file)
3. Check the file path is correct relative to the application binary
4. Review application startup logs for file system errors

##### Redirect URI Mismatch Errors

**Symptoms**: Error from Microsoft about redirect URI not matching.

**Solutions**:
1. Ensure the `redirect_url` in [`oauth_config.json`](data/oauth_config.json) exactly matches the URI registered in Azure Portal
2. Check for trailing slashes (must match exactly)
3. Verify the protocol (http vs https) matches
4. In Azure Portal: Go to "App registrations" → Your app → "Authentication" → Verify "Redirect URIs"

#### Configuration Verification

##### Local Configuration Checks

1. **Verify config file exists**:
   ```bash
   ls -la data/oauth_config.json
   cat data/oauth_config.json
   ```

2. **Validate JSON syntax**:
   ```bash
   python3 -m json.tool data/oauth_config.json
   # or
   jq '.' data/oauth_config.json
   ```

3. **Check required fields**:
   - `client_id`: Application (client) ID from Azure Portal
   - `client_secret`: Client secret value (not the ID)
   - `issuer_url`: Must be `https://login.microsoftonline.com/{tenant-id}/v2.0`
   - `redirect_url`: Must match Azure AD configuration exactly
   - `allowed_emails`: Array of authorized email addresses
   - `enabled`: Must be `true`

##### Azure Portal Verification

1. **Verify App Registration**:
   - Go to [Azure Portal](https://portal.azure.com/) → "App registrations"
   - Find your application
   - Note the "Application (client) ID" (use as `client_id`)
   - Note the "Directory (tenant) ID" (use in `issuer_url`)

2. **Check Redirect URI**:
   - In your app registration, go to "Authentication"
   - Under "Platform configurations" → "Web"
   - Verify redirect URI matches your `redirect_url` exactly
   - Format: `https://your-domain.com/auth_callback`

3. **Verify API Permissions**:
   - Go to "API permissions" in your app registration
   - Ensure these Microsoft Graph permissions are granted:
     - `openid` (Sign users in)
     - `email` (View users' email address)
     - `profile` (View users' basic profile)
   - Click "Grant admin consent" if required

4. **Validate Client Secret**:
   - Go to "Certificates & secrets"
   - Verify the secret hasn't expired
   - If expired, create a new secret and update [`oauth_config.json`](data/oauth_config.json)
   - **Important**: Use the secret **value**, not the secret ID

5. **Tenant ID Verification**:
   - From your app's Overview page, copy the "Directory (tenant) ID"
   - Your `issuer_url` should be: `https://login.microsoftonline.com/{tenant-id}/v2.0`
   - Replace `{tenant-id}` with your actual tenant ID

#### Advanced Debugging

##### Enable Detailed Logging

Set the `RUST_LOG` environment variable for verbose OAuth logs:

```bash
# Linux/macOS
export RUST_LOG=info
./platter

# Or for more detail
export RUST_LOG=debug
./platter

# Windows PowerShell
$env:RUST_LOG="info"
.\platter.exe
```

##### Check Application Logs

Look for these log patterns:

**Successful OAuth flow**:
```
OAuth login initiated, CSRF token stored in session
CSRF token validated successfully
Token subject validated: <subject-id>
Token email claim validated: user@domain.com
Email authorization successful for: user@domain.com
OAuth authentication and session creation successful
```

**Failed authentication**:
```
CSRF validation failed: token mismatch
Token validation failed: missing or empty subject identifier
Email not in allowed list
```

##### Debug Session Issues

If sessions aren't persisting:

1. Check cookie settings in browser developer tools (F12)
2. Verify `SESSION_SECRET` environment variable is set in production
3. Check for secure cookie issues (HTTPS required in production)
4. Ensure no reverse proxy is stripping session cookies

##### Browser Developer Tools

1. Open developer tools (F12)
2. Go to "Network" tab
3. Attempt login
4. Look for:
   - `/oauth/login` request (should redirect to Microsoft)
   - `/auth_callback` request (should have `state` parameter)
   - Check for 4xx/5xx errors
   - Verify cookies are being set

##### Test in Isolation

Create a minimal test config:

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

Test locally first before deploying to production.

#### Log Messages Reference

##### Successful OAuth Flow

| Log Message | Meaning |
|-------------|---------|
| `OAuth config loaded successfully` | Configuration file loaded at startup |
| `OAuth login initiated, CSRF token stored in session` | User clicked "Login with Microsoft" |
| `CSRF token validated successfully` | Security token matched (callback successful) |
| `Token subject validated: <id>` | User identity verified from Microsoft |
| `Token email claim validated: <email>` | Email extracted from token |
| `Email authorization successful for: <email>` | User's email is in allowed list |
| `OAuth authentication and session creation successful` | Login complete, user authenticated |
| `OAuth user logout, redirecting to OIDC logout endpoint` | Proper logout flow initiated |

##### Common Error Patterns

| Log Message | Issue | Solution |
|-------------|-------|----------|
| `Failed to load OAuth config: <error>` | Config file missing or invalid | Verify file exists and has valid JSON |
| `CSRF validation failed: no token in session` | Session expired or cookies disabled | Clear cookies, enable cookies, try again |
| `CSRF validation failed: token mismatch` | Possible replay attack or session issue | Clear browser data, try incognito mode |
| `Token validation failed: missing or empty subject identifier` | Invalid token from Microsoft | Check Azure AD configuration |
| `Token validation failed: invalid email format` | Email claim missing or malformed | Verify email permission in Azure AD |
| `Authorization failed: email <email> not in allowed list` | User not authorized | Add email to `allowed_emails` in config |
| `Invalid auth URL: <error>` | Malformed issuer URL | Check `issuer_url` format in config |

#### Configuration Reference

Complete example of [`data/oauth_config.json`](data/oauth_config.json) with explanations:

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

**Field Descriptions**:

- **`client_id`** *(required)*: Application (client) ID from Azure Portal → App registrations → Overview
- **`client_secret`** *(required)*: Client secret **value** (not ID) from Certificates & secrets
- **`issuer_url`** *(required)*: Must be `https://login.microsoftonline.com/{tenant-id}/v2.0` where `{tenant-id}` is your Directory (tenant) ID
- **`redirect_url`** *(required)*: Must exactly match the redirect URI configured in Azure Portal. Format: `https://your-domain.com/auth_callback`
- **`allowed_emails`** *(required)*: Array of email addresses authorized for admin access. Comparison is case-insensitive
- **`enabled`** *(required)*: Must be `true` to enable OAuth. Set to `false` to disable without deleting the configuration

**Common Configuration Mistakes**:
- Using client secret ID instead of the secret value
- Missing `/v2.0` in the issuer URL
- Redirect URL with/without trailing slash mismatch
- Wrong protocol (http vs https) in redirect URL
- Tenant ID in wrong format or position

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
