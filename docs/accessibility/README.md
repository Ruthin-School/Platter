# Accessibility Validation System

## Overview

The Platter accessibility validation system enforces WCAG 2.1 Level AA compliance through compile-time checks. This ensures that all UI components meet accessibility standards before deployment.

## Features

### ✅ Automated Validation

- **Compile-time checks**: Accessibility violations are caught during the build process
- **Semantic HTML validation**: Ensures proper HTML5 structure
- **ARIA validation**: Verifies correct ARIA roles and attributes
- **Color contrast checking**: Validates 4.5:1 ratio for normal text, 3:1 for large text
- **Form accessibility**: Checks label associations and proper input types
- **Keyboard navigation**: Validates tabindex and focus management
- **Image alt text**: Ensures all images have descriptive alternatives

### 📋 WCAG 2.1 Level AA Compliance

The system checks for:

1. **Perceivable**
   - Text alternatives for non-text content (1.1.1)
   - Color contrast ratios (1.4.3)
   - Text resize up to 200% (1.4.4)

2. **Operable**
   - Keyboard accessible (2.1.1)
   - Skip to content links (2.4.1)
   - Focus visible (2.4.7)
   - Target size minimum 44×44px (2.5.5)

3. **Understandable**
   - Language of page (3.1.1)
   - Labels or instructions (3.3.2)

4. **Robust**
   - Valid HTML and ARIA (4.1.1, 4.1.2)

## Usage

### Running Validation Manually

```bash
# Validate all templates and CSS
cargo run --bin a11y-check

# Skip accessibility checks (not recommended)
SKIP_A11Y_CHECK=1 cargo build
```

### Automatic Build-Time Validation

The validation runs automatically during:
- `cargo build`
- `cargo run`
- `cargo test`

In release mode, accessibility errors will **prevent compilation**.

### Understanding Error Messages

Each error includes:
- **Error code**: Unique identifier (e.g., A11Y-001)
- **Severity**: Critical, High, Medium, or Low
- **Line number**: Where the issue occurs
- **Description**: What the problem is
- **Remediation**: How to fix it
- **WCAG reference**: Related guideline

Example output:
```
❌ ERRORS (1):
  🔴 [A11Y-001] Image element missing alt attribute: logo.png (Line 42)
     📖 WCAG: 1.1.1 Non-text Content (Level A)
     💡 Fix: Add descriptive alt text: <img src="..." alt="Description of image">
```

## Validation Rules

### Critical Errors (Block Compilation in Release)

- **A11Y-001**: Missing alt text on images
- **A11Y-002**: Form inputs without labels
- **A11Y-004**: Insufficient color contrast
- **A11Y-005**: Missing lang attribute on HTML element
- **A11Y-012**: Buttons without accessible text

### High Priority Errors

- **A11Y-003**: Invalid heading hierarchy
- **A11Y-007**: Invalid ARIA roles
- **A11Y-008**: Missing required ARIA attributes
- **A11Y-010**: Missing focus indicators
- **A11Y-011**: Invalid tabindex values
- **A11Y-013**: Tables without proper headers

### Warnings (Recommended Fixes)

- **A11Y-W001**: Potential heading issues
- **A11Y-W002**: Missing landmark regions
- **A11Y-W003**: Information conveyed by color alone
- **A11Y-W005**: Generic link text ("click here", "read more")

## Best Practices

### Semantic HTML

✅ **Good**:
```html
<main>
  <article>
    <header>
      <h1>Page Title</h1>
    </header>
    <section>
      <h2>Section Title</h2>
      <p>Content...</p>
    </section>
  </article>
</main>
```

❌ **Bad**:
```html
<div class="main">
  <div class="article">
    <div class="header">
      <div class="title">Page Title</div>
    </div>
  </div>
</div>
```

### ARIA Attributes

✅ **Good**:
```html
<button aria-label="Close dialog">×</button>
<div role="alert" aria-live="polite">Status updated</div>
<input type="checkbox" aria-checked="false" role="checkbox">
```

❌ **Bad**:
```html
<div onclick="closeDialog()">×</div>
<div class="alert">Status updated</div>
<div class="checkbox"></div>
```

### Form Labels

✅ **Good**:
```html
<label for="email">Email Address</label>
<input type="email" id="email" name="email">

<!-- Or with aria-label -->
<input type="search" aria-label="Search products">
```

❌ **Bad**:
```html
<input type="email" placeholder="Email">
```

### Color Contrast

The system validates CSS custom properties for contrast:

```css
:root {
  /* ✅ Good: 4.54:1 ratio */
  --color-text: #333333;
  --color-background: #ffffff;
  
  /* ❌ Bad: 2.1:1 ratio */
  --color-text: #999999;
  --color-background: #ffffff;
}
```

### Keyboard Navigation

✅ **Good**:
```html
<button>Click me</button>
<a href="/page">Link</a>
<div role="button" tabindex="0" onclick="...">Custom Button</div>
```

❌ **Bad**:
```html
<div onclick="...">Click me</div>
<span class="link" onclick="...">Link</span>
```

## Configuration

### Disabling Checks (Development Only)

```bash
# Skip all accessibility checks
export SKIP_A11Y_CHECK=1
cargo build
```

### CI/CD Integration

The validation runs automatically in CI pipelines. Ensure:

1. No `SKIP_A11Y_CHECK` in CI environment
2. Use `cargo build --release` to enforce strict checks
3. Review warnings in CI logs

## Extending the System

### Adding Custom Validators

Create a new validator in `src/accessibility/`:

```rust
pub fn validate_custom(content: &str, report: &mut AccessibilityReport) {
    // Your validation logic
    if !content.contains("required-element") {
        report.add_error(AccessibilityError::custom(...));
    }
}
```

Register it in `src/accessibility/validators.rs`:

```rust
pub fn validate_all(content: &str, file_path: &str) -> AccessibilityReport {
    // ... existing validators
    custom_validator::validate_custom(content, &mut report);
    report
}
```

### Adding Error Types

Define new errors in `src/accessibility/error_types.rs`:

```rust
impl AccessibilityError {
    pub fn custom_error(line: Option<usize>) -> Self {
        Self {
            code: "A11Y-XXX".to_string(),
            severity: ErrorSeverity::Critical,
            line,
            message: "Description".to_string(),
            remediation: "How to fix".to_string(),
            wcag_reference: "X.X.X Guideline (Level X)".to_string(),
        }
    }
}
```

## Resources

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [MDN Accessibility](https://developer.mozilla.org/en-US/docs/Web/Accessibility)

## Testing

Run the test suite:

```bash
# Run all tests including accessibility validators
cargo test

# Run only accessibility tests
cargo test --lib accessibility
```

## Troubleshooting

### Build Fails with Accessibility Errors

1. Read the error messages carefully
2. Check the line numbers in your templates
3. Follow the remediation guidance
4. Refer to WCAG references for details

### False Positives

If you believe an error is incorrect:
1. Verify your HTML structure
2. Check ARIA specification
3. File an issue with details

### Performance Concerns

The validation adds minimal overhead:
- Runs only during build
- Skipped in incremental builds
- Can be disabled for rapid iteration (not recommended)

## Contributing

To improve the accessibility validation system:

1. Add new validation rules
2. Improve error messages
3. Add support for more WCAG criteria
4. Update documentation

See [CONTRIBUTING.md](../development/contributing.md) for details.