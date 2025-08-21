//
// ██████╗ ██████╗  ██████╗ ██████╗ ██████╗ ██╗███╗   ██╗ █████╗ ████████╗███████╗███████╗
// ██╔════╝██╔═══██╗██╔═══██╗██╔══██╗██╔══██╗██║████╗  ██║██╔══██╗╚══██╔══╝██╔════╝██╔════╝
// ██║     ██║   ██║██║   ██║██████╔╝██║  ██║██║██╔██╗ ██║███████║   ██║   █████╗  ███████╗
// ██║     ██║   ██║██║   ██║██╔══██╗██║  ██║██║██║╚██╗██║██╔══██║   ██║   ██╔══╝  ╚════██║
// ╚██████╗╚██████╔╝╚██████╔╝██║  ██║██████╔╝██║██║ ╚████║██║  ██║   ██║   ███████╗███████║
//  ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═════╝ ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝   ╚═╝   ╚══════╝╚══════╝
//                                               app/src/coordinates.rs

use serde::{Deserialize, Serialize};

/// US Letter page dimensions in millimeters
pub const US_LETTER_WIDTH_MM: f64 = 215.9;
pub const US_LETTER_HEIGHT_MM: f64 = 279.4;


/// Physical coordinate in millimeters
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicalCoord {
    pub x: f64, // mm from left edge
    pub y: f64, // mm from top edge
}

/// Screen coordinate in CSS pixels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ScreenCoord {
    pub x: f64, // px from left edge
    pub y: f64, // px from top edge
}

/// Device calibration data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCalibration {
    pub scale_factor: f64,     // pixels per mm
    pub confidence: f64,       // 0.0 to 1.0
    pub timestamp: u64,        // Unix timestamp
    pub device_pixel_ratio: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
}

impl Default for DeviceCalibration {
    fn default() -> Self {
        Self {
            scale_factor: 3.779527559, // 96 DPI assumption
            confidence: 0.5,
            timestamp: 0,
            device_pixel_ratio: 1.0,
            viewport_width: 1920.0,
            viewport_height: 1080.0,
        }
    }
}

/// Coordinate transformation utilities
#[derive(Clone)]
pub struct CoordinateSystem {
    calibration: DeviceCalibration,
}

impl CoordinateSystem {
    pub fn new(calibration: DeviceCalibration) -> Self {
        Self { calibration }
    }

    /// Get the calibration data
    pub fn get_calibration(&self) -> &DeviceCalibration {
        &self.calibration
    }

    /// Convert millimeters to CSS pixels
    pub fn mm_to_px(&self, mm: f64) -> f64 {
        mm * self.calibration.scale_factor
    }

    /// Convert CSS pixels to millimeters
    pub fn px_to_mm(&self, px: f64) -> f64 {
        px / self.calibration.scale_factor
    }

    /// Convert physical coordinate to screen coordinate
    pub fn physical_to_screen(&self, coord: PhysicalCoord) -> ScreenCoord {
        ScreenCoord {
            x: self.mm_to_px(coord.x),
            y: self.mm_to_px(coord.y),
        }
    }


    /// Validate coordinate is within US Letter bounds
    pub fn validate_physical_bounds(&self, coord: PhysicalCoord) -> bool {
        coord.x >= 0.0
            && coord.x <= US_LETTER_WIDTH_MM
            && coord.y >= 0.0
            && coord.y <= US_LETTER_HEIGHT_MM
    }

    /// Snap coordinate to grid with tolerance
    pub fn snap_to_grid(&self, coord: PhysicalCoord, grid_size_mm: f64, tolerance_mm: f64) -> PhysicalCoord {
        let snap_x = (coord.x / grid_size_mm).round() * grid_size_mm;
        let snap_y = (coord.y / grid_size_mm).round() * grid_size_mm;

        if (coord.x - snap_x).abs() <= tolerance_mm && (coord.y - snap_y).abs() <= tolerance_mm {
            PhysicalCoord { x: snap_x, y: snap_y }
        } else {
            coord
        }
    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_conversions() {
        let calibration = DeviceCalibration::default();
        let coord_sys = CoordinateSystem::new(calibration);

        // Test round-trip conversion
        let original_mm = 25.4; // 1 inch
        let px = coord_sys.mm_to_px(original_mm);
        let converted_mm = coord_sys.px_to_mm(px);
        
        assert!((original_mm - converted_mm).abs() < 0.001);
    }

    #[test]
    fn test_bounds_validation() {
        let calibration = DeviceCalibration::default();
        let coord_sys = CoordinateSystem::new(calibration);

        assert!(coord_sys.validate_physical_bounds(PhysicalCoord { x: 100.0, y: 100.0 }));
        assert!(!coord_sys.validate_physical_bounds(PhysicalCoord { x: 300.0, y: 100.0 }));
        assert!(!coord_sys.validate_physical_bounds(PhysicalCoord { x: -10.0, y: 100.0 }));
    }

    #[test]
    fn test_grid_snapping() {
        let calibration = DeviceCalibration::default();
        let coord_sys = CoordinateSystem::new(calibration);

        let coord = PhysicalCoord { x: 10.1, y: 20.2 };
        let snapped = coord_sys.snap_to_grid(coord, 5.0, 0.5);
        
        assert_eq!(snapped, PhysicalCoord { x: 10.0, y: 20.0 });
    }
}