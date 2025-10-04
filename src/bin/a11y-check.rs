//! Accessibility validation CLI tool
//!
//! This binary runs all accessibility checks and reports violations.
//! It's used by the build script and can be run manually for validation.

use std::env;
use std::process;

fn main() {
    println!("🔍 Running WCAG 2.1 Level AA Accessibility Validation...\n");

    // Get the project root directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

    let templates_dir = format!("{}/templates", manifest_dir);
    let css_dir = format!("{}/static/css", manifest_dir);

    // Run validation
    let result = platter::accessibility::validators::run_full_validation(&templates_dir, &css_dir);

    // Print results
    result.print_summary();

    // Exit with appropriate code
    if result.has_critical_errors() {
        process::exit(1);
    } else {
        process::exit(0);
    }
}
