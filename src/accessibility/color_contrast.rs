//! Color contrast ratio validation for WCAG 2.1 Level AA compliance
//!
//! Normal text requires 4.5:1 contrast ratio
//! Large text (18pt+ or 14pt+ bold) requires 3:1 contrast ratio

use crate::accessibility::error_types::AccessibilityError;
use std::collections::HashMap;

/// RGB color representation
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');

        if hex.len() == 3 {
            // Short form: #abc -> #aabbcc
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some(Color { r, g, b })
        } else if hex.len() == 6 {
            // Full form: #aabbcc
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color { r, g, b })
        } else {
            None
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Calculate relative luminance according to WCAG formula
    /// https://www.w3.org/TR/WCAG21/#dfn-relative-luminance
    pub fn relative_luminance(&self) -> f64 {
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;

        let r_srgb = if r <= 0.03928 {
            r / 12.92
        } else {
            ((r + 0.055) / 1.055).powf(2.4)
        };

        let g_srgb = if g <= 0.03928 {
            g / 12.92
        } else {
            ((g + 0.055) / 1.055).powf(2.4)
        };

        let b_srgb = if b <= 0.03928 {
            b / 12.92
        } else {
            ((b + 0.055) / 1.055).powf(2.4)
        };

        0.2126 * r_srgb + 0.7152 * g_srgb + 0.0722 * b_srgb
    }
}

/// Calculate contrast ratio between two colors
/// https://www.w3.org/TR/WCAG21/#dfn-contrast-ratio
pub fn calculate_contrast_ratio(color1: &Color, color2: &Color) -> f64 {
    let l1 = color1.relative_luminance();
    let l2 = color2.relative_luminance();

    let lighter = l1.max(l2);
    let darker = l1.min(l2);

    (lighter + 0.05) / (darker + 0.05)
}

/// Text size classification for contrast requirements
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextSize {
    Normal, // Requires 4.5:1
    Large,  // Requires 3:1 (18pt+ or 14pt+ bold)
}

impl TextSize {
    pub fn required_contrast(&self) -> f64 {
        match self {
            TextSize::Normal => 4.5,
            TextSize::Large => 3.0,
        }
    }

    pub fn from_css(font_size: &str, font_weight: Option<&str>) -> Self {
        // Parse font size (simplified, handles px and pt)
        let size_value = font_size
            .trim_end_matches("px")
            .trim_end_matches("pt")
            .trim_end_matches("rem")
            .parse::<f64>()
            .unwrap_or(16.0);

        let is_bold = font_weight
            .map(|w| w == "bold" || w == "700" || w.parse::<i32>().unwrap_or(400) >= 700)
            .unwrap_or(false);

        // Convert to points if needed (assuming 16px = 12pt)
        let size_pt = if font_size.ends_with("pt") {
            size_value
        } else if font_size.ends_with("px") {
            size_value * 0.75
        } else if font_size.ends_with("rem") {
            size_value * 12.0
        } else {
            12.0
        };

        if size_pt >= 18.0 || (size_pt >= 14.0 && is_bold) {
            TextSize::Large
        } else {
            TextSize::Normal
        }
    }
}

/// Extract colors from CSS custom properties and validate contrast
pub fn validate_css_colors(css_content: &str) -> Vec<AccessibilityError> {
    let mut errors = Vec::new();
    let colors = extract_css_colors(css_content);

    // Define critical color pairs to check
    let critical_pairs = vec![
        ("--color-primary", "--color-neutral-0", TextSize::Normal),
        ("--color-secondary", "--color-neutral-0", TextSize::Normal),
        (
            "--color-neutral-900",
            "--color-neutral-100",
            TextSize::Normal,
        ),
        ("--color-success", "--color-neutral-0", TextSize::Normal),
        ("--color-warning", "--color-neutral-0", TextSize::Normal),
        ("--color-error", "--color-neutral-0", TextSize::Normal),
        ("--color-info", "--color-neutral-0", TextSize::Normal),
    ];

    for (fg_var, bg_var, text_size) in critical_pairs {
        if let (Some(fg), Some(bg)) = (colors.get(fg_var), colors.get(bg_var)) {
            let ratio = calculate_contrast_ratio(fg, bg);
            let required = text_size.required_contrast();

            if ratio < required {
                errors.push(AccessibilityError::low_color_contrast(
                    None,
                    ratio,
                    required,
                    &format!("{} on {}", fg_var, bg_var),
                ));
            }
        }
    }

    errors
}

/// Extract color values from CSS custom properties
fn extract_css_colors(css: &str) -> HashMap<String, Color> {
    let mut colors = HashMap::new();

    for line in css.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("--color-")
            && trimmed.contains(':')
            && let Some((name, value)) = trimmed.split_once(':')
        {
            let name = name.trim().to_string();
            let value = value.trim().trim_end_matches(';').trim();

            if let Some(color) = Color::from_hex(value) {
                colors.insert(name, color);
            }
        }
    }

    colors
}

/// Validate inline style color contrast
pub fn validate_inline_colors(
    foreground: &str,
    background: &str,
    text_size: TextSize,
) -> Option<AccessibilityError> {
    let fg = Color::from_hex(foreground)?;
    let bg = Color::from_hex(background)?;

    let ratio = calculate_contrast_ratio(&fg, &bg);
    let required = text_size.required_contrast();

    if ratio < required {
        Some(AccessibilityError::low_color_contrast(
            None,
            ratio,
            required,
            &format!("foreground {} on background {}", foreground, background),
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);

        let color = Color::from_hex("#abc").unwrap();
        assert_eq!(color.r, 170);
        assert_eq!(color.g, 187);
        assert_eq!(color.b, 204);
    }

    #[test]
    fn test_contrast_ratio() {
        // Pure black on pure white
        let black = Color::from_rgb(0, 0, 0);
        let white = Color::from_rgb(255, 255, 255);
        let ratio = calculate_contrast_ratio(&black, &white);
        assert!((ratio - 21.0).abs() < 0.1);

        // Same color has ratio of 1:1
        let ratio = calculate_contrast_ratio(&black, &black);
        assert!((ratio - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_wcag_aa_normal_text() {
        // #767676 on white has 4.54:1 ratio (passes AA for normal text)
        let gray = Color::from_hex("#767676").unwrap();
        let white = Color::from_hex("#FFFFFF").unwrap();
        let ratio = calculate_contrast_ratio(&gray, &white);
        assert!(ratio >= 4.5);
    }

    #[test]
    fn test_text_size_classification() {
        assert_eq!(TextSize::from_css("18pt", None), TextSize::Large);
        assert_eq!(TextSize::from_css("14pt", Some("bold")), TextSize::Large);
        assert_eq!(TextSize::from_css("16px", None), TextSize::Normal);
        assert_eq!(TextSize::from_css("24px", None), TextSize::Large);
    }
}
