//! Platter - Dining Hall Management System
//!
//! This library provides core functionality for the Platter application,
//! including accessibility validation for WCAG 2.1 Level AA compliance.

pub mod accessibility;
pub mod config;

// Re-export commonly used types
pub use accessibility::{
    AccessibilityError, AccessibilityReport, AccessibilityWarning, validate_template_file,
    validate_templates_directory,
};

pub use config::{
    AdminConfig, AppSettings, ConfigError, ValidationRules,
};
