//! Build script for compile-time accessibility validation
//!
//! NOTE: Accessibility validation is now a separate tool (a11y-check binary).
//! Run it manually: cargo run --bin a11y-check --features validate-a11y
//!
//! This build script only tracks file changes for cache invalidation.

fn main() {
    // Only monitor template/css changes if validation is enabled
    #[cfg(feature = "validate-a11y")]
    {
        println!("cargo:rerun-if-changed=templates/");
        println!("cargo:rerun-if-changed=static/css/");
        println!(
            "cargo:warning=Remember to run: cargo run --bin a11y-check --features validate-a11y"
        );
    }

    // No-op for default builds - this is intentional for fast compile times
    #[cfg(not(feature = "validate-a11y"))]
    {
        println!("cargo:rerun-if-changed=build.rs");
    }
}
