//
//  ██████╗ █████╗ ██╗     ██╗██████╗ ██████╗  █████╗ ████████╗██╗ ██████╗ ███╗   ██╗
// ██╔════╝██╔══██╗██║     ██║██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝██║██╔═══██╗████╗  ██║
// ██║     ███████║██║     ██║██████╔╝██████╔╝███████║   ██║   ██║██║   ██║██╔██╗ ██║
// ██║     ██╔══██║██║     ██║██╔══██╗██╔══██╗██╔══██║   ██║   ██║██║   ██║██║╚██╗██║
// ╚██████╗██║  ██║███████╗██║██████╔╝██║  ██║██║  ██║   ██║   ██║╚██████╔╝██║ ╚████║
//  ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══╝
//                                               app/src/calibration.rs

use crate::coordinates::{DeviceCalibration, CoordinateSystem};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{window, Window};
use yew::prelude::*;

const CALIBRATION_STORAGE_KEY: &str = "fl100_device_calibration";
const TARGET_RULER_LENGTH_MM: f64 = 100.0; // 10cm ruler for calibration

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationState {
    pub step: CalibrationStep,
    pub measured_pixels: Option<f64>,
    pub estimated_scale: Option<f64>,
    pub confidence_score: f64,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CalibrationStep {
    Welcome,
    Instructions,
    Measuring,
    Validation,
    Complete,
}

impl Default for CalibrationState {
    fn default() -> Self {
        Self {
            step: CalibrationStep::Welcome,
            measured_pixels: None,
            estimated_scale: None,
            confidence_score: 0.0,
            error_message: None,
        }
    }
}

#[derive(Clone)]
pub struct CalibrationManager {
    pub state: CalibrationState,
    coordinate_system: Option<CoordinateSystem>,
}

impl PartialEq for CalibrationManager {
    fn eq(&self, other: &Self) -> bool {
        // For simplicity, we'll consider managers equal if they have the same calibration state
        self.state.step == other.state.step && 
        self.state.measured_pixels == other.state.measured_pixels
    }
}

impl CalibrationManager {
    pub fn new() -> Self {
        let loaded_calibration = Self::load_calibration();
        let coordinate_system = loaded_calibration
            .map(|cal| CoordinateSystem::new(cal));

        Self {
            state: CalibrationState::default(),
            coordinate_system,
        }
    }

    /// Load calibration from localStorage
    pub fn load_calibration() -> Option<DeviceCalibration> {
        LocalStorage::get(CALIBRATION_STORAGE_KEY).ok()
    }

    /// Save calibration to localStorage
    pub fn save_calibration(calibration: &DeviceCalibration) -> Result<(), String> {
        LocalStorage::set(CALIBRATION_STORAGE_KEY, calibration)
            .map_err(|e| format!("Failed to save calibration: {:?}", e))
    }

    /// Start calibration process
    pub fn start_calibration(&mut self) {
        self.state = CalibrationState {
            step: CalibrationStep::Instructions,
            ..Default::default()
        };
    }

    /// Process user measurement input
    pub fn process_measurement(&mut self, measured_pixels: f64) -> Result<(), String> {
        if measured_pixels <= 0.0 {
            return Err("Measurement must be positive".to_string());
        }

        if measured_pixels > 2000.0 {
            return Err("Measurement seems too large - please check your ruler".to_string());
        }

        let scale_factor = measured_pixels / TARGET_RULER_LENGTH_MM;
        let confidence = self.calculate_confidence(scale_factor);

        self.state.measured_pixels = Some(measured_pixels);
        self.state.estimated_scale = Some(scale_factor);
        self.state.confidence_score = confidence;
        self.state.step = CalibrationStep::Validation;

        Ok(())
    }

    /// Calculate confidence score based on scale factor reasonableness
    fn calculate_confidence(&self, scale_factor: f64) -> f64 {
        // Typical range: 2-6 pixels per mm (50-150 DPI)
        let min_reasonable = 2.0;
        let max_reasonable = 6.0;
        let optimal = 3.78; // ~96 DPI

        if scale_factor < min_reasonable || scale_factor > max_reasonable {
            0.3 // Low confidence for out-of-range values
        } else {
            let distance_from_optimal = (scale_factor - optimal).abs();
            let max_distance = (max_reasonable - optimal).max(optimal - min_reasonable);
            (1.0 - (distance_from_optimal / max_distance)).max(0.4)
        }
    }

    /// Complete calibration and save
    pub fn complete_calibration(&mut self) -> Result<DeviceCalibration, String> {
        let scale_factor = self.state.estimated_scale
            .ok_or("No measurement available")?;

        let window = window().ok_or("No window object")?;
        let device_pixel_ratio = window.device_pixel_ratio();
        let inner_width = window.inner_width()
            .map_err(|_| "Failed to get window width")?
            .as_f64()
            .ok_or("Invalid window width")?;
        let inner_height = window.inner_height()
            .map_err(|_| "Failed to get window height")?
            .as_f64()
            .ok_or("Invalid window height")?;

        let calibration = DeviceCalibration {
            scale_factor,
            confidence: self.state.confidence_score,
            timestamp: js_sys::Date::now() as u64,
            device_pixel_ratio,
            viewport_width: inner_width,
            viewport_height: inner_height,
        };

        Self::save_calibration(&calibration)?;
        self.coordinate_system = Some(CoordinateSystem::new(calibration.clone()));
        self.state.step = CalibrationStep::Complete;

        Ok(calibration)
    }

    /// Get current calibration state
    pub fn get_state(&self) -> &CalibrationState {
        &self.state
    }

    /// Get coordinate system (if calibrated)
    pub fn get_coordinate_system(&self) -> Option<&CoordinateSystem> {
        self.coordinate_system.as_ref()
    }

    /// Get calibration data (if calibrated)
    pub fn get_calibration(&self) -> Option<&DeviceCalibration> {
        self.coordinate_system.as_ref().map(|cs| cs.get_calibration())
    }

    /// Check if recalibration is needed
    pub fn needs_recalibration(&self) -> bool {
        match Self::load_calibration() {
            Some(cal) => {
                // Recalibrate if confidence is low or viewport changed significantly
                cal.confidence < 0.6 || self.viewport_changed_significantly(&cal)
            }
            None => true,
        }
    }

    /// Check if viewport changed significantly since last calibration
    fn viewport_changed_significantly(&self, last_cal: &DeviceCalibration) -> bool {
        if let Some(window) = window() {
            if let (Ok(current_width), Ok(current_height)) = 
                (window.inner_width(), window.inner_height()) {
                if let (Some(width), Some(height)) = 
                    (current_width.as_f64(), current_height.as_f64()) {
                    let width_change = (width - last_cal.viewport_width).abs() / last_cal.viewport_width;
                    let height_change = (height - last_cal.viewport_height).abs() / last_cal.viewport_height;
                    return width_change > 0.1 || height_change > 0.1; // 10% change threshold
                }
            }
        }
        false
    }

    /// Reset calibration (force new calibration)
    pub fn reset_calibration(&mut self) {
        let _ = LocalStorage::delete(CALIBRATION_STORAGE_KEY);
        self.coordinate_system = None;
        self.state = CalibrationState::default();
    }

    /// Get estimated DPI for display
    pub fn get_estimated_dpi(&self) -> Option<f64> {
        self.state.estimated_scale.map(|scale| scale * 25.4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_confidence() {
        let mut manager = CalibrationManager::new();
        
        // Good measurement (96 DPI = ~3.78 px/mm)
        manager.process_measurement(378.0).unwrap();
        assert!(manager.state.confidence_score > 0.8);

        // Poor measurement (too high)
        let mut manager2 = CalibrationManager::new();
        manager2.process_measurement(1000.0).unwrap();
        assert!(manager2.state.confidence_score < 0.5);
    }

    #[test]
    fn test_measurement_validation() {
        let mut manager = CalibrationManager::new();
        
        // Invalid measurements
        assert!(manager.process_measurement(0.0).is_err());
        assert!(manager.process_measurement(-10.0).is_err());
        assert!(manager.process_measurement(3000.0).is_err());
        
        // Valid measurement
        assert!(manager.process_measurement(350.0).is_ok());
    }
}