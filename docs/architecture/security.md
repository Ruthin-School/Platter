# 🛡️ Security Architecture Documentation

> **Navigation:** [Documentation Home](../README.md) → [Architecture](README.md) → Security

**Reading Time:** Approximately 15 minutes  
**Complexity:** Intermediate  
**Prerequisites:** Understanding of web application security concepts

## 📖 What This Guide Covers

This guide explains the security architecture and implementation of Platter. By the end of this guide, you will understand:

1. The security features implemented in the application
2. How to configure security settings correctly
3. Security best practices for deployment
4. How to report security vulnerabilities
5. The production security checklist

---

## 📋 Section 1: Security Features Overview

Platter implements multiple layers of security protection to safeguard user data and maintain application integrity.

### 1.1: Security Layer Summary

| Security Layer | Implementation | Purpose |
|---------------|----------------|---------|
| Authentication | Argon2 password hashing | Verify user identity |
| Authorisation | Role-based access control (RBAC) | Control resource access |
| Data Protection | Input validation + output encoding | Prevent injection attacks |
| Network Security | Secure cookies + CORS + CSRF protection | Protect data in transit |

---

## 🔐 Section 2: Authentication and Authorisation

### 2.1: Password Security

**Argon2 Password Hashing:**

The application uses Argon2, an industry-standard password hashing algorithm.

**Algorithm configuration:**
- **Type:** Argon2id (hybrid mode combining Argon2i and Argon2d)
- **Memory cost:** 65536 KiB (Kibibytes) = 64 MiB (Mebibytes)
- **Time cost:** 3 iterations
- **Parallelism:** 4 threads
- **Output length:** 32 bytes
- **Salt length:** 32 bytes (randomly generated per password)

**How Argon2 protects passwords:**

1. When a user creates a password, the system generates a random salt
2. The system combines the password and salt using Argon2
3. The system stores only the resulting hash (not the original password)
4. When the user logs in, the system hashes the submitted password with the same salt
5. The system compares the new hash with the stored hash
6. Access is granted only if the hashes match

**Why this approach is secure:**
- **Impossible to reverse:** Cannot derive original password from the hash
- **Unique salts:** Identical passwords produce different hashes
- **Memory-hard:** Resistant to GPU (Graphics Processing Unit) and ASIC (Application-Specific Integrated Circuit) attacks
- **Time-consuming:** Deliberately slow to prevent brute-force attacks

### 2.2: Session Management

**Secure Session Configuration:**

**Session cookie attributes:**
- `HttpOnly`: `true` (prevents JavaScript access to cookie)
- `Secure`: `true` (requires HTTPS in production)
- `SameSite`: `Strict` (prevents cross-site cookie transmission)
- `Max-Age`: 86400 seconds (24 hours, configurable)

**Session lifecycle:**

1. **Creation:** Session created after successful authentication
2. **Encryption:** Session data encrypted with AES-256 (Advanced Encryption Standard)
3. **Storage:** Encrypted cookie sent to browser
4. **Validation:** Cookie verified on each request
5. **Expiration:** Automatic logout after 24 hours
6. **Termination:** Manual logout clears session immediately

### 2.4: Role-Based Access Control

**Access levels defined:**

| Role | Permissions | Restrictions |
|------|------------|--------------|
| **Administrator** | Full access to all features | Must authenticate via admin credentials |
| **Public User** | View menu and notices only | Cannot access admin features or API write operations |

**Enforcement mechanism:**
- Middleware checks user role before processing requests
- Protected routes return HTTP 403 error if insufficient permissions
- API endpoints validate authentication before data operations

---

## 🔒 Section 3: Data Protection

### 3.1: Input Validation

**Validation layers:**

**Layer 1: Client-side validation (optional, for user convenience)**
- HTML5 form validation attributes
- JavaScript validation for immediate feedback
- **Not relied upon for security**

**Layer 2: Server-side validation (required, for security)**
- Type checking using Rust's strong type system
- Length validation for all string inputs
- Format validation (email addresses, URLs, dates)
- Business rule validation
- **Always enforced regardless of client-side validation**

**Validation rules by field type:**

| Field Type | Validation Rules | Example |
|-----------|------------------|---------|
| Email | RFC 5322 format compliance | `user@example.com` |
| URL | Valid protocol and domain | `https://example.com` |
| Date | ISO 8601 format | `2025-01-01T00:00:00Z` |
| String | Minimum and maximum length | `1-200 characters` |
| Number | Range validation | `0.00-999.99` |

### 3.2: Output Encoding

**Prevention of XSS (Cross-Site Scripting) attacks:**

All user-generated content is encoded before display.

**Encoding strategy:**

1. **HTML Context:** Escape `<`, `>`, `&`, `"`, `'` characters
2. **JavaScript Context:** Use JSON encoding with proper escaping
3. **URL Context:** URL-encode special characters
4. **CSS Context:** Escape special CSS characters

**Implementation:**
- Tera template engine automatically escapes output by default
- Manual escaping applied when raw output is intentionally required
- Content Security Policy header provides additional XSS protection

### 3.3: SQL Injection Prevention

**Current implementation:**
- Application uses JSON file storage (no SQL database)
- No SQL injection risk in current architecture

**Future database migration considerations:**
- Use parameterised queries exclusively
- Employ ORM (Object-Relational Mapping) with built-in protection
- Never concatenate user input into SQL statements

---

## 🌐 Section 4: Network Security

### 4.1: Secure HTTP Configuration

**Cookie Security Flags:**

```rust
Cookie::build("session", encrypted_value)
    .http_only(true)      // Prevents JavaScript access
    .secure(true)         // Requires HTTPS in production
    .same_site(SameSite::Strict)  // Prevents CSRF
    .max_age(Duration::hours(24))  // 24-hour expiration
    .finish()
```

**What each flag does:**

- **HttpOnly:** Browser prevents JavaScript from reading the cookie value
  - **Prevents:** XSS attacks from stealing session tokens
  
- **Secure:** Browser sends cookie only over HTTPS connections
  - **Prevents:** Session hijacking via network eavesdropping
  
- **SameSite=Strict:** Browser sends cookie only for same-site requests
  - **Prevents:** CSRF attacks from malicious third-party sites

### 4.2: CORS (Cross-Origin Resource Sharing) Policy

**Development mode configuration:**

```
Allowed Origins:
- http://localhost:8080
- http://127.0.0.1:8080
```

**Production mode configuration:**

Set via `CORS_ALLOWED_ORIGINS` environment variable:

```bash
CORS_ALLOWED_ORIGINS=https://yourdomain.com,https://admin.yourdomain.com
```

**CORS validation process:**

1. Browser sends preflight OPTIONS request with Origin header
2. Server checks Origin against allowed list
3. If allowed, server responds with Access-Control-Allow-Origin header
4. If not allowed, server rejects with HTTP 403

### 4.3: CSRF (Cross-Site Request Forgery) Protection

**Token-based protection:**

1. **Token Generation:**
   - Cryptographically secure random value (32 bytes)
   - Generated per session
   - Stored in session cookie

2. **Token Validation:**
   - Required for all state-changing operations (POST, PUT, DELETE)
   - Token must match session token
   - Request rejected if token missing or invalid

3. **Token Inclusion:**
   - Embedded in forms as hidden field
   - Included in AJAX request headers
   - Verified before processing request

### 4.4: Security Headers

**HTTP response headers for additional protection:**

| Header | Value | Protection Against |
|--------|-------|-------------------|
| `X-Content-Type-Options` | `nosniff` | MIME type sniffing attacks |
| `X-Frame-Options` | `DENY` | Clickjacking attacks |
| `X-XSS-Protection` | `1; mode=block` | Legacy XSS attacks |
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | Protocol downgrade attacks |
| `Content-Security-Policy` | Restrictive policy | XSS and data injection attacks |

**Content Security Policy example:**

```
Content-Security-Policy: 
  default-src 'self'; 
  script-src 'self'; 
  style-src 'self' 'unsafe-inline'; 
  img-src 'self' data:; 
  connect-src 'self'
```

**Policy explanation:**
- `default-src 'self'`: Only load resources from same origin
- `script-src 'self'`: Only execute scripts from same origin
- `style-src 'self' 'unsafe-inline'`: Allow same-origin and inline styles
- `img-src 'self' data:`: Allow same-origin images and data URIs
- `connect-src 'self'`: Only connect to same origin

---

## ⚙️ Section 5: Security Configuration

### 5.1: Environment Variables for Security

**Required for production deployment:**

| Variable | Purpose | Example | Security Impact |
|----------|---------|---------|----------------|
| `SESSION_SECRET` | Session encryption key | `$(openssl rand -base64 64)` | **Critical** - Weak secret compromises all sessions |
| `PRODUCTION` | Enable production security mode | `true` | **High** - Enables stricter security policies |
| `CORS_ALLOWED_ORIGINS` | Allowed cross-origin requests | `https://yourdomain.com` | **High** - Prevents unauthorised API access |

**How to generate secure SESSION_SECRET:**

**Method 1: Using OpenSSL (recommended)**
```bash
openssl rand -base64 64
```

**Method 2: Using Rust**
```bash
cargo install random-generator
random-generator --length 64 --base64
```

**Method 3: Using Python**
```bash
python3 -c "import secrets; print(secrets.token_urlsafe(64))"
```

⚠️ **Critical:** Never commit `SESSION_SECRET` to version control. Use environment files (.env) excluded from Git.

---

## 📋 Section 6: Best Practices

### 6.1: For Administrators

Execute these security practices:

**Practice 6.1.1: Change Default Credentials**

Default credentials (`admin`/`admin123`) must be changed before production deployment.

**Steps to change:**

1. Access admin panel using default credentials
2. Navigate to user management section
3. Create new administrator account with strong password
4. Log out and verify new account works
5. Delete or disable default admin account

**Strong password requirements:**
- Minimum 12 characters
- Include uppercase and lowercase letters
- Include numbers
- Include special characters
- Avoid dictionary words
- Avoid personal information

**Practice 6.1.2: Regular Security Updates**

Execute these tasks monthly:

1. Check for Rust security advisories: `cargo audit`
2. Update dependencies: `cargo update`
3. Review security logs for suspicious activity
4. Verify backup integrity
5. Test disaster recovery procedures

**Practice 6.1.3: HTTPS in Production**

**Why HTTPS is essential:**
- Encrypts data in transit
- Prevents session hijacking
- Enables secure cookie flag
- Required for modern browser features

**Implementation options:**

**Option A: Let's Encrypt (Free)**
```bash
# Install certbot
sudo apt install certbot

# Obtain certificate
sudo certbot certonly --standalone -d yourdomain.com
```

**Option B: Commercial Certificate**
- Purchase from certificate authority
- Follow provider's installation instructions

**Option C: Reverse Proxy with SSL/TLS**
- Use nginx or Apache as reverse proxy
- Configure SSL/TLS at proxy level
- Forward to Platter application

### 6.2: For Developers

Execute these security practices:

**Practice 6.2.1: Environment Security**

**Secure secret management:**

1. Use `.env` files for local development
2. Add `.env` to `.gitignore`
3. Document required variables in `.env.example`
4. Use secret management services in production (e.g., AWS Secrets Manager, HashiCorp Vault)

**Example `.env.example` file:**
```bash
# Session configuration
SESSION_SECRET=generate-with-openssl-rand

# Production mode
PRODUCTION=true

# CORS configuration
CORS_ALLOWED_ORIGINS=https://yourdomain.com
```

**Practice 6.2.2: Secure Coding Guidelines**

1. **Input Validation:**
   - Validate all inputs server-side
   - Use type-safe parsing
   - Reject invalid data immediately
   - Provide clear error messages

2. **Error Handling:**
   - Never expose stack traces to users
   - Log detailed errors server-side only
   - Return generic error messages to clients
   - Monitor error logs for security issues

3. **Dependency Management:**
   - Regularly run `cargo audit`
   - Review dependency security advisories
   - Update dependencies promptly
   - Minimise dependency count

**Practice 6.2.3: Principle of Least Privilege**

**Application permissions:**
- Run application with non-root user account
- Limit file system access to required directories
- Use read-only mounts where possible
- Restrict network access to required ports only

**Example systemd service configuration:**
```ini
[Service]
User=platter
Group=platter
ReadOnlyPaths=/usr /lib /lib64
ReadWritePaths=/var/lib/platter/data
NoNewPrivileges=true
PrivateTmp=true
```

---

## 🚨 Section 7: Security Reporting

### 7.1: Reporting Vulnerabilities

**If you discover a security vulnerability:**

**Step 1: Do NOT create a public GitHub issue**

**Step 2: Contact maintainers directly**

Send encrypted email to: [project maintainer email]

**Step 3: Include these details:**

1. **Vulnerability description:** Clear explanation of the issue
2. **Steps to reproduce:** Detailed reproduction instructions
3. **Impact assessment:** Potential security impact
4. **Suggested fix:** Proposed solution (if available)
5. **Disclosure timeline:** Your intended disclosure timeline

**What happens next:**

1. **Acknowledgement:** Within 48 hours
2. **Assessment:** Within 7 days
3. **Fix development:** Timeline provided after assessment
4. **Testing:** Verification of fix
5. **Disclosure:** Co-ordinated public disclosure

### 7.2: Known Security Considerations

**Current architectural limitations:**

1. **JSON File Storage:**
   - Not encrypted at rest by default
   - **Mitigation:** Use encrypted file system (LUKS, BitLocker)
   - **Consideration:** Assess sensitivity of stored data

2. **Session Cookies in Development:**
   - Default configuration uses HTTP (not HTTPS)
   - **Mitigation:** Production mode enforces HTTPS
   - **Consideration:** Never use development mode in production

3. **CORS Policy in Development:**
   - Permissive settings for convenience
   - **Mitigation:** Production mode restricts origins
   - **Consideration:** Set `PRODUCTION=true` for deployment

---

## ✅ Section 8: Production Security Checklist

Complete all items before production deployment:

### 8.1: Core Security Configuration

- [ ] **Generate unique SESSION_SECRET** using `openssl rand -base64 64`
- [ ] **Set PRODUCTION=true** to enable production security mode
- [ ] **Configure CORS_ALLOWED_ORIGINS** with actual domain (no wildcards)
- [ ] **Enable HTTPS** with valid SSL/TLS certificate
- [ ] **Change default admin credentials** to strong, unique password

### 8.2: Authentication and Authorisation

- [ ] **Verify session expiration** is appropriate for your use case
- [ ] **Test authentication flow** completely before going live
- [ ] **Document admin account recovery** procedure

### 8.3: Infrastructure Security

- [ ] **Configure firewall** to allow only necessary ports (80, 443)
- [ ] **Set up monitoring** and alerting for security events
- [ ] **Enable automatic security updates** for operating system
- [ ] **Configure log rotation** to prevent disk space issues
- [ ] **Verify backup restoration** procedure works correctly

### 8.4: Application Security

- [ ] **Run cargo audit** and resolve any vulnerabilities
- [ ] **Set appropriate file permissions** (data directory: 755, files: 644)
- [ ] **Configure Content Security Policy** header appropriately
- [ ] **Run application with non-root user** account
- [ ] **Test all security controls** (authentication, authorisation, CSRF)

### 8.5: Operational Security

- [ ] **Document incident response** procedure
- [ ] **Create security contact** information
- [ ] **Schedule regular security reviews** (monthly recommended)
- [ ] **Plan dependency update** schedule
- [ ] **Test disaster recovery** procedure

---

## 🎯 Section 9: Next Steps

After reviewing security architecture, proceed with these tasks:

### For Security Teams

Complete these tasks in sequence:

1. **Review Security Checklist** – Verify all items above
2. **Conduct Security Assessment** – Perform penetration testing
3. **Establish Monitoring** – Set up security event monitoring

### For Operations Teams

Complete these tasks in sequence:

1. **Configure Environment** – Set all required security environment variables
2. **Deploy with HTTPS** – Follow [Production Deployment Guide](../deployment/production.md)
3. **Verify Security Controls** – Test all security features in production

### For Development Teams

Complete these tasks in sequence:

1. **Review Secure Coding** – Study the best practices above
2. **Implement Security Tests** – Add security validation to test suite
3. **Document Security Changes** – Update documentation for security-related changes

---

## 📖 Related Documentation

Access these resources for additional information:

- **[Design Documentation](design.md)** – System architecture overview and design principles
- **[Configuration Guide](../guides/configuration.md)** – Security-related configuration options
- **[Production Deployment Guide](../deployment/production.md)** – Production security practices and deployment
- **[API Reference](../api/reference.md)** – API security considerations and authentication

---

[← Back to Architecture](README.md) | [Documentation Home](../README.md)