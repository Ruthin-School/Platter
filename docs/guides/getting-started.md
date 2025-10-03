# 🚀 Quick Start Guide

> **Navigation:** [Documentation Home](../README.md) → [Getting Started](README.md) → Quick Start

**Reading Time:** Approximately 10 minutes  
**Complexity:** Beginner-friendly  
**Prerequisites:** Basic command-line knowledge

## 📖 What This Guide Covers

This guide explains how to install and run Platter on your local system. By the end of this guide, you will have:

1. Installed all required prerequisites
2. Downloaded the Platter source code
3. Built and started the application
4. Accessed the application in your web browser
5. Verified the installation is working correctly

---

## 📋 Section 1: Prerequisites

Before beginning the installation, you must install the required software on your system.

### Required Software

You need the following software installed:

1. **Rust programming language** (version 1.80 or higher) with Cargo package manager
2. **Git** version control system

### Step 1.1: Install Rust

Choose the instructions for your operating system:

#### Option A: Linux or macOS

Execute these commands in your terminal:

```bash
# Download and run the Rust installer
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Load Rust into your current terminal session
source "$HOME/.cargo/env"
```

**What this does:**
- Line 1: Downloads the official Rust installer script using secure HTTPS
- Line 2: Makes Rust commands available in your current terminal

#### Option B: Windows

Follow these steps in sequence:

1. Download the installer from [rustup-init.exe](https://win.rustup.rs/)
2. Run the downloaded file
3. When prompted, select option "1) Proceed with installation"
4. When prompted, allow the installer to add Rust to your PATH environment variable

### Step 1.2: Verify Rust Installation

Execute these commands to verify installation:

```bash
# Check Rust compiler version
rustc --version

# Check Cargo package manager version
cargo --version
```

**Expected output:**
- Rust version number (must be 1.80 or higher)
- Cargo version number

> ⚠️ **Troubleshooting:** If commands are not recognised, restart your terminal and try again. The PATH environment variable requires a fresh terminal session to take effect.

---

## 📥 Section 2: Installation

### Step 2.1: Clone the Repository

Execute these commands in your terminal:

```bash
# Download the Platter source code
git clone https://github.com/Ruthin-School/Platter.git

# Navigate into the downloaded directory
cd Platter
```

**What this does:**
- Line 1: Downloads the complete source code from GitHub
- Line 2: Changes your current directory to the Platter folder

### Step 2.2: Build and Run

Execute this command:

```bash
# Build and run the application
cargo run
```

**What happens during first run:**

1. **Dependency Download** – Cargo downloads all required libraries (this may take 2-5 minutes)
2. **Compilation** – Cargo compiles the application and dependencies (this may take 3-10 minutes)
3. **Server Start** – The web server starts on port 8080
4. **Ready State** – The application displays "Server running" message

> 📘 **Note:** Subsequent runs will be much faster because dependencies are already downloaded and compiled.

### Step 2.3: Access the Application

Open your web browser and navigate to this address:

```
http://localhost:8080
```

**What you should see:**
- The Platter homepage displaying the current menu or login page

---

## 🔑 Section 3: Default Credentials

The application includes default administrative credentials for development purposes.

### Default Admin Account

Use these credentials to log in:

- **Username:** `admin`
- **Password:** `admin123`

> ⚠️ **Critical Security Warning:** These credentials are for DEVELOPMENT ONLY. You MUST change these credentials before deploying to a production environment. Failure to change default credentials creates a severe security vulnerability.

**How to change credentials for production:**

Refer to the [Security Best Practices](../architecture/security.md#best-practices) documentation for detailed instructions on:
1. Creating secure admin credentials
2. Removing default accounts
3. Implementing password policies

---

## ✅ Section 4: Verify Installation

Follow these steps to confirm the application is working correctly.

### Test 4.1: Access the Login Page

Execute these steps in sequence:

1. Open your web browser
2. Navigate to `http://localhost:8080/admin/login`
3. Verify the login form appears

**Expected result:** Login page displays with username and password fields.

### Test 4.2: Perform Admin Login

Execute these steps in sequence:

1. Enter username: `admin`
2. Enter password: `admin123`
3. Click the "Login" button or press Enter

**Expected result:** Browser redirects to the admin dashboard.

### Test 4.3: View the Dashboard

After successful login, verify the following:

1. Dashboard page displays
2. Navigation menu appears
3. No error messages are shown

**Expected result:** Admin dashboard displays with menu management options.

### Test 4.4: Check the API

Execute this command in a terminal:

```bash
# Test API endpoint response
curl http://localhost:8080/api/items
```

**Expected output:** JSON response containing menu items (may be an empty array `[]` if no items exist).

> 📘 **Note:** If curl is not installed, you can skip this test. The previous tests confirm the application is working correctly.

---

## 🎯 Section 5: Next Steps

Now that Platter is installed and running, choose your next steps based on your role.

### Next Steps for General Users

Complete these tasks in sequence:

1. **Configure OAuth Authentication** – Follow the [OAuth Setup Guide](oauth-setup.md)
2. **Configure Environment Variables** – Review the [Configuration Guide](configuration.md)
3. **Review Security Settings** – Read the [Security Documentation](../architecture/security.md)

### Next Steps for Developers

Complete these tasks in sequence:

1. **Set Up Development Environment** – Follow the [Development Setup Guide](../development/setup.md)
2. **Read API Documentation** – Study the [API Reference](../api/reference.md)
3. **Review Architecture** – Understand the [Design Documentation](../architecture/design.md)

### Next Steps for System Administrators

Complete these tasks in sequence:

1. **Deploy to Production** – Follow the [Production Deployment Guide](../deployment/production.md)
2. **Configure Docker Deployment** – Review the [Docker Deployment Guide](../deployment/docker.md)
3. **Complete Security Checklist** – Execute the [Security Checklist](../architecture/security.md#security-checklist-for-production-deployment)

---

## 🔧 Section 6: Common Issues

This section explains how to resolve common installation problems.

### Issue 6.1: Build Fails

**Symptom:** The `cargo run` command fails with compilation errors.

**Solution:**

Execute these commands in sequence:

```bash
# Remove previous build artifacts
cargo clean

# Rebuild the application
cargo build
```

**Why this works:** Cleaning removes potentially corrupted build files and forces a complete rebuild.

### Issue 6.2: Permission Errors (Linux or macOS)

**Symptom:** Error messages about file permissions for the data directory.

**Solution:**

Execute this command:

```bash
# Set correct permissions for data directory
chmod 755 data/
```

**Why this works:** This command grants read, write, and execute permissions to the owner, and read and execute permissions to others.

### Issue 6.3: Port Already in Use

**Symptom:** Error message stating "Address already in use" or "Port 8080 is already in use".

**Cause:** Another application is using port 8080.

**Solution:**

Set a different port using the PORT environment variable:

```bash
# Run on port 3000 instead of 8080
PORT=3000 cargo run
```

**Why this works:** The application checks the PORT environment variable before using the default port 8080.

---

## 📖 Related Documentation

Access these resources for additional information:

- **[Configuration Guide](configuration.md)** – Configure environment variables and system settings
- **[OAuth Setup](oauth-setup.md)** – Configure authentication with Microsoft Entra ID
- **[Development Setup](../development/setup.md)** – Set up development environment for contributors
- **[Troubleshooting](../troubleshooting/README.md)** – Resolve common issues and errors
- **[API Reference](../api/reference.md)** – Complete API documentation for integration

---

[← Back to Getting Started](README.md) | [Documentation Home](../README.md)