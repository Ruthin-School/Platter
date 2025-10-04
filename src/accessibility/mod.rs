//! # Accessibility Validation System
//!
//! Compile-time accessibility validation enforcing WCAG 2.1 Level AA compliance.
//! This module provides comprehensive checks for:
//! - Semantic HTML structure
//! - ARIA attributes and roles
//! - Color contrast ratios (4.5:1 for normal text, 3:1 for large text)
//! - Keyboard navigation support
//! - Alternative text for images and media
//! - Form label associations
//! - Heading hierarchy
//! - Touch target sizes (44x44 pixels minimum)
//! - Text resize compatibility (up to 200%)

pub mod aria_validator;
pub mod color_contrast;
pub mod error_types;
pub mod form_validator;
pub mod semantic_validator;
pub mod template_analyzer;
pub mod validators;

pub use error_types::*;
pub use validators::*;

use std::path::Path;

/// Main accessibility validation result
#[derive(Debug, Clone)]
pub struct AccessibilityReport {
    pub errors: Vec<AccessibilityError>,
    pub warnings: Vec<AccessibilityWarning>,
    pub file_path: String,
}

impl AccessibilityReport {
    pub fn new(file_path: String) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            file_path,
        }
    }

    pub fn add_error(&mut self, error: AccessibilityError) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: AccessibilityWarning) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn print_report(&self) {
        if self.has_errors() || self.has_warnings() {
            println!("\n📋 Accessibility Report: {}", self.file_path);
            println!("{}", "=".repeat(80));
        }

        if self.has_errors() {
            println!("\n❌ ERRORS ({}):", self.errors.len());
            for error in &self.errors {
                println!("{}", error);
            }
        }

        if self.has_warnings() {
            println!("\n⚠️  WARNINGS ({}):", self.warnings.len());
            for warning in &self.warnings {
                println!("{}", warning);
            }
        }

        if self.has_errors() || self.has_warnings() {
            println!("\n{}", "=".repeat(80));
        }
    }
}

/// Validate a template file for accessibility compliance
pub fn validate_template_file<P: AsRef<Path>>(
    path: P,
) -> Result<AccessibilityReport, std::io::Error> {
    let path_ref = path.as_ref();
    let content = std::fs::read_to_string(path_ref)?;
    let file_path = path_ref.display().to_string();

    let mut report = AccessibilityReport::new(file_path);

    // Run all validators
    template_analyzer::analyze_template(&content, &mut report);
    aria_validator::validate_aria(&content, &mut report);
    form_validator::validate_forms(&content, &mut report);
    semantic_validator::validate_semantics(&content, &mut report);

    Ok(report)
}

/// Validate all templates in a directory
pub fn validate_templates_directory<P: AsRef<Path>>(dir: P) -> Vec<AccessibilityReport> {
    let mut reports = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "html") {
                if let Ok(report) = validate_template_file(&path)
                    && (report.has_errors() || report.has_warnings())
                {
                    reports.push(report);
                }
            } else if path.is_dir() {
                reports.extend(validate_templates_directory(&path));
            }
        }
    }

    reports
}
