# 🔐 OAuth Setup Guide

> **Navigation:** [Documentation Home](../README.md) → [Getting Started](README.md) → OAuth Setup

**Reading Time:** Approximately 20 minutes  
**Complexity:** Intermediate to Advanced  
**Prerequisites:** Access to Microsoft Azure Portal with administrative permissions

## 📖 What This Guide Covers

This guide explains how to configure Microsoft Entra ID (formerly Azure AD) OAuth authentication for Platter. By the end of this guide, you will have:

1. Created an application registration in Microsoft Entra ID
2. Configured the required OAuth settings
3. Created the OAuth configuration file
4. Tested the OAuth authentication flow
5. Understood common troubleshooting steps

---

## 📑 Table of Contents

Navigate to specific sections:

- [Section 1: Overview](#-section-1-overview)
- [Section 2: Prerequisites](#-section-2-prerequisites)
- [Section 3: Azure AD Configuration](#-section-3-azure-ad-configuration)
- [Section 4: Application Configuration](#-section-4-application-configuration)
- [Section 5: Testing OAuth](#-section-5-testing-oauth)
- [Section 6: Troubleshooting](#-section-6-troubleshooting)

---

## 🔍 Section 1: Overview

This section explains what OAuth provides and how Platter uses it.

### What OAuth Authentication Provides

OAuth 2.0 authentication offers these security benefits:

1. **Centralised Authentication** – Users authenticate with Microsoft accounts
2. **Email-Based Authorisation** – Access controlled by email whitelist
3. **PKCE Security** – Enhanced security for public clients using Proof Key for Code Exchange
4. **Fallback Authentication** – Username/password login remains available as backup
5. **Secure Token Management** – OAuth tokens stored securely in sessions

### Key Features

The OAuth implementation includes:

1. **PKCE Support** – Prevents authorisation code interception attacks
2. **Email Whitelist** – Only specified email addresses can access admin functions
3. **Fallback Authentication** – Traditional login remains available if OAuth fails
4. **Session Management** – OAuth tokens stored securely in encrypted sessions

---

## 📝 Section 2: Prerequisites

Before configuring OAuth, ensure you have the following requirements.

### Required Access and Resources

You need:

1. **Microsoft Entra ID Tenant** – An Azure AD organisation
2. **Administrative Access** – Permissions to create app registrations in Azure Portal
3. **Public Domain or Localhost** – Either a production domain or localhost for testing
4. **Email Addresses** – List of authorised user email addresses

---

## ☁️ Section 3: Azure AD Configuration

This section guides you through configuring Microsoft Entra ID.

### Step 3.1: Register Application in Azure Portal

Follow these steps in sequence:

#### Action 1: Access Azure Portal

1. Navigate to [Azure Portal](https://portal.azure.com/) in your web browser
2. Sign in with your Microsoft account
3. Wait for the portal dashboard to load

#### Action 2: Navigate to App Registrations

1. In the left sidebar, select **Azure Active Directory** (or **Microsoft Entra ID**)
2. In the left menu, select **App registrations**
3. The app registrations list displays

#### Action 3: Create New Registration

1. Click the **New registration** button at the top
2. The registration form displays

#### Action 4: Fill Registration Details

Complete the form with these values:

**Field: Name**
- Enter: `Platter` (or your preferred application name)
- Purpose: This name appears to users during authentication

**Field: Supported account types**
- Select the appropriate option based on your requirements:
  - **Single tenant** – Only users in your organisation
  - **Multi-tenant** – Users in any organisation
  - **Personal accounts** – Include personal Microsoft accounts

**Field: Redirect URI**
- Platform: Select **Web** from the dropdown
- URI: Enter one of the following:
  - Production: `https://your-domain.com/auth_callback`
  - Testing: `http://localhost:8080/auth_callback`

> 📘 **Important:** The redirect URI must match exactly in both Azure and your application configuration.

#### Action 5: Complete Registration

1. Click the **Register** button
2. Wait for the registration to complete
3. The application overview page displays

### Step 3.2: Record Application Details

After registration, you need to record specific values.

#### Value 1: Application (Client) ID

**Location:** Application overview page

**Steps to copy:**
1. Locate the **Application (client) ID** field
2. Click the copy icon next to the value
3. Store this value securely (you will use it as `client_id`)

**Example format:** `12345678-1234-1234-1234-123456789abc`

#### Value 2: Directory (Tenant) ID

**Location:** Application overview page

**Steps to copy:**
1. Locate the **Directory (tenant) ID** field
2. Click the copy icon next to the value
3. Store this value securely (you will use it in `issuer_url`)

**Example format:** `abcdef12-3456-7890-abcd-ef1234567890`

### Step 3.3: Create Client Secret

Follow these steps to generate an authentication secret.

#### Action 1: Navigate to Certificates & Secrets

1. In your app registration, select **Certificates & secrets** from the left menu
2. The secrets management page displays

#### Action 2: Create New Secret

1. Under **Client secrets**, click **New client secret**
2. The creation form displays

#### Action 3: Configure Secret

Complete the form with these values:

**Field: Description**
- Enter: `Platter production secret` (or descriptive name)
- Purpose: Helps identify the secret later

**Field: Expires**
- Select appropriate expiration period:
  - Recommended: **6 months** or **12 months** for production
  - Maximum: **24 months**

#### Action 4: Save and Copy Secret

1. Click the **Add** button
2. The secret displays with a **Value** column
3. **Critical:** Click the copy icon in the **Value** column immediately
4. Store this value securely (you will use it as `client_secret`)

> ⚠️ **Critical Warning:** The secret value displays only once. After you navigate away, you cannot retrieve it. If lost, you must create a new secret.

**What to copy:**
- ✅ Copy the **Value** column (long alphanumeric string)
- ❌ Do not copy the **Secret ID** column (this is not the secret)

### Step 3.4: Configure API Permissions

This section configures the data your application can access.

#### Action 1: Navigate to API Permissions

1. In your app registration, select **API permissions** from the left menu
2. The permissions page displays

#### Action 2: Add Microsoft Graph Permissions

Follow these steps:

1. Click **Add a permission**
2. Select **Microsoft Graph** from the options
3. Select **Delegated permissions**
4. Add these three permissions in sequence:

**Permission 1: openid**
- **Purpose:** Enables users to sign in
- **Required:** Yes
- **Locate:** Search for "openid" in the permission list
- **Action:** Tick the checkbox next to "openid"

**Permission 2: email**
- **Purpose:** Allows reading users' email addresses
- **Required:** Yes
- **Locate:** Search for "email" in the permission list
- **Action:** Tick the checkbox next to "email"

**Permission 3: profile**
- **Purpose:** Allows reading users' basic profile information
- **Required:** Yes
- **Locate:** Search for "profile" in the permission list
- **Action:** Tick the checkbox next to "profile"

#### Action 3: Complete Permission Addition

1. Click the **Add permissions** button
2. The permissions display in the permissions list

#### Action 4: Grant Admin Consent

**If you have admin rights:**
1. Click **Grant admin consent for [organisation]**
2. Confirm the action
3. The **Status** column changes to "Granted for [organisation]"

**If you do not have admin rights:**
1. Request an administrator to grant consent
2. Provide them with the application registration link

### Step 3.5: Verify Redirect URI

This section confirms the redirect URI configuration.

#### Action 1: Navigate to Authentication

1. In your app registration, select **Authentication** from the left menu
2. The authentication configuration page displays

#### Action 2: Verify Redirect URIs

Under **Platform configurations** → **Web**, verify:

1. Your redirect URI is listed
2. The URI matches exactly what you will use in the application
3. The protocol (http or https) is correct

**Correct examples:**
- ✅ `https://yourdomain.com/auth_callback`
- ✅ `http://localhost:8080/auth_callback`

**Incorrect examples:**
- ❌ `https://yourdomain.com/auth_callback/` (trailing slash)
- ❌ `http://yourdomain.com/auth_callback` (wrong protocol)

#### Action 3: Configure Token Settings (Optional)

Under **Implicit grant and hybrid flows**:

1. **ID tokens** – Can be enabled for some configurations (optional)
2. **Access tokens** – Leave unchecked for this implementation

#### Action 4: Save Changes

1. If you made changes, click **Save** at the top
2. Wait for the confirmation message

---

## ⚙️ Section 4: Application Configuration

This section explains how to configure Platter to use OAuth.

### Step 4.1: Create OAuth Configuration File

Create the file `data/oauth_config.json` in your Platter installation directory.

#### Configuration File Template

Copy this template and replace the placeholder values:

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

### Step 4.2: Configure Field Values

Replace each field with your actual values:

#### Field: client_id

**Source:** Application (client) ID from Azure Portal  
**Example:** `12345678-1234-1234-1234-123456789abc`  
**Format:** UUID (36 characters with hyphens)

**How to obtain:**
1. Go to your app registration in Azure Portal
2. Copy the **Application (client) ID** from the overview page

#### Field: client_secret

**Source:** Client secret value from Certificates & secrets  
**Example:** `abc123def456...` (long alphanumeric string)  
**Format:** Long alphanumeric string

> ⚠️ **Critical:** Use the secret **value**, not the secret ID.

**How to obtain:**
1. Go to **Certificates & secrets** in your app registration
2. Copy the **Value** from the client secrets table

#### Field: issuer_url

**Format:** Must be exactly `https://login.microsoftonline.com/{tenant-id}/v2.0`  
**Example:** `https://login.microsoftonline.com/abcd1234-5678-90ef-ghij-klmnopqrstuv/v2.0`

**How to construct:**
1. Start with: `https://login.microsoftonline.com/`
2. Add your Directory (tenant) ID
3. End with: `/v2.0`

> 📘 **Important:** The `/v2.0` suffix is required. Do not omit it.

#### Field: redirect_url

**Format:** Must exactly match Azure AD configuration  
**Examples:**
- Production: `https://yourdomain.com/auth_callback`
- Local testing: `http://localhost:8080/auth_callback`

**Requirements:**
1. Must match the redirect URI in Azure Portal exactly
2. Must include the protocol (`http://` or `https://`)
3. Must not include trailing slashes
4. Must use the correct port number

#### Field: allowed_emails

**Format:** Array of email addresses (JSON array)  
**Example:** `["admin@example.com", "user@example.com"]`

**Purpose:** Only users with email addresses in this list can access admin functions

**Rules:**
1. Email comparison is case-insensitive (`Admin@Example.com` matches `admin@example.com`)
2. Must be valid email addresses
3. Can include multiple addresses
4. Array can be empty but must be present

#### Field: enabled

**Format:** Boolean (`true` or `false`)  
**Purpose:** Enables or disables OAuth authentication

**Values:**
- `true` – OAuth login button displays, OAuth authentication active
- `false` – OAuth disabled, only username/password login available

### Step 4.3: Validate Configuration

Execute this command to validate JSON syntax:

```bash
# Validate JSON structure
jq '.' data/oauth_config.json
```

**Expected output:** Formatted JSON with no errors

**Alternative validation (if jq is not installed):**
```bash
python3 -m json.tool data/oauth_config.json
```

### Step 4.4: Check File Permissions

Execute this command on Linux/macOS:

```bash
# View file permissions
ls -la data/oauth_config.json
```

**Expected permissions:** `-rw-r--r--` (644) or more restrictive

**If permissions are incorrect:**
```bash
chmod 644 data/oauth_config.json
```

---

## 🧪 Section 5: Testing OAuth

This section explains how to test the OAuth configuration.

### Test 5.1: Local Testing

Use these steps to test OAuth on your local development system.

#### Step 5.1.1: Configure for Localhost

Create or modify `data/oauth_config.json` with localhost settings:

```json
{
  "client_id": "your-client-id",
  "client_secret": "your-client-secret",
  "issuer_url": "https://login.microsoftonline.com/your-tenant-id/v2.0",
  "redirect_url": "http://localhost:8080/auth_callback",
  "allowed_emails": ["your-test-email@example.com"],
  "enabled": true
}
```

> 📘 **Note:** Ensure `http://localhost:8080/auth_callback` is registered in Azure Portal.

#### Step 5.1.2: Start the Application

Execute this command:

```bash
cargo run
```

**Expected output:**
- Server starts on port 8080
- "OAuth config loaded successfully" message displays

#### Step 5.1.3: Test Authentication Flow

Follow these steps in sequence:

1. **Action:** Navigate to `http://localhost:8080/admin/login`
   - **Expected:** Login page displays

2. **Action:** Verify "Login with Microsoft" button is visible
   - **Expected:** Blue Microsoft login button displays
   - **If not visible:** Check `"enabled": true` in config file

3. **Action:** Click "Login with Microsoft" button
   - **Expected:** Browser redirects to Microsoft login page

4. **Action:** Sign in with your Microsoft account
   - **Expected:** Microsoft authentication page displays
   - **Note:** Use an account with email in `allowed_emails`

5. **Action:** Complete Microsoft authentication
   - **Expected:** Browser redirects to `http://localhost:8080/auth_callback`
   - **Then:** Automatically redirects to admin dashboard

6. **Action:** Verify successful login
   - **Expected:** Admin dashboard displays
   - **Expected:** Your session is active

### Test 5.2: Production Testing

Use these steps to test OAuth in a production environment.

#### Step 5.2.1: Configure Production Settings

Update `data/oauth_config.json` with production values:

```json
{
  "client_id": "your-client-id",
  "client_secret": "your-client-secret",
  "issuer_url": "https://login.microsoftonline.com/your-tenant-id/v2.0",
  "redirect_url": "https://yourdomain.com/auth_callback",
  "allowed_emails": ["admin@yourdomain.com"],
  "enabled": true
}
```

#### Step 5.2.2: Test in Private Browsing Mode

**Why use private browsing:**
- Avoids cached credentials
- Tests the complete authentication flow
- Simulates first-time user experience

**Steps:**
1. Open private/incognito browser window
2. Navigate to your production login page
3. Test the complete OAuth flow

#### Step 5.2.3: Verification Checklist

Verify these items:

- [ ] Login button displays correctly
- [ ] Microsoft redirect functions
- [ ] Authentication completes successfully
- [ ] Admin access is granted
- [ ] Session persists correctly

---

## 🔧 Section 6: Configuration Best Practices

Follow these security and operational best practices.

### Security Best Practices

#### Practice 1: Use HTTPS in Production

**Requirement:** Always use HTTPS for production deployments

**Why this matters:**
- OAuth tokens transmitted in URL parameters
- Session cookies marked as Secure require HTTPS
- Microsoft requires HTTPS for production apps

**Implementation:**
```json
{
  "redirect_url": "https://yourdomain.com/auth_callback"
}
```

#### Practice 2: Protect Client Secrets

**Requirements:**
1. ✅ Never commit secrets to version control
2. ✅ Store secrets in environment variables or secure vaults
3. ✅ Rotate secrets periodically (every 90 days recommended)
4. ✅ Use different secrets for different environments

#### Practice 3: Maintain Minimal Email Whitelist

**Best practice:** Include only necessary email addresses

**Example configuration:**
```json
{
  "allowed_emails": [
    "admin@example.com",     // Primary admin
    "backup@example.com"     // Backup admin
  ]
}
```

**Operational guidelines:**
1. Email comparison is case-insensitive
2. Use work/organisational email addresses
3. Keep the list minimal (reduce attack surface)
4. Document who has access and why

#### Practice 4: Regular Audits

**Schedule regular reviews:**
1. Review `allowed_emails` list monthly
2. Audit OAuth access logs quarterly
3. Rotate client secrets every 90 days
4. Update redirect URIs when domains change

### Multiple Environment Configuration

Manage different configurations for different environments.

#### Development Environment

**File:** `data/oauth_config.dev.json`

```json
{
  "client_id": "dev-client-id",
  "client_secret": "dev-client-secret",
  "issuer_url": "https://login.microsoftonline.com/tenant-id/v2.0",
  "redirect_url": "http://localhost:8080/auth_callback",
  "allowed_emails": ["dev@example.com"],
  "enabled": true
}
```

**Use case:** Local development and testing

#### Production Environment

**File:** `data/oauth_config.json`

```json
{
  "client_id": "prod-client-id",
  "client_secret": "prod-client-secret",
  "issuer_url": "https://login.microsoftonline.com/tenant-id/v2.0",
  "redirect_url": "https://yourdomain.com/auth_callback",
  "allowed_emails": ["admin@example.com"],
  "enabled": true
}
```

**Use case:** Live production environment

---

## 🔍 Section 7: Troubleshooting

This section provides quick diagnostics for common issues.

### Quick Diagnostic Checklist

If OAuth is not working, check these items in sequence:

#### Check 1: Configuration File Exists

Execute this command:
```bash
ls data/oauth_config.json
```

**Expected:** File exists and displays

#### Check 2: Valid JSON

Execute this command:
```bash
jq '.' data/oauth_config.json
```

**Expected:** Formatted JSON with no errors

#### Check 3: OAuth Enabled

Execute this command:
```bash
jq '.enabled' data/oauth_config.json
```

**Expected:** Output displays `true`

#### Check 4: Email in Whitelist

Verify your email is in the allowed list:
```bash
jq '.allowed_emails' data/oauth_config.json
```

**Expected:** Your email appears in the array

#### Check 5: Redirect URI Matches

Verify redirect URL matches Azure configuration:
```bash
jq '.redirect_url' data/oauth_config.json
```

**Expected:** Matches Azure Portal redirect URI exactly

### Common Issues Quick Fixes

#### Issue: Login Button Not Appearing

**Symptoms:**
- OAuth login button missing from login page

**Quick fixes to try:**
1. Verify `"enabled": true` in configuration file
2. Restart the application
3. Clear browser cache and reload

#### Issue: Authentication Fails After Redirect

**Symptoms:**
- Login redirects to Microsoft
- Returns to login page or shows error

**Quick fixes to try:**
1. Verify email is in `allowed_emails` array
2. Clear browser cookies
3. Test in private/incognito window
4. Check application logs for error messages

#### Issue: CSRF Token Errors

**Symptoms:**
- "CSRF token validation failed" error message

**Quick fixes to try:**
1. Clear all browser cookies
2. Ensure cookies are enabled in browser
3. Try a different browser
4. Check reverse proxy preserves cookies

> 📘 **Detailed Troubleshooting:** For comprehensive troubleshooting steps, refer to the [OAuth Troubleshooting Guide](../troubleshooting/oauth.md).

---

## 📖 Related Documentation

Access these resources for additional information:

- **[OAuth Troubleshooting Guide](../troubleshooting/oauth.md)** – Detailed troubleshooting procedures and log analysis
- **[Configuration Guide](configuration.md)** – Environment variables and system settings
- **[Security Documentation](../architecture/security.md)** – Security best practices and guidelines
- **[Getting Started Guide](getting-started.md)** – Initial installation and setup

---

[← Back to Getting Started](README.md) | [Documentation Home](../README.md)