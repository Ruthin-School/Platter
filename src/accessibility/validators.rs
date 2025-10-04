//! Main validation orchestration

use crate::accessibility::{AccessibilityReport, color_contrast};
use std::path::Path;

/// Validate all accessibility requirements for a template
pub fn validate_all(content: &str, file_path: &str) -> AccessibilityReport {
    let mut report = AccessibilityReport::new(file_path.to_string());

    // Run all validators
    super::template_analyzer::analyze_template(content, &mut report);
    super::aria_validator::validate_aria(content, &mut report);
    super::form_validator::validate_forms(content, &mut report);
    super::semantic_validator::validate_semantics(content, &mut report);

    report
}

/// Validate CSS file for color contrast compliance
pub fn validate_css_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<super::AccessibilityError>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(color_contrast::validate_css_colors(&content))
}

/// Run all accessibility checks and return combined report
pub fn run_full_validation(templates_dir: &str, css_dir: &str) -> ValidationResult {
    let mut all_reports = Vec::new();
    let mut css_errors = Vec::new();

    // Validate templates
    all_reports.extend(super::validate_templates_directory(templates_dir));

    // Validate CSS files
    if let Ok(entries) = std::fs::read_dir(css_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "css")
                && let Ok(errors) = validate_css_file(&path)
            {
                css_errors.extend(errors);
            }
        }
    }

    ValidationResult {
        template_reports: all_reports,
        css_errors,
    }
}

/// Combined validation result
pub struct ValidationResult {
    pub template_reports: Vec<AccessibilityReport>,
    pub css_errors: Vec<super::AccessibilityError>,
}

impl ValidationResult {
    pub fn has_critical_errors(&self) -> bool {
        self.template_reports.iter().any(|r| r.has_errors()) || !self.css_errors.is_empty()
    }

    pub fn print_summary(&self) {
        let total_errors: usize = self
            .template_reports
            .iter()
            .map(|r| r.errors.len())
            .sum::<usize>()
            + self.css_errors.len();

        let total_warnings: usize = self.template_reports.iter().map(|r| r.warnings.len()).sum();

        println!("\n╔════════════════════════════════════════════════════════════════╗");
        println!("║     WCAG 2.1 Level AA Accessibility Validation Report         ║");
        println!("╚════════════════════════════════════════════════════════════════╝");

        if total_errors == 0 && total_warnings == 0 {
            println!("\n✅ All accessibility checks passed!");
            println!("   No violations found.");
        } else {
            println!("\n📊 Summary:");
            println!("   • Total Errors:   {}", total_errors);
            println!("   • Total Warnings: {}", total_warnings);
            println!("   • Files Checked:  {}", self.template_reports.len());

            // Print template reports
            for report in &self.template_reports {
                report.print_report();
            }

            // Print CSS errors
            if !self.css_errors.is_empty() {
                println!("\n📋 CSS Color Contrast Issues:");
                println!("{}", "=".repeat(80));
                for error in &self.css_errors {
                    println!("{}", error);
                }
                println!("{}", "=".repeat(80));
            }

            if total_errors > 0 {
                println!("\n❌ VALIDATION FAILED");
                println!(
                    "   {} critical accessibility error(s) must be fixed before deployment.",
                    total_errors
                );
            } else {
                println!("\n⚠️  VALIDATION PASSED WITH WARNINGS");
                println!(
                    "   Consider addressing {} warning(s) for better accessibility.",
                    total_warnings
                );
            }
        }

        println!("\n📚 Resources:");
        println!("   • WCAG 2.1 Guidelines: https://www.w3.org/WAI/WCAG21/quickref/");
        println!("   • WebAIM Contrast Checker: https://webaim.org/resources/contrastchecker/");
        println!("   • ARIA Authoring Practices: https://www.w3.org/WAI/ARIA/apg/");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_all() {
        let html = r##"
            <!DOCTYPE html>
            <html lang="en">
            <head><title>Test</title></head>
            <body>
                <a href="#main-content" class="skip-link">Skip to main content</a>
                <main id="main-content">
                    <h1>Test Page</h1>
                    <img src="test.jpg" alt="Test image">
                </main>
            </body>
            </html>
        "##;

        let report = validate_all(html, "test.html");
        assert!(!report.has_errors());
    }
}
