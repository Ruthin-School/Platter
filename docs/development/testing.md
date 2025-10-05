# 🧪 Testing Guide

> **Navigation:** [Documentation Home](../README.md) → [Development](README.md) → Testing

**Reading Time:** Approximately 18 minutes  
**Complexity:** Intermediate  
**Prerequisites:** Understanding of Rust programming and unit testing concepts

## 📖 What This Guide Covers

This guide provides comprehensive testing practices and procedures for Platter. By the end of this guide, you will understand:

1. How to run different types of tests
2. How to write effective unit and integration tests
3. How to organise test code properly
4. How to measure and improve test coverage
5. Testing best practices and patterns

---

## 📋 Section 1: Running Tests

### 1.1: Basic Test Commands

Execute these commands to run tests:

**Command 1.1.1: Run All Tests**
```bash
cargo test
```

**What this does:** Executes all tests in the project sequentially.

**Command 1.1.2: Run Tests with Output**
```bash
cargo test -- --nocapture
```

**What this does:** Displays `println!` statements from tests (normally hidden).

**Command 1.1.3: Run Specific Test**
```bash
cargo test test_function_name
```

**What this does:** Executes only the test function matching the specified name.

**Command 1.1.4: Run Tests in Module**
```bash
cargo test module_name::
```

**What this does:** Executes all tests within the specified module.

**Command 1.1.5: Run Tests Matching Pattern**
```bash
cargo test pattern
```

**What this does:** Executes all tests whose names contain the specified pattern.

### 1.2: Advanced Test Execution

**Command 1.2.1: Run Tests with Verbose Output**
```bash
cargo test -- --nocapture --test-threads=1
```

**What this does:**
- `--nocapture`: Shows all output from tests
- `--test-threads=1`: Runs tests one at a time (prevents concurrent execution)

**When to use:** Debugging tests that may interfere with each other.

**Command 1.2.2: Run Ignored Tests**
```bash
cargo test -- --ignored
```

**What this does:** Executes only tests marked with `#[ignore]` attribute.

**Command 1.2.3: Run All Tests Including Ignored**
```bash
cargo test -- --include-ignored
```

**What this does:** Executes all tests, including those marked as ignored.

**Command 1.2.4: Show Test Execution Time**
```bash
cargo test -- --show-output
```

**What this does:** Displays execution time for each test.

### 1.3: Test Coverage Analysis

**Step 1.3.1: Install Coverage Tool**
```bash
cargo install cargo-tarpaulin
```

**What this does:** Installs Tarpaulin, a code coverage tool for Rust.

**Step 1.3.2: Generate HTML Coverage Report**
```bash
cargo tarpaulin --out Html
```

**What this does:**
1. Runs all tests
2. Measures which code lines are executed
3. Generates HTML report in `tarpaulin-report.html`

**Step 1.3.3: Enforce Coverage Threshold**
```bash
cargo tarpaulin --fail-under 80
```

**What this does:** Fails if test coverage is below 80%.

---

## ✍️ Section 2: Writing Tests

### 2.1: Unit Tests

**Purpose:** Unit tests verify individual functions work correctly in isolation.

**Location:** Place unit tests in the same file as the code being tested.

**Example unit test module:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_creation() {
        // Given: Input data
        let name = "Burger";
        let description = "Delicious burger";
        let category = "Main Course";
        
        // When: Create menu item
        let item = MenuItem::new(name, description, category);
        
        // Then: Verify properties
        assert_eq!(item.name, "Burger");
        assert_eq!(item.category, "Main Course");
    }

    #[test]
    fn test_validation_fails_for_empty_name() {
        // Given: Empty name
        let name = "";
        
        // When: Validate name
        let result = MenuItem::validate_name(name);
        
        // Then: Should return error
        assert!(result.is_err());
    }
}
```

**Module structure explanation:**
- `#[cfg(test)]`: Only compiles this module during testing
- `mod tests`: Contains all test functions
- `use super::*`: Imports items from parent module
- `#[test]`: Marks function as a test

### 2.2: Integration Tests

**Purpose:** Integration tests verify multiple components work together correctly.

**Location:** Create integration tests in the `tests/` directory at project root.

**Example integration test:**

```rust
// tests/api_tests.rs
use platter::handlers;
use actix_web::{test, App};

#[actix_web::test]
async fn test_get_menu_items() {
    // Given: Test application
    let app = test::init_service(
        App::new().service(handlers::get_items)
    ).await;

    // When: Request menu items
    let req = test::TestRequest::get()
        .uri("/api/items")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Then: Should return success
    assert!(resp.status().is_success());
}
```

**Integration test characteristics:**
- Tests complete request/response cycle
- Verifies handler functions work correctly
- Uses Actix-web test utilities

### 2.3: Asynchronous Tests

**Purpose:** Test asynchronous functions that return futures.

**Attribute required:** `#[actix_web::test]` for async tests.

**Example async test:**

```rust
#[actix_web::test]
async fn test_async_function() {
    // Given: Test setup
    let input = "test data";
    
    // When: Execute async function
    let result = async_function(input).await;
    
    // Then: Verify result
    assert!(result.is_ok());
}
```

**What `#[actix_web::test]` does:**
- Sets up async runtime
- Allows using `.await` in test
- Manages async execution

### 2.4: Testing with Mock Data

**Purpose:** Create reusable test data for consistent testing.

**Example mock data helper:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_menu_item() -> MenuItem {
        MenuItem {
            id: "test-123".to_string(),
            name: "Test Item".to_string(),
            description: "Test Description".to_string(),
            category: "Test Category".to_string(),
            price: "9.99".to_string(),
            available: true,
            tags: vec!["test".to_string()],
        }
    }

    #[test]
    fn test_with_mock_data() {
        // Given: Test item
        let item = create_test_menu_item();
        
        // When: Access properties
        let id = &item.id;
        
        // Then: Verify values
        assert_eq!(id, "test-123");
    }
}
```

**Benefits of mock data helpers:**
- Reduces code duplication
- Ensures consistent test data
- Simplifies test setup

---

## 📂 Section 3: Test Organisation

### 3.1: Nested Test Modules

**Purpose:** Group related tests for better organisation.

**Structure:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Group tests by functionality
    mod creation {
        use super::*;

        #[test]
        fn creates_valid_item() {
            // Test implementation
        }

        #[test]
        fn fails_with_invalid_data() {
            // Test implementation
        }
    }

    mod validation {
        use super::*;

        #[test]
        fn validates_required_fields() {
            // Test implementation
        }

        #[test]
        fn rejects_invalid_format() {
            // Test implementation
        }
    }
}
```

**Benefits:**
- Logical grouping of related tests
- Easier to locate specific tests
- Clear test organisation

### 3.2: Integration Test Directory Structure

**Recommended structure:**

```
tests/
├── api/
│   ├── auth_tests.rs          # Authentication endpoint tests
│   ├── items_tests.rs         # Menu item endpoint tests
│   └── notices_tests.rs       # Notice endpoint tests
├── integration/
│   ├── storage_tests.rs       # Storage integration tests
│   └── scheduler_tests.rs     # Scheduler integration tests
└── common/
    └── mod.rs                 # Shared test utilities
```

**Purpose of each directory:**
- `api/`: Tests for HTTP endpoints
- `integration/`: Tests for component integration
- `common/`: Shared test utilities and helpers

---

## 📊 Section 4: Test Coverage

### 4.1: Coverage Goals

**Minimum coverage requirements:**

| Code Type | Minimum Coverage | Target Coverage | Reasoning |
|-----------|-----------------|-----------------|-----------|
| New features | 70% | 80% | Ensures basic quality |
| Bug fixes | Include regression test | N/A | Prevents bug recurrence |
| Refactoring | Maintain existing | Improve if possible | No coverage regression |
| Critical paths | 90%+ | 95%+ | Authentication, data storage |

### 4.2: Generating Coverage Reports

**Step 4.2.1: Generate HTML Report**

```bash
# Generate HTML coverage report
cargo tarpaulin --out Html --output-dir coverage

# Open report in browser
open coverage/index.html  # macOS
xdg-open coverage/index.html  # Linux
start coverage/index.html  # Windows
```

**What the report shows:**
- Percentage of lines covered by tests
- Which lines are executed during tests
- Which lines are never executed
- Coverage by file and function

**Step 4.2.2: Generate Console Report**

```bash
# Display coverage in terminal
cargo tarpaulin --out Stdout
```

**Step 4.2.3: Enforce Coverage Threshold**

```bash
# Fail if coverage below 80%
cargo tarpaulin --fail-under 80
```

**Use case:** Add to CI/CD pipeline to maintain quality.

### 4.3: Coverage Exclusions

**Purpose:** Exclude debug/development code from coverage requirements.

**Example:**

```rust
#[cfg(not(tarpaulin_include))]
fn debug_helper() {
    // This function is excluded from coverage analysis
    println!("Debug information");
}
```

**When to exclude:**
- Debug logging functions
- Development-only utilities
- Unreachable error paths
- Platform-specific code

---

## ✅ Section 5: Best Practices

### 5.1: Test Naming Conventions

**Format:** `test_<what>_<when>_<expected>`

**Good examples:**

```rust
#[test]
fn test_user_authentication_succeeds_with_valid_credentials() {
    // Test implementation
}

#[test]
fn test_menu_item_validation_fails_for_empty_name() {
    // Test implementation
}

#[test]
fn test_schedule_creation_requires_valid_date() {
    // Test implementation
}
```

**Poor examples to avoid:**

```rust
#[test]
fn test_auth() {
    // ❌ Too vague - what about auth is being tested?
}

#[test]
fn test1() {
    // ❌ No information about what is tested
}

#[test]
fn it_works() {
    // ❌ Unclear what functionality is tested
}
```

**Naming benefits:**
- Immediately understand test purpose
- Easier to locate specific tests
- Self-documenting test suite

### 5.2: Arrange-Act-Assert Pattern

**Purpose:** Structure tests in three clear phases.

**Pattern structure:**

```rust
#[test]
fn test_item_creation() {
    // Arrange: Set up test data
    let name = "Test Item";
    let category = "Test Category";
    
    // Act: Execute the function being tested
    let item = MenuItem::new(name, category);
    
    // Assert: Verify the results
    assert_eq!(item.name, name);
    assert_eq!(item.category, category);
}
```

**Benefits:**
- Clear test structure
- Easy to understand test flow
- Consistent test format

### 5.3: Testing Error Cases

**Purpose:** Verify functions handle errors correctly.

**Example error tests:**

```rust
#[test]
fn test_validation_error() {
    // Given: Invalid input
    let email = "";
    
    // When: Validate email
    let result = validate_email(email);
    
    // Then: Should return error
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Email cannot be empty"
    );
}

#[test]
#[should_panic(expected = "Invalid configuration")]
fn test_panics_with_invalid_config() {
    // This test expects a panic with specific message
    load_config("invalid.json");
}
```

**Error testing strategies:**
- Test all error conditions
- Verify error messages
- Use `#[should_panic]` for expected panics
- Test error recovery mechanisms

### 5.4: Testing Asynchronous Code

**Pattern for async tests:**

```rust
#[actix_web::test]
async fn test_database_query() {
    // Arrange: Set up test database
    let db = setup_test_database().await;
    
    // Act: Execute async query
    let result = db.query_items().await;
    
    // Assert: Verify results
    assert!(result.is_ok());
    
    // Cleanup: Remove test data
    cleanup_test_database(db).await;
}
```

**Async testing guidelines:**
- Always clean up async resources
- Use `.await` for async operations
- Handle async errors appropriately
- Consider timeout scenarios

### 5.5: Parameterised Tests

**Purpose:** Test multiple inputs with same logic.

**Example:**

```rust
#[test]
fn test_validation_with_multiple_inputs() {
    // Given: Test cases (input, expected_result)
    let test_cases = vec![
        ("", false),                    // Empty string invalid
        ("a", false),                   // Single character invalid
        ("valid@email.com", true),      // Valid email
        ("no-at-sign.com", false),      // Missing @ symbol
        ("@nodomain.com", false),       // Missing local part
    ];
    
    // When & Then: Test each case
    for (input, expected) in test_cases {
        let result = validate_email(input);
        assert_eq!(
            result.is_ok(),
            expected,
            "Failed for input: '{}'",
            input
        );
    }
}
```

**Benefits:**
- Test multiple scenarios efficiently
- Reduce code duplication
- Easy to add new test cases

### 5.6: Test Fixtures

**Purpose:** Provide consistent test setup and teardown.

**Example test fixture:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext {
        app: TestApp,
        database: TestDatabase,
    }

    impl TestContext {
        fn new() -> Self {
            Self {
                app: create_test_app(),
                database: create_test_database(),
            }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            // Automatic cleanup when test completes
            self.database.cleanup();
        }
    }

    #[test]
    fn test_with_context() {
        // Given: Test context (automatically cleaned up)
        let ctx = TestContext::new();
        
        // When: Use context in test
        let result = ctx.app.process_request();
        
        // Then: Verify result
        assert!(result.is_ok());
        
        // Context automatically cleaned up here
    }
}
```

**Fixture benefits:**
- Consistent test setup
- Automatic cleanup
- Reduced boilerplate code

---

## 🔍 Section 6: Testing Specific Components

### 6.1: Testing HTTP Handlers

**Purpose:** Verify request handlers return correct responses.

**Example:**

```rust
#[actix_web::test]
async fn test_get_items_handler() {
    // Given: Test application with handler
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::default()))
            .service(get_items)
    ).await;

    // When: Send GET request
    let req = test::TestRequest::get()
        .uri("/api/items")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    // Then: Verify response
    assert_eq!(resp.status(), StatusCode::OK);
}
```

**What this tests:**
- Handler responds to correct HTTP method
- Handler returns expected status code
- Handler processes requests correctly

### 6.2: Testing Authentication

**Purpose:** Verify protected endpoints require authentication.

**Example:**

```rust
#[actix_web::test]
async fn test_protected_endpoint_requires_auth() {
    // Given: Application with authentication middleware
    let app = test::init_service(
        App::new()
            .wrap(SessionMiddleware::new())
            .service(protected_handler)
    ).await;

    // When: Request without authentication
    let req = test::TestRequest::get()
        .uri("/admin/dashboard")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    // Then: Should return unauthorised
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
```

**Authentication test scenarios:**
- Unauthenticated access rejected
- Valid credentials accepted
- Invalid credentials rejected
- Session expiration handled correctly

### 6.3: Testing Data Storage

**Purpose:** Verify data persistence operations work correctly.

**Example:**

```rust
#[test]
fn test_save_and_load() {
    // Given: Test item
    let item = create_test_menu_item();
    
    // When: Save item
    let save_result = storage::save_item(&item);
    assert!(save_result.is_ok());
    
    // And: Load item
    let load_result = storage::load_item(&item.id);
    
    // Then: Loaded item matches saved item
    assert!(load_result.is_ok());
    assert_eq!(load_result.unwrap(), item);
}
```

**Storage test scenarios:**
- Save operation succeeds
- Load operation retrieves correct data
- Update operation modifies correctly
- Delete operation removes data
- Concurrent access handled safely

---

## 🚀 Section 7: Continuous Integration

### 7.1: GitHub Actions Configuration

**Purpose:** Automatically run tests on every commit and pull request.

**Example workflow file (`.github/workflows/test.yml`):**

```yaml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v2
      
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Run tests
        run: cargo test --all-features
      
      - name: Generate coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml
      
      - name: Upload coverage
        uses: codecov/codecov-action@v1
        with:
          file: ./cobertura.xml
```

**What this workflow does:**
1. Triggers on push and pull request events
2. Checks out code
3. Installs Rust toolchain
4. Runs all tests
5. Generates coverage report
6. Uploads coverage to Codecov

### 7.2: Local CI Simulation

**Command to simulate CI locally:**

```bash
# Run the same checks as CI
cargo fmt -- --check  # Check formatting
cargo clippy -- -D warnings  # Check for linting issues
cargo test --all-features  # Run all tests
```

**When to run:** Before pushing commits to avoid CI failures.

---

## 🎯 Section 8: Next Steps

After understanding testing practices, proceed with these tasks:

### For New Contributors

Complete these tasks in sequence:

1. **Write First Test** – Add a simple unit test to understand the process
2. **Run Test Suite** – Execute `cargo test` to see all tests pass
3. **Review Test Coverage** – Generate coverage report and review results
4. **Practice Test Writing** – Add tests for a new feature or bug fix

### For Experienced Developers

Complete these tasks in sequence:

1. **Improve Coverage** – Identify uncovered code and add tests
2. **Add Integration Tests** – Create end-to-end test scenarios
3. **Optimise Test Performance** – Identify and improve slow tests
4. **Document Test Patterns** – Share effective testing approaches with team

---

## 📖 Related Documentation

Access these resources for additional information:

- **[Development Setup Guide](setup.md)** – Set up your development environment
- **[Contributing Guidelines](contributing.md)** – How to contribute to the project
- **[API Reference](../api/reference.md)** – API documentation for testing endpoints
- **[Design Documentation](../architecture/design.md)** – Architecture overview for integration tests

---

[← Back to Development](README.md) | [Documentation Home](../README.md)