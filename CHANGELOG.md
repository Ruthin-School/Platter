# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Project renamed from \"Dining Hall Dashboard\" to \"Platter\"

## [0.4.0] - 2025-09-28

### Added
- Comprehensive API documentation in API.md
- Detailed design documentation in DESIGN.md
- Complete development guide in DEVELOPMENT.md
- Production deployment guide in DEPLOYMENT.md
- Security best practices in SECURITY.md
- Enhanced README with detailed configuration instructions
- Basic test module in src/main.rs

### Changed
- Updated application from version 0.3.0 to 0.4.0
- Replaced `#[actix_web::main]` with `#[tokio::main]` for async runtime
- Improved session security with environment variable support for SESSION_SECRET
- Updated deprecated std::io::Error::new() to std::io::Error::other()
- Simplified error mapping syntax in handlers
- Enhanced documentation with comprehensive tables of contents and detailed explanations

### Security
- Added security-focused session management with environment-based secrets
- Improved CORS configuration for production environments
- Enhanced security documentation with best practices checklist
- Updated error handling to prevent information disclosure

## [0.3.0] - 2025-08-15

### Added
- Menu preset functionality
- Menu scheduling system
- Admin interface improvements
- Data validation and sanitization
- User session management

### Changed
- Improved error handling and reporting
- Enhanced template system with Tera
- Updated to Rust 2024 edition
- Refactored storage system for better performance

## [0.2.0] - 2025-07-10

### Added
- Admin authentication system with Argon2 password hashing
- Menu item CRUD operations
- Notice system with priority levels
- JSON file-based storage
- Responsive web interface

### Changed
- Implemented secure session management
- Added CSRF protection
- Improved input validation
- Enhanced security headers

## [0.1.0] - 2025-06-01

### Added
- Initial release of Platter
- Basic menu management
- User authentication
- Web-based administration interface
- File-based data persistence