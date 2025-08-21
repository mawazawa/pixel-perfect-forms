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

/// Rendering engine that manages precise placement of form fields
pub struct RenderingEngine {
    calibration_manager: CalibrationManager,
}

impl RenderingEngine {
    /// Create a new rendering engine
    pub fn new() -> Self {
        Self {
            calibration_manager: CalibrationManager::new(),
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
}
