# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0] - 2025-10-04

### Added
- Compile-time WCAG 2.1 Level AA accessibility validation system
- ARIA role and attribute validation
- Colour contrast ratio checking (4.5:1 normal text, 3:1 large text)
- Semantic HTML structure validation
- Form label association validation
- Template analyser for heading hierarchy
- Standalone a11y-check CLI tool for manual validation
- Comprehensive CI/CD workflows (tests, security, coverage, accessibility)
- Git hooks infrastructure with pre-commit and commit-msg validation
- Detailed development workflow documentation
- Branch protection and merge strategies guides
- Build optimisation guide with feature flag documentation
- Git workflow guide with SemVer and release tagging
- Security audit configuration with cargo-deny

### Changed
- Improved warning colour contrast from 4.03:1 to 5.38:1 for WCAG AA compliance
- Refactored code to pass all clippy warnings with strict linting
- Applied consistent code formatting across entire codebase
- Enhanced button text detection in accessibility validator
- Updated documentation structure with comprehensive navigation

### Fixed
- Button text detection logic in template analyzer
- Unused imports in accessibility validation tests
- Clippy warnings for collapsible if statements
- Redundant closures in error handling
- Dead code warnings with appropriate annotations

### Security
- Documented RUSTSEC-2023-0071 (RSA Marvin Attack) exception in deny.toml
- Added comprehensive security audit workflows
- Implemented strict commit signing requirements
- Enhanced branch protection documentation

## [0.4.0] - 2025-09-28

### Added
- Comprehensive API documentation in `API.md`
- Detailed design documentation in `DESIGN.md`
- Complete development guide in `DEVELOPMENT.md`
- Production deployment guide in `DEPLOYMENT.md`
- Security best practices in `SECURITY.md`
- Enhanced README with detailed configuration instructions
- Basic test module in `src/main.rs`

### Changed
- Updated application from version 0.3.0 to 0.4.0
- Replaced `#[actix_web::main]` with `#[tokio::main]` for async runtime
- Improved session security with environment variable support for `SESSION_SECRET`
- Updated deprecated `std::io::Error::new()` to `std::io::Error::other()`
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
- Data validation and sanitisation
- User session management

### Changed
- Improved error handling and reporting
- Enhanced template system with Tera
- Updated to Rust 2024 edition
- Refactored storage system for improved performance

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