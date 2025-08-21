//
// ███████╗ ██████╗ ███╗   ██╗████████╗    ███╗   ███╗███████╗████████╗██████╗ ██╗ ██████╗███████╗
// ██╔════╝██╔═══██╗████╗  ██║╚══██╔══╝    ████╗ ████║██╔════╝╚══██╔══╝██╔══██╗██║██╔════╝██╔════╝
// █████╗  ██║   ██║██╔██╗ ██║   ██║       ██╔████╔██║█████╗     ██║   ██████╔╝██║██║     ███████╗
// ██╔══╝  ██║   ██║██║╚██╗██║   ██║       ██║╚██╔╝██║██╔══╝     ██║   ██╔══██╗██║██║     ╚════██║
// ██║     ╚██████╔╝██║ ╚████║   ██║       ██║ ╚═╝ ██║███████╗   ██║   ██║  ██║██║╚██████╗███████║
// ╚═╝      ╚═════╝ ╚═╝  ╚═══╝   ╚═╝       ╚═╝     ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝ ╚═════╝╚══════╝
//                                                app/src/font_metrics.rs

use crate::coordinates::{PhysicalCoord, CoordinateSystem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

/// PDF-standard font families supported for legal documents
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StandardFont {
    Helvetica,
    Times,
    Courier,
    TimesRoman,
    HelveticaBold,
    CourierBold,
}

impl StandardFont {
    /// Get CSS font family string for this standard font
    pub fn css_family(&self) -> &'static str {
        match self {
            StandardFont::Helvetica => "Helvetica, Arial, sans-serif",
            StandardFont::Times => "Times, 'Times New Roman', serif",
            StandardFont::Courier => "'Courier New', Courier, monospace",
            StandardFont::TimesRoman => "Times, 'Times New Roman', serif",
            StandardFont::HelveticaBold => "Helvetica, Arial, sans-serif",
            StandardFont::CourierBold => "'Courier New', Courier, monospace",
        }
    }

    /// Get font weight for this standard font
    pub fn css_weight(&self) -> &'static str {
        match self {
            StandardFont::HelveticaBold | StandardFont::CourierBold => "bold",
            _ => "normal",
        }
    }
}

/// Precise font metrics measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontMetrics {
    /// Font size in points
    pub font_size_pt: f64,
    /// Font size in CSS pixels
    pub font_size_px: f64,
    /// Ascent from baseline in CSS pixels
    pub ascent_px: f64,
    /// Descent from baseline in CSS pixels (positive value)
    pub descent_px: f64,
    /// Line height in CSS pixels
    pub line_height_px: f64,
    /// Character width for monospace fonts (CSS pixels)
    pub char_width_px: Option<f64>,
    /// Baseline offset from top of text box (CSS pixels)
    pub baseline_offset_px: f64,
    /// Font family used
    pub font_family: StandardFont,
    /// Measurement timestamp
    pub timestamp: u64,
}

/// Font metrics calculator for precise text positioning
pub struct FontMetricsCalculator {
    /// Canvas context for font measurements
    context: Option<CanvasRenderingContext2d>,
    /// Cached font metrics by font and size
    metrics_cache: HashMap<(StandardFont, u32), FontMetrics>,
}

impl FontMetricsCalculator {
    /// Create new font metrics calculator
    pub fn new() -> Self {
        Self {
            context: Self::create_measurement_context(),
            metrics_cache: HashMap::new(),
        }
    }

    /// Create hidden canvas context for font measurements
    fn create_measurement_context() -> Option<CanvasRenderingContext2d> {
        let window = web_sys::window()?;
        let document = window.document()?;
        
        // Create hidden canvas for measurements
        let canvas = document
            .create_element("canvas")
            .ok()?
            .dyn_into::<HtmlCanvasElement>()
            .ok()?;
        
        canvas.set_width(100);
        canvas.set_height(100);
        
        // Cast to HtmlElement to access style
        let element: HtmlElement = canvas.clone().dyn_into().ok()?;
        element.style().set_property("display", "none").ok()?;
        
        document.body()?.append_child(&canvas).ok()?;
        
        canvas
            .get_context("2d")
            .ok()?
            .and_then(|ctx| ctx.dyn_into::<CanvasRenderingContext2d>().ok())
    }

    /// Measure font metrics for given font and size
    pub fn measure_font(&mut self, font: StandardFont, size_pt: f64) -> Option<FontMetrics> {
        let size_key = (size_pt * 10.0) as u32; // Cache key with 0.1pt precision
        let cache_key = (font.clone(), size_key);
        
        // Check cache first
        if let Some(cached) = self.metrics_cache.get(&cache_key) {
            return Some(cached.clone());
        }

        let context = self.context.as_ref()?;
        
        // Convert points to pixels (1pt = 96/72 px at 96 DPI)
        let size_px = size_pt * 96.0 / 72.0;
        
        // Set font on canvas context
        let font_string = format!("{} {}px {}", 
            font.css_weight(), 
            size_px, 
            font.css_family()
        );
        context.set_font(&font_string);
        
        // Measure text metrics using canvas
        let test_text = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let text_metrics = context.measure_text(test_text).ok()?;
        
        // Calculate font metrics
        let ascent = text_metrics.actual_bounding_box_ascent();
        let descent = text_metrics.actual_bounding_box_descent();
        let line_height = ascent + descent;
        let baseline_offset = ascent;
        
        // For monospace fonts, calculate character width
        let char_width = if matches!(font, StandardFont::Courier | StandardFont::CourierBold) {
            Some(context.measure_text("M").ok()?.width())
        } else {
            None
        };
        
        let metrics = FontMetrics {
            font_size_pt: size_pt,
            font_size_px: size_px,
            ascent_px: ascent,
            descent_px: descent,
            line_height_px: line_height,
            char_width_px: char_width,
            baseline_offset_px: baseline_offset,
            font_family: font.clone(),
            timestamp: js_sys::Date::now() as u64,
        };
        
        // Cache the result
        self.metrics_cache.insert(cache_key, metrics.clone());
        
        Some(metrics)
    }

    /// Calculate text positioning for form field alignment
    pub fn calculate_text_position(
        &mut self,
        font: StandardFont,
        size_pt: f64,
        field_position: PhysicalCoord,
        field_height_mm: f64,
        coord_system: &CoordinateSystem,
    ) -> Option<TextPosition> {
        let metrics = self.measure_font(font, size_pt)?;
        
        // Convert field position to screen coordinates
        let field_screen = coord_system.physical_to_screen(field_position);
        let field_height_px = coord_system.mm_to_px(field_height_mm);
        
        // Calculate vertical centering within field
        let text_center_y = field_screen.y + (field_height_px / 2.0);
        let baseline_y = text_center_y + (metrics.line_height_px / 4.0); // Slight adjustment for visual centering
        
        Some(TextPosition {
            baseline_x: field_screen.x,
            baseline_y,
            font_metrics: metrics,
        })
    }

    /// Get cached metrics if available
    pub fn get_cached_metrics(&self, font: &StandardFont, size_pt: f64) -> Option<&FontMetrics> {
        let size_key = (size_pt * 10.0) as u32;
        let cache_key = (font.clone(), size_key);
        self.metrics_cache.get(&cache_key)
    }

    /// Clear metrics cache
    pub fn clear_cache(&mut self) {
        self.metrics_cache.clear();
    }
}

/// Calculated text position for rendering
#[derive(Debug, Clone)]
pub struct TextPosition {
    /// X coordinate of text baseline in CSS pixels
    pub baseline_x: f64,
    /// Y coordinate of text baseline in CSS pixels
    pub baseline_y: f64,
    /// Font metrics used for this positioning
    pub font_metrics: FontMetrics,
}

impl TextPosition {
    /// Get CSS style string for absolute positioning
    pub fn to_css_style(&self) -> String {
        format!(
            "position: absolute; left: {}px; top: {}px; font-family: {}; font-weight: {}; font-size: {}px; line-height: {}px;",
            self.baseline_x,
            self.baseline_y - self.font_metrics.baseline_offset_px,
            self.font_metrics.font_family.css_family(),
            self.font_metrics.font_family.css_weight(),
            self.font_metrics.font_size_px,
            self.font_metrics.line_height_px
        )
    }

    /// Calculate text box dimensions
    pub fn calculate_text_bounds(&self, text: &str) -> TextBounds {
        let text_width = if let Some(char_width) = self.font_metrics.char_width_px {
            // Monospace font - use character width
            text.len() as f64 * char_width
        } else {
            // Proportional font - estimate based on font size
            text.len() as f64 * self.font_metrics.font_size_px * 0.6
        };

        TextBounds {
            x: self.baseline_x,
            y: self.baseline_y - self.font_metrics.ascent_px,
            width: text_width,
            height: self.font_metrics.line_height_px,
        }
    }
}

/// Text bounding box dimensions
#[derive(Debug, Clone)]
pub struct TextBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_font_css() {
        assert_eq!(StandardFont::Helvetica.css_family(), "Helvetica, Arial, sans-serif");
        assert_eq!(StandardFont::Times.css_family(), "Times, 'Times New Roman', serif");
        assert_eq!(StandardFont::Courier.css_family(), "'Courier New', Courier, monospace");
        
        assert_eq!(StandardFont::Helvetica.css_weight(), "normal");
        assert_eq!(StandardFont::HelveticaBold.css_weight(), "bold");
    }

    #[test]
    fn test_font_metrics_creation() {
        let metrics = FontMetrics {
            font_size_pt: 12.0,
            font_size_px: 16.0,
            ascent_px: 12.0,
            descent_px: 4.0,
            line_height_px: 16.0,
            char_width_px: Some(9.6),
            baseline_offset_px: 12.0,
            font_family: StandardFont::Courier,
            timestamp: 1234567890,
        };

        assert_eq!(metrics.font_size_pt, 12.0);
        assert_eq!(metrics.line_height_px, 16.0);
        assert!(metrics.char_width_px.is_some());
    }
}