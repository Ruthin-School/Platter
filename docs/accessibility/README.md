# 🌐 Accessibility Validation System

> **Navigation:** [Documentation Home](../README.md) → Accessibility

> 📘 **Comprehensive Documentation:** For complete accessibility validation documentation, see [`ACCESSIBILITY.md`](../../ACCESSIBILITY.md) in the project root.

## Quick Reference

Platter includes a compile-time accessibility validation system that enforces WCAG 2.1 Level AA compliance.

### Running Validation

```bash
# Validate all templates and CSS
cargo run --bin a11y-check

# Or use the alias
cargo a11y
```

### Key Features

- ✅ **Compile-time validation** – Catches issues during build
- ✅ **WCAG 2.1 Level AA compliance** – Industry-standard accessibility
- ✅ **Zero runtime overhead** – All validation at compile-time
- ✅ **Developer-friendly errors** – Clear remediation guidance

### Error Codes

**Critical Errors (Block Compilation):**
- **A11Y-001**: Missing alt text on images
- **A11Y-002**: Form inputs without labels
- **A11Y-004**: Insufficient colour contrast
- **A11Y-012**: Buttons without accessible text

**High Priority Errors:**
- **A11Y-003**: Invalid heading hierarchy
- **A11Y-007**: Invalid ARIA roles
- **A11Y-008**: Missing required ARIA attributes

**Warnings:**
- **A11Y-W001**: Potential heading issues
- **A11Y-W002**: Missing landmark regions
- **A11Y-W005**: Generic link text

### Configuration

```bash
# Skip checks (development only, not recommended)
export SKIP_A11Y_CHECK=1
cargo build
```

## Extending the System

Developers can add custom validators by creating modules in [`src/accessibility/`](../../src/accessibility/) and registering them in [`validators.rs`](../../src/accessibility/validators.rs:1).

See the [main accessibility documentation](../../ACCESSIBILITY.md) for detailed extension instructions.

## Resources

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [MDN Accessibility](https://developer.mozilla.org/en-US/docs/Web/Accessibility)

## Related Documentation

- **[Main Accessibility Guide](../../ACCESSIBILITY.md)** – Complete accessibility validation documentation
- **[Contributing Guidelines](../development/contributing.md)** – How to contribute accessibility improvements
- **[Build Optimisation Guide](../guides/build-optimization.md)** – Feature flags and build configuration

---

[← Back to Documentation Home](../README.md)