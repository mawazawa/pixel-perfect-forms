//
// ██████╗ ███████╗███╗   ██╗███████╗███╗   ██╗████████╗ ██████╗ ███████╗███╗   ██╗██╗██████╗ ██╗   ██╗██████╗  █████╗ ███╗   ██╗ █████╗ ██╗     
// ██╔══██╗██╔════╝████╗  ██║██╔════╝████╗  ██║╚══██╔══╝██╔════╝ ██╔════╝████╗  ██║██║██╔══██╗██║   ██║██╔══██╗██╔══██╗████╗  ██║██╔══██╗██║     
// ██████╔╝█████╗  ██╔██╗ ██║█████╗  ██╔██╗ ██║   ██║   ██║  ███╗█████╗  ██╔██╗ ██║██║██████╔╝██║   ██║██████╔╝███████║██╔██╗ ██║███████║██║     
// ██╔══██╗██╔══╝  ██║╚██╗██║██╔══╝  ██║╚██╗██║   ██║   ██║   ██║██╔══╝  ██║╚██╗██║██║██╔══██╗██║   ██║██╔═══╝ ██╔══██║██║╚██╗██║██╔══██║██║     
// ██║  ██║███████╗██║ ╚████║███████╗██║ ╚████║   ██║   ╚██████╔╝███████╗██║ ╚████║██║██║  ██║╚██████╔╝██║     ██║  ██║██║ ╚████║██║  ██║███████╗
// ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═══╝   ╚═╝    ╚═════╝ ╚══════╝╚═╝  ╚═══╝╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝
//                                                            app/src/rendering_engine.rs

use crate::coordinates::{PhysicalCoord, ScreenCoord};
use crate::calibration::CalibrationManager;
use crate::font_metrics::{FontMetricsCalculator, StandardFont, TextPosition};
use std::rc::Rc;
use std::cell::RefCell;

/// Rendering engine that manages precise placement of form fields
pub struct RenderingEngine {
    calibration_manager: CalibrationManager,
    font_calculator: Rc<RefCell<FontMetricsCalculator>>,
}

impl RenderingEngine {
    /// Create a new rendering engine
    pub fn new() -> Self {
        Self {
            calibration_manager: CalibrationManager::new(),
            font_calculator: Rc::new(RefCell::new(FontMetricsCalculator::new())),
        }
    }

    /// Convert physical coordinates to screen coordinates using calibration data
    pub fn physical_to_screen(&self, physical_coord: PhysicalCoord) -> Option<ScreenCoord> {
        if let Some(coord_system) = self.calibration_manager.get_coordinate_system() {
            Some(coord_system.physical_to_screen(physical_coord))
        } else {
            None
        }
    }

    /// Get the current calibration scale factor
    pub fn get_scale_factor(&self) -> Option<f64> {
        if let Some(coord_system) = self.calibration_manager.get_coordinate_system() {
            Some(coord_system.get_calibration().scale_factor)
        } else {
            None
        }
    }

    /// Check if the device is calibrated
    pub fn is_calibrated(&self) -> bool {
        self.calibration_manager.get_coordinate_system().is_some()
    }

    /// Calculate precise text positioning for a form field
    pub fn calculate_text_position(
        &self,
        font: StandardFont,
        font_size_pt: f64,
        field_position: PhysicalCoord,
        field_height_mm: f64,
    ) -> Option<TextPosition> {
        if let Some(coord_system) = self.calibration_manager.get_coordinate_system() {
            if let Ok(mut calculator) = self.font_calculator.try_borrow_mut() {
                calculator.calculate_text_position(
                    font,
                    font_size_pt,
                    field_position,
                    field_height_mm,
                    &coord_system,
                )
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get font metrics for specific font and size
    pub fn get_font_metrics(&self, font: StandardFont, font_size_pt: f64) -> Option<crate::font_metrics::FontMetrics> {
        if let Ok(mut calculator) = self.font_calculator.try_borrow_mut() {
            calculator.measure_font(font, font_size_pt)
        } else {
            None
        }
    }

    /// Get shared reference to font calculator for advanced usage
    pub fn get_font_calculator(&self) -> Rc<RefCell<FontMetricsCalculator>> {
        self.font_calculator.clone()
    }

    /// Validate that text will fit within specified field bounds
    pub fn validate_text_fit(
        &self,
        text: &str,
        font: StandardFont,
        font_size_pt: f64,
        field_width_mm: f64,
        field_height_mm: f64,
    ) -> bool {
        if let Some(coord_system) = self.calibration_manager.get_coordinate_system() {
            if let Ok(mut calculator) = self.font_calculator.try_borrow_mut() {
                if let Some(metrics) = calculator.measure_font(font, font_size_pt) {
                    // Convert field dimensions to pixels
                    let field_width_px = coord_system.mm_to_px(field_width_mm);
                    let field_height_px = coord_system.mm_to_px(field_height_mm);
                    
                    // Estimate text width
                    let text_width = if let Some(char_width) = metrics.char_width_px {
                        text.len() as f64 * char_width
                    } else {
                        text.len() as f64 * metrics.font_size_px * 0.6 // Approximation for proportional fonts
                    };
                    
                    // Check if text fits within bounds
                    text_width <= field_width_px && metrics.line_height_px <= field_height_px
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}
