# Security Policy

Platter prioritizes security in its design and implementation to protect user data and maintain application integrity.

## 🛡️ Security Features

The application implements multiple layers of security:

### Authentication & Authorization
- **Argon2 Password Hashing**: Industry-standard password security with salt
- **OAuth 2.0 Support**: Secure authentication with Microsoft Entra ID using OpenID Connect
- **PKCE Protection**: Proof Key for Code Exchange for enhanced OAuth security
- **Email Whitelist**: OAuth users validated against approved email addresses
- **Role-based Access Control**: Admin-only access to sensitive operations
- **Secure Session Management**: Session encryption and proper lifecycle management

### Data Protection
- **Input Validation**: Server-side validation and sanitization of all inputs
- **Output Encoding**: Prevention of XSS through proper output encoding
- **SQL Injection Prevention**: Parameterized queries where applicable

### Network Security
- **Secure HTTP-only Cookies**: Session cookies with HttpOnly and Secure flags
- **CSRF Protection**: Cross-site request forgery protection tokens
- **Rate Limiting**: Built-in rate limiting on authentication endpoints
- **CORS Configuration**: Configurable Cross-Origin Resource Sharing policies

## ⚙️ Configuration for Security

### Environment Variables

| Variable              | Description                                                                 | Required for Production | Default Value                             |
|-----------------------|-----------------------------------------------------------------------------|-------------------------|-------------------------------------------|
| `SESSION_SECRET`      | 64-byte secret key for session encryption (required in production)          | ✅                      | Development key (insecure)                |
| `PRODUCTION`          | Enable production mode with enhanced security                               | ❌                      | Not set (development mode)                |
| `CORS_ALLOWED_ORIGINS`| Comma-separated list of allowed origins (when in production mode)           | ❌                      | `http://localhost:8080,http://127.0.0.1:8080` |
| `PORT`                | Port to run the application on                                              | ❌                      | `8080`                                    |
| `HOST`                | Host address to bind to                                                     | ❌                      | `0.0.0.0`                                 |
| `RUST_LOG`            | Logging level (be cautious with debug in production)                        | ❌                      | `info`                                    |

### Session Configuration

For production deployments:
1. Always set a secure, randomly generated `SESSION_SECRET`
2. Set `cookie_secure = true` when serving over HTTPS
3. Consider reducing session expiration time based on your requirements

Example production configuration:
```bash
SESSION_SECRET=$(openssl rand -base64 64)
PRODUCTION=true
CORS_ALLOWED_ORIGINS=https://yourdomain.com,https://admin.yourdomain.com
```

## 🔐 Best Practices

### For Administrators

1. **Change Default Credentials**: Never use default admin credentials in production
2. **Use Strong Passwords**: Implement strong password requirements
3. **Regular Updates**: Keep the application updated with security patches
4. **Monitor Logs**: Regularly review application logs for suspicious activity
5. **HTTPS in Production**: Always serve the application over HTTPS in production

### For Developers

1. **Environment Security**: Use environment variables for secrets, never hardcode
2. **Input Validation**: Validate and sanitize all user inputs
3. **Principle of Least Privilege**: Run the application with minimal required permissions
4. **Secure Dependencies**: Regularly audit and update dependencies

## 🚨 Security Reporting

If you discover a security vulnerability in this project, please contact the maintainers directly instead of opening a public issue. We will work to address security concerns promptly.

### Known Security Considerations
- The application stores data in JSON files by default; consider this when planning for sensitive environments
- Default session cookies are configured for development; production deployments require HTTPS for secure cookies
- CORS policy is permissive in development mode for convenience

## 📋 Security Checklist for Production Deployment

- [ ] Set a strong, unique `SESSION_SECRET`
- [ ] Configure `CORS_ALLOWED_ORIGINS` to only allow trusted domains
- [ ] Ensure the application serves over HTTPS
- [ ] Change the default admin credentials before going live
- [ ] Configure OAuth settings with proper client credentials and email whitelist
- [ ] Set `PRODUCTION=true` to enable production security settings
- [ ] Configure proper logging and monitoring
- [ ] Ensure the application runs with minimal permissions
- [ ] Validate that data directories have appropriate permissions