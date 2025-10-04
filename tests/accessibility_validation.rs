//! Integration tests for accessibility validation system

use platter::accessibility::validators::validate_all;

#[test]
fn test_valid_template_passes() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Test Page</title>
        </head>
        <body>
            <a href="#main-content" class="skip-link">Skip to main content</a>
            <header>
                <h1>Main Title</h1>
            </header>
            <main id="main-content">
                <article>
                    <h2>Article Title</h2>
                    <p>Content goes here.</p>
                    <img src="image.jpg" alt="Descriptive alt text">
                </article>
            </main>
            <footer>
                <p>Footer content</p>
            </footer>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(
        !report.has_errors(),
        "Valid template should not have errors"
    );
}

#[test]
fn test_missing_alt_text_fails() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <img src="logo.png">
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(
        report.has_errors(),
        "Missing alt text should produce errors"
    );
    assert!(report.errors.iter().any(|e| e.code == "A11Y-001"));
}

#[test]
fn test_invalid_heading_hierarchy_fails() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <h1>Title</h1>
            <h3>Skipped h2</h3>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(
        report.has_errors(),
        "Invalid heading hierarchy should produce errors"
    );
    assert!(report.errors.iter().any(|e| e.code == "A11Y-003"));
}

#[test]
fn test_form_without_label_fails() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <form>
                <input type="text" id="username">
                <button type="submit">Submit</button>
            </form>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(
        report.has_errors(),
        "Form input without label should produce errors"
    );
    assert!(report.errors.iter().any(|e| e.code == "A11Y-002"));
}

#[test]
fn test_form_with_label_passes() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <form>
                <label for="username">Username</label>
                <input type="text" id="username">
                <button type="submit">Submit</button>
            </form>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    // Should not have A11Y-002 error
    assert!(!report.errors.iter().any(|e| e.code == "A11Y-002"));
}

#[test]
fn test_invalid_aria_role_fails() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <div role="invalid-role">Content</div>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(
        report.has_errors(),
        "Invalid ARIA role should produce errors"
    );
    assert!(report.errors.iter().any(|e| e.code == "A11Y-007"));
}

#[test]
fn test_button_without_text_fails() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <button></button>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(
        report.has_errors(),
        "Button without text should produce errors"
    );
    assert!(report.errors.iter().any(|e| e.code == "A11Y-012"));
}

#[test]
fn test_button_with_aria_label_passes() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <button aria-label="Close">×</button>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(!report.errors.iter().any(|e| e.code == "A11Y-012"));
}

#[test]
fn test_missing_lang_attribute_fails() {
    let html = r##"
        <!DOCTYPE html>
        <html>
        <body>
            <p>Content</p>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(
        report.has_errors(),
        "Missing lang attribute should produce errors"
    );
    assert!(report.errors.iter().any(|e| e.code == "A11Y-005"));
}

#[test]
fn test_div_button_antipattern_fails() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <div onclick="doSomething()">Click me</div>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(
        report.has_errors(),
        "Div with onclick should produce errors"
    );
    assert!(report.errors.iter().any(|e| e.code == "A11Y-015"));
}

#[test]
fn test_positive_tabindex_fails() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <div tabindex="5">Content</div>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(
        report.has_errors(),
        "Positive tabindex should produce errors"
    );
    assert!(report.errors.iter().any(|e| e.code == "A11Y-011"));
}

#[test]
fn test_generic_link_text_warns() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <a href="/page">click here</a>
        </body>
        </html>
    "##;

    let report = validate_all(html, "test.html");
    assert!(
        report.has_warnings(),
        "Generic link text should produce warnings"
    );
    assert!(report.warnings.iter().any(|w| w.code == "A11Y-W005"));
}
