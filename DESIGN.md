# Design Documentation

This document provides an overview of the architecture, design decisions, and technical considerations of Platter.

## 📋 Table of Contents

- [Architecture Overview](#architecture-overview)
- [System Design](#system-design)
- [Technology Stack](#technology-stack)
- [User Interface Design](#user-interface-design)
- [Data Management](#data-management)
- [Security Architecture](#security-architecture)
- [Performance Considerations](#performance-considerations)
- [Scalability](#scalability)

## Architecture Overview

Platter follows a traditional web application architecture pattern with a focus on security, performance, and maintainability:

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   User Agent    │    │  Application     │    │   Data Storage  │
│ (Browser/Client)│◄──►│  (Actix-web)     │◄──►│   (JSON Files)  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                            │
                   ┌──────────────────┐
                   │ Template Engine  │
                   │     (Tera)       │
                   └──────────────────┘
```

### Architecture Layers

1. **Presentation Layer**: HTML templates with Tera templating engine
2. **Application Layer**: Business logic, authentication, and request handling
3. **Data Layer**: JSON-based file system storage
4. **Security Layer**: Authentication, session management, and input validation

## System Design

### Core Principles

- **Security First**: Built-in security measures at every level
- **Simplicity**: Minimal dependencies and straightforward architecture
- **Maintainability**: Clean, well-documented Rust code
- **Performance**: Optimized for fast response times

### Key Components

#### Authentication System
- Session-based authentication using secure cookies
- Password hashing with Argon2
- Role-based access control for admin functions

#### Data Management
- JSON-based file storage for simple deployment
- Structured data models for menu items, notices, and schedules
- Data validation and sanitization

#### Scheduling System
- Automated menu scheduling capabilities
- Date/time handling with Chrono
- Configurable scheduling rules

## Technology Stack

### Backend Technologies

| Technology | Purpose | Version Specified |
|------------|---------|-------------------|
| [Rust](https://www.rust-lang.org/) | Programming language | 1.80+ |
| [Actix-web](https://actix.rs/) | Web framework | 4.11.0 |
| [Tokio](https://tokio.rs/) | Asynchronous runtime | 1.47.1 |
| [Serde](https://serde.rs/) | Serialization/deserialization | 1.0.226 |
| [Tera](https://keats.github.io/tera/) | Template engine | 1.20.0 |
| [Argon2](https://github.com/p-i-c-o/rust-argon2) | Password hashing | 0.5.3 |
| [Chrono](https://github.com/chronotope/chrono) | Date/time handling | 0.4.42 |

### Frontend Technologies

- **HTML5**: Semantic markup with Tera templating
- **CSS3**: Responsive design with mobile-first approach
- **Vanilla JavaScript**: Where needed for client-side interactivity

### Storage

- **JSON Files**: Simple file-based storage in the `data/` directory
- **File-based**: No external database required for basic deployment

## User Interface Design

### Design Philosophy

#### Responsive Design
- **Mobile-First**: Interface designed primarily for mobile devices, then enhanced for larger screens
- **Progressive Enhancement**: Core functionality available on all devices
- **Touch-Friendly**: Adequate touch targets for mobile users

#### Accessibility
- **WCAG 2.1 AA Compliance**: Designed with accessibility in mind
- **Keyboard Navigation**: Full functionality available via keyboard
- **Screen Reader Support**: Proper semantic HTML and ARIA attributes
- **Color Contrast**: Sufficient contrast ratios for readability

#### User Experience
- **Intuitive Navigation**: Clear information architecture
- **Fast Load Times**: Optimized assets and efficient rendering
- **Clear Feedback**: Visual indicators for user actions

## Data Management

### Data Models

#### Menu Items
```rust
struct MenuItem {
    id: String,
    name: String,
    description: String,
    category: String,
    price: String,
    available: bool,
    tags: Vec<String>,
}
```

#### Notices
```rust
struct Notice {
    id: String,
    title: String,
    content: String,
    priority: Priority, // normal | high | urgent
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}
```

#### Schedules
```rust
struct Schedule {
    id: String,
    preset_id: String,
    date: Date<Utc>,
    meal_type: MealType, // breakfast | lunch | dinner
    notes: Option<String>,
}
```

### Storage Strategy

- **File-based**: Simple JSON files for easy deployment
- **Atomic Updates**: Safe file operations to prevent corruption
- **Backup-Friendly**: Easy to backup and restore data

## Security Architecture

### Authentication & Authorization

- **Session Management**: Secure session handling with encrypted cookies
- **Password Security**: Argon2 hashing with salt for password storage
- **Input Validation**: Server-side validation of all user inputs
- **Rate Limiting**: Protection against brute force attacks

### Data Security

- **Data Validation**: All inputs are validated and sanitized
- **Output Encoding**: Prevention of XSS through proper encoding
- **Access Control**: Role-based restrictions on sensitive operations

### Network Security

- **CORS Policy**: Configurable origin restrictions
- **Secure Headers**: Proper HTTP headers for security
- **CSRF Protection**: Token-based protection against CSRF attacks

## Performance Considerations

### Optimization Strategies

- **Fast Startup**: Minimal initialization time
- **Memory Efficiency**: Efficient memory usage patterns
- **Concurrent Requests**: Asynchronous request handling with Tokio
- **Static Asset Serving**: Optimized delivery of CSS, JS, and images

### Caching

- **Template Caching**: Tera templates are cached after compilation
- **Response Caching**: Appropriate cache headers for static content
- **Session Caching**: In-memory session data (with file persistence)

## Scalability

### Current Limitations

- **File-Based Storage**: May not scale to very high loads
- **Single Process**: No built-in clustering or multi-process support
- **No Database**: Limited querying capabilities

### Future Scalability Options

1. **Database Migration**: Support for SQL/NoSQL databases
2. **Caching Layer**: Redis for session and data caching
3. **Load Balancing**: Multiple instances behind a load balancer
4. **CDN Integration**: Static asset delivery via CDN
5. **Microservices**: Breaking down into smaller, specialized services

### Horizontal Scaling Considerations

- **Stateless Design**: Sessions can be made stateless for easier scaling
- **Shared Storage**: NFS or database for shared data across instances
- **Load Balancer**: Proper session affinity if needed