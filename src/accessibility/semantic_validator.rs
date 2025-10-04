//! Semantic HTML validation

use crate::accessibility::{AccessibilityError, AccessibilityReport, AccessibilityWarning};
use regex::Regex;

/// Validate semantic HTML usage
pub fn validate_semantics(content: &str, report: &mut AccessibilityReport) {
    check_div_button_antipattern(content, report);
    check_semantic_alternatives(content, report);
    check_focus_management(content, report);
}

/// Check for <div> or <span> used as buttons (anti-pattern)
fn check_div_button_antipattern(content: &str, report: &mut AccessibilityReport) {
    let patterns = [
        (r#"<div[^>]*onclick[^>]*>"#, "div"),
        (r#"<span[^>]*onclick[^>]*>"#, "span"),
    ];

    for (pattern, element) in patterns.iter() {
        let regex = Regex::new(pattern).unwrap();

        for (line_num, line) in content.lines().enumerate() {
            for div_match in regex.find_iter(line) {
                let tag = div_match.as_str();

                // Check if it has role="button" which partially mitigates the issue
                if !tag.contains(r#"role="button""#) {
                    report.add_error(AccessibilityError::semantic_element_misuse(
                        Some(line_num + 1),
                        &format!("<{}> with onclick", element),
                        &format!("Use <button> instead of <{}> for clickable elements, or add role=\"button\" and tabindex=\"0\"", element),
                    ));
                } else if !tag.contains("tabindex=") {
                    report.add_warning(AccessibilityWarning::potential_heading_issue(
                        Some(line_num + 1),
                        &format!("<{}> with role=\"button\" should have tabindex=\"0\" for keyboard access", element),
                    ));
                }
            }
        }
    }
}

/// Check for opportunities to use semantic HTML
fn check_semantic_alternatives(content: &str, report: &mut AccessibilityReport) {
    let patterns = vec![
        (
            r#"<div[^>]*class="[^"]*header[^"]*"[^>]*>"#,
            "header",
            "<header>",
        ),
        (
            r#"<div[^>]*class="[^"]*footer[^"]*"[^>]*>"#,
            "footer",
            "<footer>",
        ),
        (
            r#"<div[^>]*class="[^"]*nav[^"]*"[^>]*>"#,
            "navigation",
            "<nav>",
        ),
        (
            r#"<div[^>]*class="[^"]*article[^"]*"[^>]*>"#,
            "article",
            "<article>",
        ),
        (
            r#"<div[^>]*class="[^"]*section[^"]*"[^>]*>"#,
            "section",
            "<section>",
        ),
    ];

    for (pattern, _name, suggestion) in patterns.iter() {
        let regex = Regex::new(pattern).unwrap();

        for (line_num, line) in content.lines().enumerate() {
            if regex.is_match(line) {
                report.add_warning(AccessibilityWarning::potential_heading_issue(
                    Some(line_num + 1),
                    &format!(
                        "Consider using {} instead of <div class=\"...\">",
                        suggestion
                    ),
                ));
            }
        }
    }
}

/// Check focus management for interactive elements
fn check_focus_management(content: &str, report: &mut AccessibilityReport) {
    // Check for elements that should be focusable but might not be
    let interactive_regex = Regex::new(r#"<(div|span)[^>]*(onclick|onkeypress)[^>]*>"#).unwrap();

    for (line_num, line) in content.lines().enumerate() {
        for cap in interactive_regex.captures_iter(line) {
            let element = &cap[1];
            let full_tag = cap.get(0).map(|m| m.as_str()).unwrap_or("");

            // Check if it has appropriate ARIA role and tabindex
            let has_role = full_tag.contains("role=");
            let has_tabindex = full_tag.contains("tabindex=");

            if !has_role || !has_tabindex {
                report.add_error(AccessibilityError::semantic_element_misuse(
                    Some(line_num + 1),
                    &format!("<{}> with event handler", element),
                    "Interactive elements must have appropriate ARIA role and tabindex for keyboard accessibility",
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_button_antipattern() {
        let html = r#"<div onclick="doSomething()">Click me</div>"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_div_button_antipattern(html, &mut report);
        assert!(report.has_errors());
    }

    #[test]
    fn test_div_with_button_role() {
        let html = r#"<div role="button" tabindex="0" onclick="doSomething()">Click me</div>"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_div_button_antipattern(html, &mut report);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_semantic_alternatives() {
        let html = r#"<div class="header">Header</div>"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_semantic_alternatives(html, &mut report);
        assert!(report.has_warnings());
    }
}
