# 🏗️ System Design Documentation

> **Navigation:** [Documentation Home](../README.md) → [Architecture](README.md) → Design

**Reading Time:** Approximately 20 minutes  
**Complexity:** Intermediate to Advanced  
**Prerequisites:** Understanding of web application architecture and Rust programming language

## 📖 What This Guide Covers

This guide explains the technical architecture and design decisions of Platter. By the end of this guide, you will understand:

1. The overall system architecture and component relationships
2. The technology stack and why each component was selected
3. How data flows through the application
4. Security architecture implementation
5. Performance optimisation strategies
6. Current scalability limitations and future expansion options

---

## 📋 Section 1: Architecture Overview

### 1.1: Architecture Pattern

Platter follows a traditional web application architecture pattern with emphasis on security, performance, and maintainability.

**Visual Architecture Diagram:**

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

**Data flow explanation:**
1. User agent (web browser) sends HTTP (Hypertext Transfer Protocol) requests to the application
2. Application processes requests using business logic
3. Template engine renders HTML (Hypertext Markup Language) responses
4. Data storage provides persistence for application data

### 1.2: Architecture Layers

The application is organised into four distinct layers:

| Layer Number | Layer Name | Purpose | Technologies |
|--------------|------------|---------|--------------|
| 1 | Presentation Layer | User interface rendering | HTML templates with Tera templating engine |
| 2 | Application Layer | Business logic and request handling | Rust with Actix-web framework |
| 3 | Data Layer | Data persistence and retrieval | JSON-based file system storage |
| 4 | Security Layer | Authentication and authorisation | Argon2 password hashing, session management |

**Layer interaction:**
- Each layer communicates only with adjacent layers
- This separation ensures maintainability and testability
- Changes in one layer minimise impact on other layers

---

## 🎯 Section 2: Core Design Principles

### 2.1: Security First

All design decisions prioritise security:

1. **Input Validation** – All user inputs are validated before processing
2. **Output Encoding** – All outputs are properly encoded to prevent injection attacks
3. **Secure Defaults** – Default configurations favour security over convenience
4. **Defence in Depth** – Multiple layers of security controls protect the system

### 2.2: Simplicity

The architecture avoids unnecessary complexity:

1. **Minimal Dependencies** – Only essential libraries are included
2. **Clear Code Structure** – Code organisation follows predictable patterns
3. **Direct Data Flow** – Request handling follows straightforward paths
4. **No Over-Engineering** – Solutions match actual requirements

### 2.3: Maintainability

The codebase is designed for long-term maintenance:

1. **Comprehensive Documentation** – All modules include clear documentation
2. **Consistent Style** – Code follows Rust style guidelines throughout
3. **Modular Design** – Features are separated into logical modules
4. **Clear Abstractions** – Abstractions are simple and well-defined

### 2.4: Performance

The system optimises for fast response times:

1. **Efficient Memory Usage** – Memory allocations are minimised
2. **Asynchronous Processing** – I/O (Input/Output) operations are non-blocking
3. **Static Asset Optimisation** – CSS (Cascading Style Sheets) and images are optimised for delivery
4. **Template Caching** – Compiled templates are cached in memory

---

## 🔧 Section 3: Core Components

### 3.1: Authentication System

**Purpose:** Verifies user identity and manages access permissions.

**Components:**

1. **Password Hashing**
   - Algorithm: Argon2 (memory-hard hashing function)
   - Salt: Randomly generated per password
   - Purpose: Prevents rainbow table attacks

2. **Session Management**
   - Storage: Encrypted cookies
   - Lifetime: Configurable expiration time
   - Security: HTTP-only and Secure flags enabled

3. **Access Control**
   - Model: Role-based access control (RBAC)
   - Roles: Administrator and standard user
   - Enforcement: Middleware checks on protected routes

**Authentication flow:**

1. User submits credentials via login form
2. Server hashes submitted password using Argon2
3. Server compares hash with stored hash
4. On match, server creates encrypted session cookie
5. Server sends cookie to user's browser
6. Browser includes cookie in subsequent requests

### 3.2: Data Management System

**Purpose:** Provides reliable data storage and retrieval.

**Storage approach:**

| Aspect | Implementation | Reason |
|--------|---------------|--------|
| Format | JSON (JavaScript Object Notation) | Human-readable and easy to backup |
| Location | File system in `data/` directory | No external database required |
| Structure | Separate files per data type | Clear organisation and easy maintenance |
| Updates | Atomic file operations | Prevents data corruption |

**Data files:**

1. `admin_users.json` – Administrator accounts and credentials
2. `menu_items.json` – Menu items with pricing and descriptions
3. `notices.json` – Announcements and notices
4. `menu_presets.json` – Reusable menu combinations
5. `menu_schedules.json` – Scheduled menu assignments

**Data validation:**

- All inputs are validated against schemas before storage
- Type checking ensures data integrity
- Required fields are enforced
- String lengths are validated

### 3.3: Scheduling System

**Purpose:** Manages automated menu scheduling.

**Components:**

1. **Date/Time Handling**
   - Library: Chrono (Rust date/time library)
   - Timezone: UTC (Coordinated Universal Time) for consistency
   - Formats: ISO 8601 standard

2. **Schedule Rules**
   - Validation: Prevents scheduling conflicts
   - Types: Breakfast, lunch, dinner
   - Duration: Single-day assignments

3. **Preset Management**
   - Definition: Reusable menu combinations
   - Assignment: Link presets to specific dates
   - Flexibility: Multiple presets per day possible

---

## 🔨 Section 4: Technology Stack

### 4.1: Backend Technologies

| Technology | Version | Purpose | Selection Reason |
|------------|---------|---------|------------------|
| [Rust](https://www.rust-lang.org/) | 1.80+ | Programming language | Memory safety, performance, concurrency |
| [Actix-web](https://actix.rs/) | 4.11.0 | Web framework | High performance, async support |
| [Tokio](https://tokio.rs/) | 1.47.1 | Asynchronous runtime | Efficient async I/O handling |
| [Serde](https://serde.rs/) | 1.0.226 | Serialisation/deserialisation | Fast JSON processing |
| [Tera](https://keats.github.io/tera/) | 1.20.0 | Template engine | Familiar Jinja2-like syntax |
| [Argon2](https://github.com/p-i-c-o/rust-argon2) | 0.5.3 | Password hashing | Industry-standard security |
| [Chrono](https://github.com/chronotope/chrono) | 0.4.42 | Date/time handling | Comprehensive timezone support |

**Technology selection criteria:**

1. **Security** – Libraries with proven security track records
2. **Performance** – Efficient implementations with minimal overhead
3. **Maintenance** – Active development and community support
4. **Compatibility** – Works well with other selected technologies

### 4.2: Frontend Technologies

**HTML5 (Hypertext Markup Language version 5):**
- Semantic markup provides clear document structure
- Tera templating enables dynamic content generation
- Accessibility attributes ensure screen reader compatibility

**CSS3 (Cascading Style Sheets version 3):**
- Mobile-first responsive design approach
- Flexbox and Grid layouts for modern interfaces
- Custom properties for theme consistency

**JavaScript:**
- Vanilla JavaScript without frameworks
- Used only where progressive enhancement is needed
- Minimal footprint for fast page loads

### 4.3: Data Storage

**JSON File-Based Storage:**

**Advantages:**
- ✅ No external database installation required
- ✅ Human-readable format for easy debugging
- ✅ Simple backup process (copy files)
- ✅ Version control friendly
- ✅ Fast for small to medium datasets

**Limitations:**
- ❌ Not optimised for very large datasets
- ❌ Limited querying capabilities
- ❌ Concurrent write access requires careful handling
- ❌ No built-in transaction support

---

## 🎨 Section 5: User Interface Design

### 5.1: Design Philosophy

#### 5.1.1: Responsive Design Approach

**Mobile-First Strategy:**

1. **Initial Design** – Interface designed for mobile screens (320px width minimum)
2. **Progressive Enhancement** – Additional features added for larger screens
3. **Breakpoints** – Layout adapts at tablet (768px) and desktop (1024px) widths
4. **Touch Optimisation** – Minimum 44×44 pixel touch targets for mobile users

**Implementation details:**

```css
/* Mobile-first base styles */
.menu-item {
  padding: 1rem;
  display: block;
}

/* Tablet enhancement */
@media (min-width: 768px) {
  .menu-item {
    display: flex;
    padding: 1.5rem;
  }
}

/* Desktop enhancement */
@media (min-width: 1024px) {
  .menu-item {
    padding: 2rem;
  }
}
```

#### 5.1.2: Accessibility Standards

**WCAG (Web Content Accessibility Guidelines) 2.1 AA Compliance:**

| Requirement | Implementation | Purpose |
|-------------|---------------|---------|
| Keyboard Navigation | All interactive elements accessible via Tab key | Users who cannot use a mouse |
| Screen Reader Support | Semantic HTML and ARIA (Accessible Rich Internet Applications) attributes | Visually impaired users |
| Colour Contrast | Minimum 4.5:1 ratio for normal text | Users with colour vision deficiency |
| Focus Indicators | Visible focus states on all interactive elements | Keyboard navigation users |

#### 5.1.3: User Experience Principles

**Intuitive Navigation:**
- Clear page hierarchy with breadcrumb trails
- Consistent menu placement across all pages
- Descriptive link text (avoid "click here")

**Fast Load Times:**
- Optimised images (WebP format with fallbacks)
- Minified CSS and JavaScript
- Efficient server-side rendering

**Clear Feedback:**
- Loading indicators for async operations
- Success messages after form submissions
- Error messages with specific guidance

---

## 💾 Section 6: Data Model Structures

### 6.1: Menu Item Structure

```rust
struct MenuItem {
    id: String,              // Unique identifier
    name: String,            // Display name
    description: String,     // Detailed description
    category: String,        // Category (e.g., "Main Course")
    price: String,           // Price as string (e.g., "10.99")
    available: bool,         // Availability status
    tags: Vec<String>,       // Filtering tags (e.g., ["vegetarian"])
}
```

**Field validation rules:**
- `id`: Auto-generated UUID (Universally Unique Identifier)
- `name`: 1-100 characters, required
- `description`: 1-500 characters, required
- `category`: 1-50 characters, required
- `price`: Decimal format with 2 decimal places
- `available`: Boolean, defaults to `true`
- `tags`: Array, each tag 1-30 characters

### 6.2: Notice Structure

```rust
struct Notice {
    id: String,                    // Unique identifier
    title: String,                 // Notice headline
    content: String,               // Notice message
    priority: Priority,            // Priority level enum
    start_date: DateTime<Utc>,     // Activation date-time
    end_date: DateTime<Utc>,       // Expiration date-time
}

enum Priority {
    Normal,   // Standard notices
    High,     // Important notices
    Urgent,   // Critical notices
}
```

**Field validation rules:**
- `id`: Auto-generated UUID
- `title`: 1-200 characters, required
- `content`: 1-2000 characters, required
- `priority`: Must be `Normal`, `High`, or `Urgent`
- `start_date`: Must be valid ISO 8601 format
- `end_date`: Must be after `start_date`

### 6.3: Schedule Structure

```rust
struct Schedule {
    id: String,                // Unique identifier
    preset_id: String,         // Referenced preset ID
    date: Date<Utc>,           // Schedule date
    meal_type: MealType,       // Meal type enum
    notes: Option<String>,     // Optional notes
}

enum MealType {
    Breakfast,   // Morning meal
    Lunch,       // Midday meal
    Dinner,      // Evening meal
}
```

**Field validation rules:**
- `id`: Auto-generated UUID
- `preset_id`: Must reference existing preset
- `date`: Valid date, cannot be in the past
- `meal_type`: Must be `Breakfast`, `Lunch`, or `Dinner`
- `notes`: Optional, maximum 500 characters

---

## 🔐 Section 7: Security Architecture

### 7.1: Authentication and Authorisation

**Password Security:**

1. **Hashing Algorithm:** Argon2id (hybrid version)
   - Memory cost: 64 MiB (Mebibytes)
   - Time cost: 3 iterations
   - Parallelism: 4 threads
   - Output length: 32 bytes

2. **Salt Generation:** Cryptographically secure random values
   - Length: 32 bytes
   - Uniqueness: Generated per password
   - Storage: Included in hash output

3. **Session Management:**
   - Cookie encryption: AES-256 (Advanced Encryption Standard)
   - Cookie flags: `HttpOnly`, `Secure`, `SameSite=Strict`
   - Expiration: Configurable, default 24 hours

### 7.2: Input Validation and Sanitisation

**Validation layers:**

1. **Client-side** (optional, for user convenience):
   - HTML5 form validation
   - JavaScript validation for immediate feedback

2. **Server-side** (required, for security):
   - Type checking using Rust's type system
   - Length validation for all string inputs
   - Format validation (emails, URLs, dates)
   - Business rule validation

**Sanitisation process:**

1. Trim whitespace from string inputs
2. Escape HTML special characters in user content
3. Validate and normalise URLs
4. Remove potentially dangerous characters from file paths

### 7.3: Network Security

**CORS (Cross-Origin Resource Sharing) Configuration:**

**Development mode:**
- Allowed origins: `http://localhost:8080`, `http://127.0.0.1:8080`
- Purpose: Convenience during development

**Production mode:**
- Allowed origins: Configured via `CORS_ALLOWED_ORIGINS` environment variable
- Strict validation of origin headers
- Credential support: Enabled only for allowed origins

**Security headers:**

| Header | Value | Purpose |
|--------|-------|---------|
| `X-Content-Type-Options` | `nosniff` | Prevents MIME type sniffing |
| `X-Frame-Options` | `DENY` | Prevents clickjacking attacks |
| `X-XSS-Protection` | `1; mode=block` | Enables XSS (Cross-Site Scripting) filter |
| `Content-Security-Policy` | Restrictive policy | Prevents XSS and injection attacks |

**CSRF (Cross-Site Request Forgery) Protection:**
- Token generation: Cryptographically secure random values
- Token validation: Checked on all state-changing operations
- Token storage: Included in session cookie

---

## ⚡ Section 8: Performance Optimisation

### 8.1: Application Performance

**Startup Optimisation:**

1. **Lazy Loading** – Resources loaded only when needed
2. **Parallel Initialisation** – Independent components initialise concurrently
3. **Minimal Startup Tasks** – Essential operations only at startup

**Runtime Optimisation:**

1. **Asynchronous I/O** – Non-blocking file and network operations
2. **Connection Pooling** – Reuse of network connections
3. **Efficient Memory Usage** – Minimal allocations, strategic buffer reuse

### 8.2: Caching Strategies

**Template Caching:**
- Compiled templates stored in memory
- Recompilation only on template file changes
- Significant performance improvement for repeated renders

**Static Asset Caching:**
- Browser caching enabled via `Cache-Control` headers
- Versioned URLs prevent stale cache issues
- Compression enabled for text-based assets

**Session Caching:**
- Active sessions stored in memory
- Periodic persistence to files for durability
- Automatic cleanup of expired sessions

### 8.3: Database-Free Architecture Benefits

**Performance advantages:**

1. **No Network Overhead** – File system access is local
2. **Simple Queries** – Direct object access without query parsing
3. **Fast Reads** – In-memory caching of frequently accessed data
4. **No Connection Management** – No database connection pool required

**Trade-offs:**

1. **Limited Concurrent Writes** – File locking required for safety
2. **No Complex Queries** – Application logic handles filtering
3. **Scalability Ceiling** – Not suitable for very high loads

---

## 📈 Section 9: Scalability Considerations

### 9.1: Current Limitations

**Identified constraints:**

| Limitation | Impact | Current Mitigation |
|------------|--------|-------------------|
| File-based storage | Not suitable for high traffic | In-memory caching reduces file access |
| Single process | Limited concurrent request handling | Asynchronous I/O maximises efficiency |
| No query optimisation | Inefficient for complex searches | Data structures optimised for common queries |
| Local file system | No distributed deployment | Suitable for current use case |

### 9.2: Horizontal Scaling Options

**Future scalability paths:**

1. **Database Migration**
   - Target: PostgreSQL or MongoDB
   - Benefit: Better query capabilities, transaction support
   - Effort: Medium (requires data migration layer)

2. **Caching Layer**
   - Target: Redis for session and data caching
   - Benefit: Faster data access, session sharing
   - Effort: Low (minimal code changes)

3. **Load Balancing**
   - Target: Multiple application instances behind load balancer
   - Benefit: Higher concurrent request capacity
   - Effort: Medium (requires shared session storage)

4. **CDN (Content Delivery Network) Integration**
   - Target: CloudFlare or similar for static assets
   - Benefit: Faster global asset delivery
   - Effort: Low (configuration only)

5. **Microservices Architecture**
   - Target: Separate services for authentication, menu, scheduling
   - Benefit: Independent scaling of components
   - Effort: High (significant architectural changes)

### 9.3: Vertical Scaling Considerations

**Current resource efficiency:**
- Memory usage: ~50-100 MB under typical load
- CPU usage: Minimal except during request bursts
- Disk I/O: Limited to session persistence and data updates

**Scaling recommendations:**

1. **Low Traffic** (< 100 concurrent users):
   - 1 CPU core, 512 MB RAM sufficient
   - Current architecture performs well

2. **Medium Traffic** (100-1000 concurrent users):
   - 2-4 CPU cores, 1-2 GB RAM recommended
   - Consider Redis for session storage

3. **High Traffic** (> 1000 concurrent users):
   - Requires architectural changes
   - Database migration essential
   - Load balancing recommended

---

## 🎯 Section 10: Next Steps

### For System Architects

Complete these tasks in sequence:

1. **Review Security Architecture** – Study the [Security Documentation](security.md)
2. **Understand Deployment Options** – Read the [Production Deployment Guide](../deployment/production.md)
3. **Plan Scalability Strategy** – Assess your traffic expectations against current limitations

### For Developers

Complete these tasks in sequence:

1. **Set Up Development Environment** – Follow the [Development Setup Guide](../development/setup.md)
2. **Study API Design** – Review the [API Reference](../api/reference.md)
3. **Understand Data Models** – Examine the data structures documented above

### For Operations Teams

Complete these tasks in sequence:

1. **Review Performance Characteristics** – Understand resource requirements
2. **Plan Monitoring Strategy** – Identify key metrics to monitor
3. **Prepare Scaling Plan** – Choose appropriate scaling approach for your traffic

---

## 📖 Related Documentation

Access these resources for additional information:

- **[Security Documentation](security.md)** – Security architecture and implementation details
- **[Development Setup Guide](../development/setup.md)** – Development environment configuration
- **[API Reference](../api/reference.md)** – API design and endpoint documentation
- **[Production Deployment Guide](../deployment/production.md)** – Production architecture and deployment
- **[Configuration Guide](../guides/configuration.md)** – Configuration options and environment variables

---

[← Back to Architecture](README.md) | [Documentation Home](../README.md)