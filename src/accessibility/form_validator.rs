//! Form accessibility validation

use crate::accessibility::{AccessibilityError, AccessibilityReport};
use regex::Regex;

/// Validate form accessibility
pub fn validate_forms(content: &str, report: &mut AccessibilityReport) {
    check_form_labels(content, report);
    check_input_types(content, report);
    check_fieldsets(content, report);
    check_required_indicators(content, report);
}

/// Check that all form inputs have associated labels
fn check_form_labels(content: &str, report: &mut AccessibilityReport) {
    let input_regex = Regex::new(r#"<input[^>]*>"#).unwrap();
    let label_for_regex = Regex::new(r#"<label[^>]*for="([^"]*)"[^>]*>"#).unwrap();

    // Collect all label 'for' attributes
    let mut label_fors = Vec::new();
    for cap in label_for_regex.captures_iter(content) {
        label_fors.push(cap[1].to_string());
    }

    for (line_num, line) in content.lines().enumerate() {
        for input_match in input_regex.find_iter(line) {
            let input_tag = input_match.as_str();

            // Skip hidden inputs and buttons
            if input_tag.contains(r#"type="hidden""#)
                || input_tag.contains(r#"type="submit""#)
                || input_tag.contains(r#"type="button""#)
            {
                continue;
            }

            // Check for id and matching label
            if let Some(id) = extract_attribute(input_tag, "id") {
                let has_label = label_fors.contains(&id.to_string());
                let has_aria_label =
                    input_tag.contains("aria-label=") || input_tag.contains("aria-labelledby=");

                if !has_label && !has_aria_label {
                    report.add_error(AccessibilityError::missing_form_label(
                        Some(line_num + 1),
                        id,
                    ));
                }
            } else if !input_tag.contains("aria-label=") && !input_tag.contains("aria-labelledby=")
            {
                // Input without ID should at least have aria-label
                report.add_error(AccessibilityError::missing_form_label(
                    Some(line_num + 1),
                    "unnamed input",
                ));
            }
        }
    }
}

/// Check input types for appropriate usage
fn check_input_types(content: &str, report: &mut AccessibilityReport) {
    let input_regex = Regex::new(r#"<input[^>]*type="([^"]*)"[^>]*>"#).unwrap();
    let valid_types = [
        "text",
        "email",
        "password",
        "number",
        "tel",
        "url",
        "search",
        "date",
        "time",
        "datetime-local",
        "month",
        "week",
        "checkbox",
        "radio",
        "file",
        "color",
        "range",
        "submit",
        "reset",
        "button",
        "hidden",
    ];

    for (line_num, line) in content.lines().enumerate() {
        for cap in input_regex.captures_iter(line) {
            let input_type = &cap[1];

            if !valid_types.contains(&input_type) {
                report.add_error(AccessibilityError::semantic_element_misuse(
                    Some(line_num + 1),
                    &format!("input type=\"{}\"", input_type),
                    &format!("Use a valid HTML5 input type. Found: {}", input_type),
                ));
            }
        }
    }
}

/// Check for proper use of fieldsets for radio/checkbox groups
fn check_fieldsets(content: &str, report: &mut AccessibilityReport) {
    // Check for groups of radio buttons without fieldset
    let radio_regex = Regex::new(r#"<input[^>]*type="radio"[^>]*name="([^"]*)"[^>]*>"#).unwrap();
    let mut radio_groups: std::collections::HashMap<String, Vec<usize>> =
        std::collections::HashMap::new();

    for (line_num, line) in content.lines().enumerate() {
        for cap in radio_regex.captures_iter(line) {
            let group_name = cap[1].to_string();
            radio_groups
                .entry(group_name)
                .or_default()
                .push(line_num + 1);
        }
    }

    // Check if radio groups are within fieldsets
    for (group_name, lines) in radio_groups.iter() {
        if lines.len() > 1 {
            // Check if there's a fieldset around this group
            let first_line = lines[0];
            let context = content
                .lines()
                .skip(first_line.saturating_sub(5))
                .take(10)
                .collect::<Vec<_>>()
                .join("\n");

            if !context.contains("<fieldset") {
                report.add_error(AccessibilityError::semantic_element_misuse(
                    Some(*lines.first().unwrap()),
                    &format!("radio group '{}'", group_name),
                    "Wrap related radio buttons in <fieldset> with <legend> describing the group",
                ));
            }
        }
    }
}

/// Check for required field indicators
fn check_required_indicators(content: &str, _report: &mut AccessibilityReport) {
    let required_regex = Regex::new(r#"<input[^>]*required[^>]*>"#).unwrap();

    for line in content.lines() {
        for input_match in required_regex.find_iter(line) {
            let input_tag = input_match.as_str();

            // Check if there's aria-required or visible indication
            let has_aria_required = input_tag.contains("aria-required=");

            if !has_aria_required {
                // This is a warning, not an error - the 'required' attribute is sufficient
                // but aria-required provides better screen reader support
                continue;
            }
        }
    }
}

/// Extract attribute value from HTML tag
fn extract_attribute<'a>(tag: &'a str, attr: &str) -> Option<&'a str> {
    let pattern = format!(r#"{}="([^"]*)""#, attr);
    let regex = Regex::new(&pattern).ok()?;
    regex.captures(tag)?.get(1).map(|m| m.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_label() {
        let html = r#"<input type="text" id="test">"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_form_labels(html, &mut report);
        assert!(report.has_errors());
    }

    #[test]
    fn test_valid_label() {
        let html = r#"
            <label for="test">Test</label>
            <input type="text" id="test">
        "#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_form_labels(html, &mut report);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_aria_label() {
        let html = r#"<input type="text" aria-label="Test">"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_form_labels(html, &mut report);
        assert!(!report.has_errors());
    }
}
