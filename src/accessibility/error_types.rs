//! Error types for accessibility validation with remediation guidance

use std::fmt;

/// Critical accessibility errors that violate WCAG 2.1 Level AA
#[derive(Debug, Clone)]
pub struct AccessibilityError {
    pub code: String,
    pub severity: ErrorSeverity,
    pub line: Option<usize>,
    pub message: String,
    pub remediation: String,
    pub wcag_reference: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Critical, // Blocks compilation
    High,     // Blocks compilation
    Medium,   // Warning only
    Low,      // Warning only
}

impl fmt::Display for AccessibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity_icon = match self.severity {
            ErrorSeverity::Critical => "🔴",
            ErrorSeverity::High => "🟠",
            ErrorSeverity::Medium => "🟡",
            ErrorSeverity::Low => "🔵",
        };

        write!(f, "\n  {} [{}] {}", severity_icon, self.code, self.message)?;
        if let Some(line) = self.line {
            write!(f, " (Line {})", line)?;
        }
        write!(f, "\n     📖 WCAG: {}", self.wcag_reference)?;
        write!(f, "\n     💡 Fix: {}", self.remediation)?;
        Ok(())
    }
}

/// Potential accessibility issues that should be reviewed
#[derive(Debug, Clone)]
pub struct AccessibilityWarning {
    pub code: String,
    pub line: Option<usize>,
    pub message: String,
    pub suggestion: String,
}

impl fmt::Display for AccessibilityWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n  ⚠️  [{}] {}", self.code, self.message)?;
        if let Some(line) = self.line {
            write!(f, " (Line {})", line)?;
        }
        write!(f, "\n     💡 Suggestion: {}", self.suggestion)?;
        Ok(())
    }
}

impl AccessibilityError {
    pub fn missing_alt_text(line: Option<usize>, element: &str) -> Self {
        Self {
            code: "A11Y-001".to_string(),
            severity: ErrorSeverity::Critical,
            line,
            message: format!("Image element missing alt attribute: {}", element),
            remediation: "Add descriptive alt text: <img src=\"...\" alt=\"Description of image\">. Use alt=\"\" for decorative images.".to_string(),
            wcag_reference: "1.1.1 Non-text Content (Level A)".to_string(),
        }
    }

    pub fn missing_form_label(line: Option<usize>, input_id: &str) -> Self {
        Self {
            code: "A11Y-002".to_string(),
            severity: ErrorSeverity::Critical,
            line,
            message: format!("Form input missing associated label: {}", input_id),
            remediation: "Add a label: <label for=\"input-id\">Label Text</label> or use aria-label/aria-labelledby.".to_string(),
            wcag_reference: "1.3.1 Info and Relationships (Level A), 3.3.2 Labels or Instructions (Level A)".to_string(),
        }
    }

    pub fn invalid_heading_hierarchy(line: Option<usize>, found: &str, expected: &str) -> Self {
        Self {
            code: "A11Y-003".to_string(),
            severity: ErrorSeverity::High,
            line,
            message: format!(
                "Invalid heading hierarchy: found {} after {}",
                found, expected
            ),
            remediation:
                "Use headings in sequential order (h1 → h2 → h3, etc.). Don't skip levels."
                    .to_string(),
            wcag_reference:
                "1.3.1 Info and Relationships (Level A), 2.4.6 Headings and Labels (Level AA)"
                    .to_string(),
        }
    }

    pub fn low_color_contrast(
        line: Option<usize>,
        ratio: f64,
        required: f64,
        elements: &str,
    ) -> Self {
        Self {
            code: "A11Y-004".to_string(),
            severity: ErrorSeverity::Critical,
            line,
            message: format!(
                "Insufficient color contrast ratio {:.2}:1 (required: {:.1}:1) for {}",
                ratio, required, elements
            ),
            remediation: format!(
                "Increase contrast to at least {:.1}:1. Use tools like WebAIM Contrast Checker to find compliant colors.",
                required
            ),
            wcag_reference: "1.4.3 Contrast (Minimum) (Level AA)".to_string(),
        }
    }

    pub fn missing_lang_attribute(line: Option<usize>) -> Self {
        Self {
            code: "A11Y-005".to_string(),
            severity: ErrorSeverity::High,
            line,
            message: "Missing lang attribute on <html> element".to_string(),
            remediation: "Add lang attribute: <html lang=\"en\"> (use appropriate language code)."
                .to_string(),
            wcag_reference: "3.1.1 Language of Page (Level A)".to_string(),
        }
    }

    pub fn missing_skip_link(line: Option<usize>) -> Self {
        Self {
            code: "A11Y-006".to_string(),
            severity: ErrorSeverity::Medium,
            line,
            message: "Missing skip-to-content link for keyboard navigation".to_string(),
            remediation: "Add skip link: <a href=\"#main-content\" class=\"skip-link\">Skip to main content</a> at the start of <body>.".to_string(),
            wcag_reference: "2.4.1 Bypass Blocks (Level A)".to_string(),
        }
    }

    pub fn invalid_aria_role(line: Option<usize>, role: &str, element: &str) -> Self {
        Self {
            code: "A11Y-007".to_string(),
            severity: ErrorSeverity::High,
            line,
            message: format!("Invalid ARIA role '{}' on <{}> element", role, element),
            remediation: "Use valid ARIA roles from the ARIA specification. Check https://www.w3.org/TR/wai-aria-1.2/#role_definitions".to_string(),
            wcag_reference: "4.1.2 Name, Role, Value (Level A)".to_string(),
        }
    }

    pub fn missing_aria_attribute(line: Option<usize>, role: &str, required_attr: &str) -> Self {
        Self {
            code: "A11Y-008".to_string(),
            severity: ErrorSeverity::High,
            line,
            message: format!(
                "ARIA role '{}' requires '{}' attribute",
                role, required_attr
            ),
            remediation: format!(
                "Add required attribute: <element role=\"{}\" {}=\"...\">",
                role, required_attr
            ),
            wcag_reference: "4.1.2 Name, Role, Value (Level A)".to_string(),
        }
    }

    pub fn small_touch_target(line: Option<usize>, size: &str, element: &str) -> Self {
        Self {
            code: "A11Y-009".to_string(),
            severity: ErrorSeverity::Medium,
            line,
            message: format!("Touch target too small ({}): {}", size, element),
            remediation: "Ensure interactive elements are at least 44×44 pixels. Add padding or increase size in CSS.".to_string(),
            wcag_reference: "2.5.5 Target Size (Level AAA) - Best Practice for Level AA".to_string(),
        }
    }

    pub fn missing_focus_indicator(line: Option<usize>, element: &str) -> Self {
        Self {
            code: "A11Y-010".to_string(),
            severity: ErrorSeverity::High,
            line,
            message: format!("Interactive element may lack visible focus indicator: {}", element),
            remediation: "Ensure :focus styles are defined with visible outline or border. Example: .element:focus { outline: 2px solid blue; }".to_string(),
            wcag_reference: "2.4.7 Focus Visible (Level AA)".to_string(),
        }
    }

    pub fn invalid_tabindex(line: Option<usize>, value: i32) -> Self {
        Self {
            code: "A11Y-011".to_string(),
            severity: ErrorSeverity::High,
            line,
            message: format!("Invalid tabindex value: {}. Use 0, -1, or avoid tabindex on naturally focusable elements", value),
            remediation: "Use tabindex=\"0\" for custom interactive elements, tabindex=\"-1\" to remove from tab order. Avoid positive values.".to_string(),
            wcag_reference: "2.4.3 Focus Order (Level A)".to_string(),
        }
    }

    pub fn button_without_text(line: Option<usize>) -> Self {
        Self {
            code: "A11Y-012".to_string(),
            severity: ErrorSeverity::Critical,
            line,
            message: "Button element without accessible text content".to_string(),
            remediation: "Add text content, aria-label, or aria-labelledby: <button aria-label=\"Close\">×</button>".to_string(),
            wcag_reference: "4.1.2 Name, Role, Value (Level A)".to_string(),
        }
    }

    pub fn table_missing_headers(line: Option<usize>) -> Self {
        Self {
            code: "A11Y-013".to_string(),
            severity: ErrorSeverity::High,
            line,
            message: "Data table missing proper headers (<th> elements)".to_string(),
            remediation: "Use <th> elements with scope attribute: <th scope=\"col\">Header</th> or <th scope=\"row\">Header</th>".to_string(),
            wcag_reference: "1.3.1 Info and Relationships (Level A)".to_string(),
        }
    }

    pub fn redundant_title_attribute(line: Option<usize>, element: &str) -> Self {
        Self {
            code: "A11Y-014".to_string(),
            severity: ErrorSeverity::Low,
            line,
            message: format!("Redundant title attribute (duplicates visible text): {}", element),
            remediation: "Remove title if it duplicates visible text. Use title only for supplementary information.".to_string(),
            wcag_reference: "Best Practice - Avoid redundant attributes".to_string(),
        }
    }

    pub fn semantic_element_misuse(line: Option<usize>, element: &str, suggestion: &str) -> Self {
        Self {
            code: "A11Y-015".to_string(),
            severity: ErrorSeverity::Medium,
            line,
            message: format!("Non-semantic element used: {}", element),
            remediation: format!("Use semantic HTML: {}", suggestion),
            wcag_reference: "1.3.1 Info and Relationships (Level A)".to_string(),
        }
    }
}

impl AccessibilityWarning {
    pub fn potential_heading_issue(line: Option<usize>, context: &str) -> Self {
        Self {
            code: "A11Y-W001".to_string(),
            line,
            message: format!("Potential heading hierarchy issue: {}", context),
            suggestion: "Verify heading order is logical and sequential".to_string(),
        }
    }

    pub fn missing_landmark(line: Option<usize>, landmark: &str) -> Self {
        Self {
            code: "A11Y-W002".to_string(),
            line,
            message: format!("Missing landmark region: <{}>", landmark),
            suggestion: format!(
                "Consider adding <{}> element for better page structure",
                landmark
            ),
        }
    }

    pub fn color_only_distinction(line: Option<usize>, context: &str) -> Self {
        Self {
            code: "A11Y-W003".to_string(),
            line,
            message: format!("Information may be conveyed by color alone: {}", context),
            suggestion: "Ensure information is also conveyed through text, icons, or patterns"
                .to_string(),
        }
    }

    pub fn auto_playing_media(line: Option<usize>) -> Self {
        Self {
            code: "A11Y-W004".to_string(),
            line,
            message: "Media element may auto-play".to_string(),
            suggestion: "Ensure auto-playing media can be paused and has controls".to_string(),
        }
    }

    pub fn generic_link_text(line: Option<usize>, text: &str) -> Self {
        Self {
            code: "A11Y-W005".to_string(),
            line,
            message: format!("Generic link text found: '{}'", text),
            suggestion: "Use descriptive link text that makes sense out of context (avoid 'click here', 'read more')".to_string(),
        }
    }
}
