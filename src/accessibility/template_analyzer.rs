//! Template analyzer for semantic HTML structure validation

use crate::accessibility::{AccessibilityError, AccessibilityReport, AccessibilityWarning};
use regex::Regex;

/// Analyze HTML template for accessibility issues
pub fn analyze_template(content: &str, report: &mut AccessibilityReport) {
    check_html_lang(content, report);
    check_skip_link(content, report);
    check_heading_hierarchy(content, report);
    check_images(content, report);
    check_buttons(content, report);
    check_links(content, report);
    check_landmark_regions(content, report);
    check_tables(content, report);
}

/// Check for lang attribute on html element
fn check_html_lang(content: &str, report: &mut AccessibilityReport) {
    let html_regex = Regex::new(r"<html[^>]*>").unwrap();

    if let Some(html_match) = html_regex.find(content) {
        let html_tag = html_match.as_str();
        let line = content[..html_match.start()].lines().count();

        if !html_tag.contains("lang=") {
            report.add_error(AccessibilityError::missing_lang_attribute(Some(line)));
        }
    }
}

/// Check for skip-to-content link
fn check_skip_link(content: &str, report: &mut AccessibilityReport) {
    let skip_patterns = [
        r##"<a[^>]*href="#main[^"]*"[^>]*>.*?skip.*?</a>"##,
        r#"<a[^>]*class="[^"]*skip[^"]*"[^>]*>"#,
    ];

    let has_skip_link = skip_patterns
        .iter()
        .any(|pattern| Regex::new(pattern).unwrap().is_match(content));

    if !has_skip_link && content.contains("<body") {
        let line = content
            .lines()
            .position(|l| l.contains("<body"))
            .map(|p| p + 1);
        report.add_error(AccessibilityError::missing_skip_link(line));
    }
}

/// Check heading hierarchy (h1 -> h2 -> h3, no skipping)
fn check_heading_hierarchy(content: &str, report: &mut AccessibilityReport) {
    let heading_regex = Regex::new(r"<(h[1-6])[^>]*>").unwrap();
    let mut last_level = 0;
    let mut found_h1 = false;

    for (line_num, line) in content.lines().enumerate() {
        for cap in heading_regex.captures_iter(line) {
            let heading = &cap[1];
            let current_level = heading.chars().last().unwrap().to_digit(10).unwrap() as i32;

            if current_level == 1 {
                if found_h1 {
                    report.add_warning(AccessibilityWarning::potential_heading_issue(
                        Some(line_num + 1),
                        "Multiple h1 elements found - ensure semantic hierarchy is correct",
                    ));
                }
                found_h1 = true;
            }

            if last_level > 0 && current_level > last_level + 1 {
                report.add_error(AccessibilityError::invalid_heading_hierarchy(
                    Some(line_num + 1),
                    heading,
                    &format!("h{}", last_level),
                ));
            }

            last_level = current_level;
        }
    }

    if !found_h1 && content.contains("<main") {
        report.add_warning(AccessibilityWarning::potential_heading_issue(
            None,
            "No h1 element found - page should have a main heading",
        ));
    }
}

/// Check images for alt text
fn check_images(content: &str, report: &mut AccessibilityReport) {
    let img_regex = Regex::new(r"<img[^>]*>").unwrap();

    for (line_num, line) in content.lines().enumerate() {
        for img_match in img_regex.find_iter(line) {
            let img_tag = img_match.as_str();

            if !img_tag.contains("alt=") {
                let src = extract_attribute(img_tag, "src").unwrap_or("unknown");
                report.add_error(AccessibilityError::missing_alt_text(
                    Some(line_num + 1),
                    src,
                ));
            }
        }
    }
}

/// Check buttons for accessible text
fn check_buttons(content: &str, report: &mut AccessibilityReport) {
    let button_regex = Regex::new(r"<button[^>]*>(.*?)</button>").unwrap();

    for (line_num, line) in content.lines().enumerate() {
        for cap in button_regex.captures_iter(line) {
            let button_content = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let full_tag = cap.get(0).map(|m| m.as_str()).unwrap_or("");

            // Check if button has text content or aria-label
            let has_text = !button_content.trim().is_empty();

            let has_aria_label =
                full_tag.contains("aria-label=") || full_tag.contains("aria-labelledby=");

            if !has_text && !has_aria_label {
                report.add_error(AccessibilityError::button_without_text(Some(line_num + 1)));
            }
        }
    }
}

/// Check links for descriptive text
fn check_links(content: &str, report: &mut AccessibilityReport) {
    let link_regex = Regex::new(r"<a[^>]*>(.*?)</a>").unwrap();
    let generic_texts = ["click here", "here", "read more", "more", "link"];

    for (line_num, line) in content.lines().enumerate() {
        for cap in link_regex.captures_iter(line) {
            let link_text = cap
                .get(1)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let trimmed = link_text.trim();

            if generic_texts.contains(&trimmed) {
                report.add_warning(AccessibilityWarning::generic_link_text(
                    Some(line_num + 1),
                    trimmed,
                ));
            }
        }
    }
}

/// Check for landmark regions
fn check_landmark_regions(content: &str, report: &mut AccessibilityReport) {
    let landmarks = [
        ("header", "<header"),
        ("main", "<main"),
        ("nav", "<nav"),
        ("footer", "<footer"),
    ];

    for (name, tag) in landmarks.iter() {
        if !content.contains(tag) && content.contains("<body") {
            if *name == "main" {
                // Main is critical
                let line = content
                    .lines()
                    .position(|l| l.contains("<body"))
                    .map(|p| p + 1);
                report.add_error(AccessibilityError::semantic_element_misuse(
                    line,
                    "missing <main>",
                    "Add <main> element to wrap primary page content",
                ));
            } else {
                report.add_warning(AccessibilityWarning::missing_landmark(None, name));
            }
        }
    }
}

/// Check tables for proper headers
fn check_tables(content: &str, report: &mut AccessibilityReport) {
    let table_regex = Regex::new(r"<table[^>]*>.*?</table>").unwrap();

    for table_match in table_regex.find_iter(content) {
        let table_content = table_match.as_str();
        let line = content[..table_match.start()].lines().count();

        // Check if table has <th> elements
        if !table_content.contains("<th") {
            report.add_error(AccessibilityError::table_missing_headers(Some(line)));
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
    fn test_missing_alt_text() {
        let html = r#"<img src="test.jpg">"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_images(html, &mut report);
        assert!(report.has_errors());
    }

    #[test]
    fn test_valid_alt_text() {
        let html = r#"<img src="test.jpg" alt="Description">"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_images(html, &mut report);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_heading_hierarchy() {
        let html = r#"
            <h1>Title</h1>
            <h3>Skipped h2</h3>
        "#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_heading_hierarchy(html, &mut report);
        assert!(report.has_errors());
    }

    #[test]
    fn test_button_without_text() {
        let html = r#"<button></button>"#;
        let mut report = AccessibilityReport::new("test.html".to_string());
        check_buttons(html, &mut report);
        assert!(report.has_errors());
    }
}
