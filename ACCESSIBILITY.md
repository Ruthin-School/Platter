# 🌐 Accessibility Validation System

## Overview

Platter includes a comprehensive **compile-time accessibility validation system** that enforces **WCAG 2.1 Level AA compliance** across all UI components. This ensures that accessibility violations are caught during development, not in production.

## Key Features

### ✅ Compile-Time Validation
- **Automated checks** run during every build
- **Zero runtime overhead** - all validation happens at compile-time
- **Strict enforcement** in release builds prevents non-compliant code from deploying
- **Developer-friendly** error messages with remediation guidance

### 📋 WCAG 2.1 Level AA Compliance

The system validates:

| Category | Checks | WCAG Guidelines |
|----------|--------|-----------------|
| **Semantic HTML** | Proper structure, landmark regions, heading hierarchy | 1.3.1, 2.4.6 |
| **ARIA** | Valid roles, required attributes, proper usage | 4.1.2 |
| **Color Contrast** | 4.5:1 for normal text, 3:1 for large text | 1.4.3 |
| **Forms** | Label associations, input types, fieldsets | 3.3.2, 1.3.1 |
| **Images** | Alt text for all images and media | 1.1.1 |
| **Keyboard** | Focus management, tabindex, skip links | 2.1.1, 2.4.1, 2.4.7 |
| **Touch Targets** | Minimum 44×44 pixels | 2.5.5 |
| **Language** | Lang attribute on HTML element | 3.1.1 |

## Quick Start

### Run Accessibility Check

```bash
# Validate all templates and CSS
cargo run --bin a11y-check

# Or use the alias
cargo a11y
```

### Build with Validation

```bash
# Development build (warnings only)
cargo build

# Release build (errors block compilation)
cargo build --release
```

### Run Tests

```bash
# Run all tests including accessibility validators
cargo test
```

## Error Codes Reference

### Critical Errors (🔴 Block Compilation)

- **A11Y-001**: Missing alt text on images
- **A11Y-002**: Form inputs without labels
- **A11Y-004**: Insufficient color contrast
- **A11Y-012**: Buttons without accessible text

### High Priority Errors (🟠)

- **A11Y-003**: Invalid heading hierarchy
- **A11Y-005**: Missing lang attribute
- **A11Y-007**: Invalid ARIA roles
- **A11Y-008**: Missing required ARIA attributes
- **A11Y-010**: Missing focus indicators
- **A11Y-011**: Invalid tabindex values
- **A11Y-013**: Tables without headers
- **A11Y-015**: Semantic element misuse

### Warnings (⚠️ Recommended Fixes)

- **A11Y-W001**: Potential heading issues
- **A11Y-W002**: Missing landmark regions
- **A11Y-W003**: Color-only information
- **A11Y-W005**: Generic link text

## Example Output

```
🔍 Running WCAG 2.1 Level AA Accessibility Validation...

╔════════════════════════════════════════════════════════════════╗
║     WCAG 2.1 Level AA Accessibility Validation Report         ║
╚════════════════════════════════════════════════════════════════╝

📋 Accessibility Report: templates/menu.html
================================================================================

❌ ERRORS (2):
  🔴 [A11Y-001] Image element missing alt attribute: logo.png (Line 42)
     📖 WCAG: 1.1.1 Non-text Content (Level A)
     💡 Fix: Add descriptive alt text: <img src="..." alt="Description of image">

  🟠 [A11Y-003] Invalid heading hierarchy: found h3 after h1 (Line 58)
     📖 WCAG: 1.3.1 Info and Relationships (Level A)
     💡 Fix: Use headings in sequential order (h1 → h2 → h3, etc.)

⚠️  WARNINGS (1):
  ⚠️  [A11Y-W005] Generic link text found: 'click here' (Line 73)
     💡 Suggestion: Use descriptive link text that makes sense out of context

================================================================================

❌ VALIDATION FAILED
   2 critical accessibility error(s) must be fixed before deployment.

📚 Resources:
   • WCAG 2.1 Guidelines: https://www.w3.org/WAI/WCAG21/quickref/
   • WebAIM Contrast Checker: https://webaim.org/resources/contrastchecker/
   • ARIA Authoring Practices: https://www.w3.org/WAI/ARIA/apg/
```

## Configuration

### Environment Variables

```bash
# Skip accessibility checks (development only, not recommended)
export SKIP_A11Y_CHECK=1

# Build profile (affects strictness)
export PROFILE=release  # Strict: errors block compilation
export PROFILE=debug    # Lenient: errors are warnings
```

### Custom Cargo Commands

```bash
# Run accessibility validation
cargo a11y

# Skip validation for quick checks (not recommended)
cargo build-skip-a11y

# Run all tests
cargo test-all
```

## Architecture

### Module Structure

```
src/accessibility/
├── mod.rs                   # Main module and exports
├── validators.rs            # Validation orchestration
├── error_types.rs           # Error and warning types
├── template_analyzer.rs     # HTML structure validation
├── aria_validator.rs        # ARIA roles and attributes
├── form_validator.rs        # Form accessibility
├── semantic_validator.rs    # Semantic HTML patterns
└── color_contrast.rs        # Color contrast calculations
```

### Validation Flow

```
Build Process
    ↓
build.rs (compile-time)
    ↓
cargo run --bin a11y-check
    ↓
validators::run_full_validation()
    ↓
┌─────────────────────────────────┐
│  Template Analyzer              │
│  • Semantic HTML                │
│  • Heading hierarchy            │
│  • Images & alt text            │
│  • Skip links                   │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│  ARIA Validator                 │
│  • Valid roles                  │
│  • Required attributes          │
│  • Tabindex values              │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│  Form Validator                 │
│  • Label associations           │
│  • Input types                  │
│  • Fieldsets                    │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│  Semantic Validator             │
│  • Anti-patterns                │
│  • Semantic alternatives        │
│  • Focus management             │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│  Color Contrast Validator       │
│  • CSS custom properties        │
│  • Contrast ratios              │
│  • Text size considerations     │
└─────────────────────────────────┘
    ↓
Generate Report
    ↓
Exit Code (0 = pass, 1 = fail)
```

## Best Practices

### ✅ Do

- Use semantic HTML elements (`<header>`, `<main>`, `<nav>`, `<footer>`)
- Provide descriptive alt text for all images
- Ensure proper label associations for form inputs
- Maintain logical heading hierarchy (h1 → h2 → h3)
- Use sufficient color contrast (4.5:1 minimum)
- Include skip-to-content links
- Use ARIA attributes correctly
- Ensure keyboard accessibility with proper tabindex

### ❌ Don't

- Use `<div>` or `<span>` for clickable elements
- Skip heading levels (h1 → h3)
- Use positive tabindex values (>0)
- Rely on color alone to convey information
- Use generic link text ("click here", "read more")
- Create buttons without accessible text
- Implement custom controls without proper ARIA

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Accessibility Validation

on: [push, pull_request]

jobs:
  accessibility:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run Accessibility Validation
        run: cargo run --bin a11y-check
      
      - name: Run Tests
        run: cargo test --all
```

### GitLab CI Example

```yaml
accessibility:
  stage: test
  script:
    - cargo run --bin a11y-check
    - cargo test --all
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

## Extending the System

### Adding Custom Validators

1. Create a new validator module in `src/accessibility/`
2. Implement validation logic following existing patterns
3. Register in `validators.rs`
4. Add tests in `tests/accessibility_validation.rs`
5. Document in error types

See [docs/accessibility/README.md](docs/accessibility/README.md) for detailed extension guide.

## Testing

### Run Accessibility Tests

```bash
# All accessibility tests
cargo test --lib accessibility

# Specific test
cargo test test_missing_alt_text

# With output
cargo test -- --nocapture
```

### Coverage

Current test coverage includes:
- ✅ Semantic HTML validation
- ✅ ARIA role and attribute checking
- ✅ Form label associations
- ✅ Color contrast ratios
- ✅ Heading hierarchy
- ✅ Image alt text
- ✅ Keyboard accessibility

## Resources

### Official Guidelines
- [WCAG 2.1](https://www.w3.org/WAI/WCAG21/quickref/) - Web Content Accessibility Guidelines
- [ARIA 1.2](https://www.w3.org/TR/wai-aria-1.2/) - Accessible Rich Internet Applications
- [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/) - Design patterns and widgets

### Tools
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [axe DevTools](https://www.deque.com/axe/devtools/)
- [WAVE](https://wave.webaim.org/)

### Learning
- [MDN Accessibility](https://developer.mozilla.org/en-US/docs/Web/Accessibility)
- [WebAIM](https://webaim.org/)
- [A11y Project](https://www.a11yproject.com/)

## Troubleshooting

### Common Issues

**Build fails with accessibility errors**
- Read error messages carefully
- Check line numbers in templates
- Follow remediation guidance
- Consult WCAG references

**Too many false positives**
- Review HTML structure
- Verify ARIA usage against spec
- File an issue if validation is incorrect

**Slow build times**
- Validation runs only on template/CSS changes
- Use `cargo check-quick` for rapid iteration
- Consider `SKIP_A11Y_CHECK=1` for experimental work (not for commits)

## Support

For questions or issues:
1. Check [documentation](docs/accessibility/README.md)
2. Review [test examples](tests/accessibility_validation.rs)
3. Search [GitHub Issues](https://github.com/Ruthin-School/Platter/issues)
4. Create a new issue with details

## License

This accessibility validation system is part of Platter and licensed under AGPLv3.

---

**Built with accessibility in mind. Everyone deserves access to digital content.**