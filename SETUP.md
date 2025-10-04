# 🚀 Platter Setup Guide - v0.8.0

> **Navigation:** [Documentation Home](docs/README.md) → Setup

> 📘 **Quick Start:** For a beginner-friendly getting started guide, see [`docs/guides/getting-started.md`](docs/guides/getting-started.md).

**Reading Time:** Approximately 10 minutes
**Complexity:** Intermediate
**Prerequisites:** Rust 1.80+, Git, basic command-line knowledge

## What This Guide Covers

This guide explains the v0.8.0 hybrid JSON/TOML configuration system and setup procedures. You will learn how to:

1. Configure the TOML-based admin and validation system
2. Set up enhanced JSON data files with metadata
3. Understand the new file structure
4. Troubleshoot configuration issues

---

## Quick Start

### Step 1: Clone and Build

```bash
git clone https://github.com/Ruthin-School/Platter.git
cd Platter
cargo build --release
```

### Step 2: Configure Application

#### Create Configuration Files

```bash
# Copy template files
cp config/admin.toml.example config/admin.toml
cp data/menu_items.json.example data/menu_items.json
```

#### Set Up Admin User

Edit `config/admin.toml` and update:
- Generate a new UUID for `id`
- Set your `username`
- Generate password hash using: `cargo run --bin platter -- admin hash-password`

> ⚠️ **Important:** Never commit `config/admin.toml` to version control (already in `.gitignore`)

### Step 3: Customise Settings

Edit `config/settings.toml` to configure:
- Server port and host
- Security settings
- Feature flags
- Logging preferences

Edit `config/validation.toml` to set:
- Field length constraints
- Validation rules
- Allergen lists

### Step 4: Run Application

```bash
# Development
cargo run

# Production
./target/release/platter
```

Application will be available at: `http://localhost:8080`

---

## File Structure (v0.8.0)

```
platter/
├── config/                          # TOML Configuration
│   ├── admin.toml                  # ❌ DO NOT COMMIT (in .gitignore)
│   ├── admin.toml.example          # ✅ Template - safe to commit
│   ├── validation.toml             # ✅ Safe to commit
│   └── settings.toml               # ✅ Safe to commit
│
├── data/                            # JSON Data Storage
│   ├── menu_items.json             # ❌ DO NOT COMMIT (in .gitignore)
│   ├── menu_items.json.example     # ✅ Template - safe to commit
│   ├── notices.json                # ❌ DO NOT COMMIT (in .gitignore)
│   ├── menu_presets.json           # ❌ DO NOT COMMIT (in .gitignore)
│   └── menu_schedules.json         # ❌ DO NOT COMMIT (in .gitignore)
│
├── src/                             # Source Code
│   ├── config.rs                   # TOML configuration loader
│   ├── storage.rs                  # Legacy storage (v0.7.1)
│   ├── storage_v2.rs               # Hybrid storage (v0.8.0)
│   └── ...
│
├── docs/                            # Documentation
├── Cargo.toml                       # Project dependencies
├── STORAGE_ARCHITECTURE.md          # Architecture overview
├── STORAGE_MIGRATION.md             # Migration guide
└── SETUP.md                         # This file
```

---

## Configuration Details

### Admin Configuration (config/admin.toml)

```toml
[[admin_users]]
id = "uuid-here"                    # Generate new UUID
username = "admin"
password_hash = "$argon2id$..."     # Use CLI to generate
roles = ["super_admin"]
is_active = true
```

**Generate password hash:**
```bash
cargo run --bin platter -- admin hash-password
# Enter password when prompted
```

### Validation Rules (config/validation.toml)

Controls data validation:
- Field length constraints (names, descriptions)
- Allowed allergens list
- Schedule duration limits
- Duplicate prevention rules

### App Settings (config/settings.toml)

Configure:
- **Server:** Port, host, workers
- **Security:** Session timeout, login attempts, CORS
- **Logging:** Level, format, file rotation
- **Features:** Enable/disable functionality

## Data Files

### Menu Items (data/menu_items.json)

Enhanced JSON format with metadata:

```json
{
  "schema_version": "1.0.0",
  "last_updated": "2025-10-04T15:00:00Z",
  "metadata": {
    "total_items": 50,
    "data_integrity_check": "passed"
  },
  "items": [
    {
      "id": "uuid",
      "name": "Menu Item Name",
      "category": "Mains",
      "description": "Description here",
      "allergens": ["eggs", "milk"],
      "is_available": true
    }
  ]
}
```

**Categories:** Mains, Sides, Desserts, Beverages

### Notices (data/notices.json)

System-wide notices displayed to users.

### Menu Presets (data/menu_presets.json)

Predefined menu configurations for quick activation.

### Menu Schedules (data/menu_schedules.json)

Automated menu scheduling with recurrence rules.

## Security Best Practices

### Files to NEVER commit:
- ❌ `config/admin.toml` (contains password hashes)
- ❌ `data/*.json` (contains application data)

### Files safe to commit:
- ✅ `config/admin.toml.example` (template)
- ✅ `config/validation.toml` (rules)
- ✅ `config/settings.toml` (settings template)
- ✅ `data/*.json.example` (data templates)

### File Permissions

```bash
# Secure admin config
chmod 600 config/admin.toml

# Readable data files
chmod 644 data/*.json

# Ensure data directory exists
mkdir -p data/backups
```

## Environment-Specific Configuration

### Development
```bash
PLATTER_ENV=development cargo run
```

### Staging
```bash
PLATTER_ENV=staging ./target/release/platter
```

### Production
```bash
PLATTER_ENV=production ./target/release/platter
```

Create environment-specific configs:
- `config/settings.development.toml`
- `config/settings.staging.toml`
- `config/settings.production.toml`

## Troubleshooting

### "Config file not found"
```bash
# Copy from templates
cp config/admin.toml.example config/admin.toml
cp data/menu_items.json.example data/menu_items.json
```

### "TOML parsing error"
```bash
# Validate syntax
toml check config/admin.toml

# Common issues:
# - Missing quotes around special characters
# - Invalid datetime format (use RFC3339)
# - Duplicate keys
```

### "Permission denied"
```bash
# Fix file permissions
chmod 600 config/admin.toml
chmod 644 data/*.json
```

### "Database/Storage error"
```bash
# Verify data integrity
cargo run --bin platter -- validate

# Restore from backup
cp data/backups/menu_items.json.backup data/menu_items.json
```

## Migration from v0.7.1

If upgrading from version 0.7.1:

1. **Backup your data:**
   ```bash
   cp -r data/ data_backup_v0.7.1/
   ```

2. **Run migration:**
   ```bash
   cargo run --bin platter -- migrate
   ```

3. **Verify migration:**
   ```bash
   cargo run --bin platter -- validate
   ```

See [`STORAGE_MIGRATION.md`](STORAGE_MIGRATION.md) for detailed migration guide.

## CLI Commands

```bash
# Hash password for admin user
cargo run --bin platter -- admin hash-password

# Add new admin user
cargo run --bin platter -- admin add-user --username <name>

# Validate data integrity
cargo run --bin platter -- validate

# Check configuration
cargo run --bin platter -- config check

# Migrate from v0.7.1
cargo run --bin platter -- migrate
```

## Development Workflow

1. **Make changes to code**
2. **Update validation rules** in `config/validation.toml` if needed
3. **Test locally:**
   ```bash
   cargo test
   cargo run
   ```
4. **Commit changes** (excluding data files)
5. **Deploy** to staging/production

## Related Documentation

Access these resources for additional information:

- **[Storage Architecture](STORAGE_ARCHITECTURE.md)** – Hybrid JSON/TOML architecture overview
- **[Migration Guide](STORAGE_MIGRATION.md)** – Upgrading from v0.7.1 to v0.8.0
- **[API Documentation](docs/api/reference.md)** – API endpoints and usage
- **[Configuration Guide](docs/guides/configuration.md)** – Environment variables and settings
- **[Getting Started Guide](docs/guides/getting-started.md)** – Beginner-friendly installation

## Getting Help

If you encounter issues or need assistance:

1. **First Step** – Check the [Troubleshooting Guide](docs/troubleshooting/README.md)
2. **Second Step** – Search existing [GitHub Issues](https://github.com/Ruthin-School/Platter/issues)
3. **Third Step** – [Open a new issue](https://github.com/Ruthin-School/Platter/issues/new) with detailed information

---

**Version:** 0.8.0
**Last Updated:** 2025-10-04
**Maintained By:** Ruthin School

[← Back to Main README](README.md) | [Documentation Home](docs/README.md)