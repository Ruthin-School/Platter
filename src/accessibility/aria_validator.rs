//! ARIA (Accessible Rich Internet Applications) validation

use crate::accessibility::{AccessibilityError, AccessibilityReport};
use regex::Regex;
use std::collections::HashMap;

/// Validate ARIA attributes and roles in HTML content
pub fn validate_aria(content: &str, report: &mut AccessibilityReport) {
    check_aria_roles(content, report);
    check_aria_required_attributes(content, report);
    check_tabindex(content, report);
}

/// Valid ARIA roles and their required attributes
fn get_aria_role_requirements() -> HashMap<&'static str, Vec<&'static str>> {
    let mut roles = HashMap::new();

    // Landmark roles
    roles.insert("banner", vec![]);
    roles.insert("complementary", vec![]);
    roles.insert("contentinfo", vec![]);
    roles.insert("main", vec![]);
    roles.insert("navigation", vec![]);
    roles.insert("region", vec!["aria-labelledby", "aria-label"]);
    roles.insert("search", vec![]);

    // Widget roles
    roles.insert("button", vec![]);
    roles.insert("checkbox", vec!["aria-checked"]);
    roles.insert("radio", vec!["aria-checked"]);
    roles.insert(
        "slider",
        vec!["aria-valuenow", "aria-valuemin", "aria-valuemax"],
    );
    roles.insert(
        "spinbutton",
        vec!["aria-valuenow", "aria-valuemin", "aria-valuemax"],
    );
    roles.insert("textbox", vec![]);
    roles.insert("combobox", vec!["aria-expanded"]);
    roles.insert("listbox", vec![]);
    roles.insert("menu", vec![]);
    roles.insert("menubar", vec![]);
    roles.insert("tab", vec!["aria-selected"]);
    roles.insert("tabpanel", vec![]);
    roles.insert("tablist", vec![]);
    roles.insert("tree", vec![]);
    roles.insert("treegrid", vec![]);
    roles.insert("grid", vec![]);

    // Document structure roles
    roles.insert("article", vec![]);
    roles.insert("cell", vec![]);
    roles.insert("columnheader", vec![]);
    roles.insert("definition", vec![]);
    roles.insert("directory", vec![]);
    roles.insert("document", vec![]);
    roles.insert("feed", vec![]);
    roles.insert("figure", vec![]);
    roles.insert("group", vec![]);
    roles.insert("heading", vec!["aria-level"]);
    roles.insert("img", vec![]);
    roles.insert("list", vec![]);
    roles.insert("listitem", vec![]);
    roles.insert("row", vec![]);
    roles.insert("rowgroup", vec![]);
    roles.insert("rowheader", vec![]);
    roles.insert("table", vec![]);

    // Live region roles
    roles.insert("alert", vec![]);
    roles.insert("log", vec![]);
    roles.insert("marquee", vec![]);
    roles.insert("status", vec![]);
    roles.insert("timer", vec![]);

    // Window roles
    roles.insert("alertdialog", vec![]);
    roles.insert("dialog", vec![]);

    roles
}

/// Check for valid ARIA roles
fn check_aria_roles(content: &str, report: &mut AccessibilityReport) {
    let role_regex = Regex::new(r#"role="([^"]*)""#).unwrap();
    let valid_roles = get_aria_role_requirements();

    for (line_num, line) in content.lines().enumerate() {
        for cap in role_regex.captures_iter(line) {
            let role = &cap[1];

            if !valid_roles.contains_key(role) {
                // Extract element name from context
                let element = extract_element_name(line).unwrap_or("unknown");
                report.add_error(AccessibilityError::invalid_aria_role(
                    Some(line_num + 1),
                    role,
                    element,
                ));
            }
        }
    }
}

/// Check for required ARIA attributes based on role
fn check_aria_required_attributes(content: &str, report: &mut AccessibilityReport) {
    let role_requirements = get_aria_role_requirements();
    let role_regex = Regex::new(r#"role="([^"]*)""#).unwrap();

    for (line_num, line) in content.lines().enumerate() {
        for cap in role_regex.captures_iter(line) {
            let role = &cap[1];

            if let Some(required_attrs) = role_requirements.get(role)
                && !required_attrs.is_empty()
            {
                // Check if at least one required attribute is present
                let has_required = required_attrs
                    .iter()
                    .any(|attr| line.contains(&format!("{}=", attr)));

                if !has_required {
                    let attrs_list = required_attrs.join(" or ");
                    report.add_error(AccessibilityError::missing_aria_attribute(
                        Some(line_num + 1),
                        role,
                        &attrs_list,
                    ));
                }
            }
        }
    }
}

/// Check tabindex values
fn check_tabindex(content: &str, report: &mut AccessibilityReport) {
    let tabindex_regex = Regex::new(r#"tabindex="([^"]*)""#).unwrap();

    for (line_num, line) in content.lines().enumerate() {
        for cap in tabindex_regex.captures_iter(line) {
            if let Ok(value) = cap[1].parse::<i32>()
                && value > 0
            {
                report.add_error(AccessibilityError::invalid_tabindex(
                    Some(line_num + 1),
                    value,
                ));
            }
        }
    }
}

/// Extract element name from an HTML line
fn extract_element_name(line: &str) -> Option<&str> {
    let element_regex = Regex::new(r"<(\w+)").ok()?;
    element_regex.captures(line)?.get(1).map(|m| m.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_aria_role() {
        let html = r#"<div role="invalid-role">Test</div>"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_aria_roles(html, &mut report);
        assert!(report.has_errors());
    }

    #[test]
    fn test_valid_aria_role() {
        let html = r#"<div role="button">Test</div>"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_aria_roles(html, &mut report);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_missing_required_aria_attribute() {
        let html = r#"<div role="checkbox">Test</div>"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_aria_required_attributes(html, &mut report);
        assert!(report.has_errors());
    }

    #[test]
    fn test_invalid_positive_tabindex() {
        let html = r#"<div tabindex="5">Test</div>"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_tabindex(html, &mut report);
        assert!(report.has_errors());
    }

    #[test]
    fn test_valid_tabindex() {
        let html = r#"<div tabindex="0">Test</div>"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_tabindex(html, &mut report);
        assert!(!report.has_errors());
    }
}
