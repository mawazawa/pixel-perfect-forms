//
// ██████╗██╗   ██╗     █████╗ ██╗     ██╗ ██████╗ ███╗   ██╗███╗   ███╗███████╗███╗   ██╗████████╗
//██╔════╝██║   ██║    ██╔══██╗██║     ██║██╔════╝ ████╗ ████║████╗ ████║██╔════╝████╗  ██║╚══██╔══╝
//██║     ██║   ██║    ███████║██║     ██║██║  ███╗██╔████╔██║██╔████╔██║█████╗  ██╔██╗ ██║   ██║   
//██║     ╚██╗ ██╔╝    ██╔══██║██║     ██║██║   ██║██║╚██╔╝██║██║╚██╔╝██║██╔══╝  ██║╚██╗██║   ██║   
//╚██████╗ ╚████╔╝     ██║  ██║███████╗██║╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║███████╗██║ ╚████║   ██║   
// ╚═════╝  ╚═══╝      ╚═╝  ╚═╝╚══════╝╚═╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚══════╝╚═╝  ╚═══╝   ╚═╝   
//                                                  app/src/cv_alignment.rs

use crate::coordinates::{PhysicalCoord, CoordinateSystem};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Worker, MessageEvent, ImageData};
use yew::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Computer vision alignment correction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentCorrection {
    /// X-axis correction in pixels
    pub x: f64,
    /// Y-axis correction in pixels  
    pub y: f64,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Whether correction was applied
    pub applied: bool,
}

/// Field boundary detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldBoundary {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub area: f64,
    pub aspect_ratio: f64,
}

/// SSIM quality assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessment {
    /// SSIM score (0.0 to 1.0)
    pub ssim: f64,
    /// Quality category
    pub quality: String,
}

/// CV worker task types
#[derive(Debug, Clone)]
pub enum CVTask {
    TemplateMatch {
        field_id: String,
        image_data: ImageData,
        template_data: ImageData,
        threshold: f64,
    },
    CalculateSSIM {
        image_data1: ImageData,
        image_data2: ImageData,
    },
    DetectFields {
        image_data: ImageData,
        threshold1: f64,
        threshold2: f64,
    },
}

/// CV worker result
#[derive(Debug)]
pub enum CVResult {
    TemplateMatch {
        field_id: String,
        correction: AlignmentCorrection,
    },
    SSIM {
        assessment: QualityAssessment,
    },
    FieldDetection {
        fields: Vec<FieldBoundary>,
    },
    Error {
        message: String,
    },
}

/// Computer vision alignment system for automatic form field correction
pub struct CVAlignmentSystem {
    worker: Option<Worker>,
    pending_tasks: Rc<RefCell<HashMap<u32, Callback<CVResult>>>>,
    task_counter: Rc<RefCell<u32>>,
    coordinate_system: Option<CoordinateSystem>,
}

impl CVAlignmentSystem {
    /// Create new CV alignment system
    pub fn new() -> Self {
        Self {
            worker: Self::create_worker(),
            pending_tasks: Rc::new(RefCell::new(HashMap::new())),
            task_counter: Rc::new(RefCell::new(0)),
            coordinate_system: None,
        }
    }

    /// Create and initialize the CV worker
    fn create_worker() -> Option<Worker> {
        let window = web_sys::window()?;
        let worker = Worker::new("/static/workers/cv-worker.js").ok()?;
        
        // Send ping to verify worker is ready
        let init_msg = js_sys::Object::new();
        js_sys::Reflect::set(&init_msg, &"type".into(), &"ping".into()).ok()?;
        js_sys::Reflect::set(&init_msg, &"id".into(), &0.into()).ok()?;
        worker.post_message(&init_msg).ok()?;
        
        Some(worker)
    }

    /// Set coordinate system for physical-to-screen conversions
    pub fn set_coordinate_system(&mut self, coord_system: CoordinateSystem) {
        self.coordinate_system = Some(coord_system);
    }

    /// Process template matching for automatic alignment
    pub fn process_template_matching(
        &mut self,
        field_id: String,
        image_data: ImageData,
        template_data: ImageData,
        threshold: f64,
        callback: Callback<CVResult>,
    ) -> Result<(), JsValue> {
        let worker = self.worker.as_ref().ok_or("Worker not initialized")?;
        
        let task_id = {
            let mut counter = self.task_counter.borrow_mut();
            *counter += 1;
            *counter
        };

        // Store callback for this task
        self.pending_tasks.borrow_mut().insert(task_id, callback);

        // Prepare message for worker
        let msg = js_sys::Object::new();
        js_sys::Reflect::set(&msg, &"type".into(), &"template-match".into())?;
        js_sys::Reflect::set(&msg, &"id".into(), &task_id.into())?;
        
        let data = js_sys::Object::new();
        js_sys::Reflect::set(&data, &"fieldId".into(), &field_id.into())?;
        js_sys::Reflect::set(&data, &"imageData".into(), &image_data)?;
        js_sys::Reflect::set(&data, &"templateData".into(), &template_data)?;
        js_sys::Reflect::set(&data, &"threshold".into(), &threshold.into())?;
        
        js_sys::Reflect::set(&msg, &"data".into(), &data)?;

        worker.post_message(&msg)?;
        Ok(())
    }

    /// Calculate SSIM quality assessment between two images
    pub fn calculate_ssim(
        &mut self,
        image_data1: ImageData,
        image_data2: ImageData,
        callback: Callback<CVResult>,
    ) -> Result<(), JsValue> {
        let worker = self.worker.as_ref().ok_or("Worker not initialized")?;
        
        let task_id = {
            let mut counter = self.task_counter.borrow_mut();
            *counter += 1;
            *counter
        };

        self.pending_tasks.borrow_mut().insert(task_id, callback);

        let msg = js_sys::Object::new();
        js_sys::Reflect::set(&msg, &"type".into(), &"calculate-ssim".into())?;
        js_sys::Reflect::set(&msg, &"id".into(), &task_id.into())?;
        
        let data = js_sys::Object::new();
        js_sys::Reflect::set(&data, &"imageData1".into(), &image_data1)?;
        js_sys::Reflect::set(&data, &"imageData2".into(), &image_data2)?;
        
        js_sys::Reflect::set(&msg, &"data".into(), &data)?;

        worker.post_message(&msg)?;
        Ok(())
    }

    /// Detect form field boundaries using edge detection
    pub fn detect_field_boundaries(
        &mut self,
        image_data: ImageData,
        threshold1: f64,
        threshold2: f64,
        callback: Callback<CVResult>,
    ) -> Result<(), JsValue> {
        let worker = self.worker.as_ref().ok_or("Worker not initialized")?;
        
        let task_id = {
            let mut counter = self.task_counter.borrow_mut();
            *counter += 1;
            *counter
        };

        self.pending_tasks.borrow_mut().insert(task_id, callback);

        let msg = js_sys::Object::new();
        js_sys::Reflect::set(&msg, &"type".into(), &"detect-fields".into())?;
        js_sys::Reflect::set(&msg, &"id".into(), &task_id.into())?;
        
        let data = js_sys::Object::new();
        js_sys::Reflect::set(&data, &"imageData".into(), &image_data)?;
        js_sys::Reflect::set(&data, &"threshold1".into(), &threshold1.into())?;
        js_sys::Reflect::set(&data, &"threshold2".into(), &threshold2.into())?;
        
        js_sys::Reflect::set(&msg, &"data".into(), &data)?;

        worker.post_message(&msg)?;
        Ok(())
    }

    /// Apply alignment correction to physical coordinates
    pub fn apply_alignment_correction(
        &self,
        original_position: PhysicalCoord,
        correction: &AlignmentCorrection,
    ) -> Option<PhysicalCoord> {
        if !correction.applied {
            return Some(original_position);
        }

        let coord_system = self.coordinate_system.as_ref()?;

        // Convert correction from pixels to millimeters
        let correction_x_mm = correction.x / coord_system.get_calibration().scale_factor;
        let correction_y_mm = correction.y / coord_system.get_calibration().scale_factor;

        Some(PhysicalCoord {
            x: original_position.x + correction_x_mm,
            y: original_position.y + correction_y_mm,
        })
    }

    /// Setup worker message handler
    pub fn setup_worker_handler(&mut self) {
        if let Some(worker) = &self.worker {
            let pending_tasks = self.pending_tasks.clone();
            
            let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
                let data = event.data();
                
                // Parse worker response
                if let Ok(msg_type) = js_sys::Reflect::get(&data, &"type".into()) {
                    let msg_type_str = msg_type.as_string().unwrap_or_default();
                    
                    match msg_type_str.as_str() {
                        "result" => {
                            if let (Ok(id), Ok(result_data)) = (
                                js_sys::Reflect::get(&data, &"id".into()),
                                js_sys::Reflect::get(&data, &"data".into())
                            ) {
                                if let Some(task_id) = id.as_f64() {
                                    let task_id = task_id as u32;
                                    
                                    if let Some(callback) = pending_tasks.borrow_mut().remove(&task_id) {
                                        // Parse result based on original task type
                                        if let Some(result) = Self::parse_worker_result(&result_data) {
                                            callback.emit(result);
                                        }
                                    }
                                }
                            }
                        }
                        "error" => {
                            if let (Ok(id), Ok(error)) = (
                                js_sys::Reflect::get(&data, &"id".into()),
                                js_sys::Reflect::get(&data, &"error".into())
                            ) {
                                if let Some(task_id) = id.as_f64() {
                                    let task_id = task_id as u32;
                                    
                                    if let Some(callback) = pending_tasks.borrow_mut().remove(&task_id) {
                                        let error_msg = error.as_string().unwrap_or("Unknown error".to_string());
                                        callback.emit(CVResult::Error { message: error_msg });
                                    }
                                }
                            }
                        }
                        "ready" => {
                            web_sys::console::log_1(&"CV Worker ready".into());
                        }
                        _ => {}
                    }
                }
            }) as Box<dyn FnMut(MessageEvent)>);

            worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            onmessage.forget();
        }
    }

    /// Parse worker result data into CVResult enum
    fn parse_worker_result(data: &JsValue) -> Option<CVResult> {
        // Check if this is a template matching result
        if let Ok(field_id) = js_sys::Reflect::get(data, &"fieldId".into()) {
            if let Some(field_id_str) = field_id.as_string() {
                if let Ok(correction_data) = js_sys::Reflect::get(data, &"correction".into()) {
                    let correction = AlignmentCorrection {
                        x: js_sys::Reflect::get(&correction_data, &"x".into())
                            .ok()?.as_f64().unwrap_or(0.0),
                        y: js_sys::Reflect::get(&correction_data, &"y".into())
                            .ok()?.as_f64().unwrap_or(0.0),
                        confidence: js_sys::Reflect::get(&correction_data, &"confidence".into())
                            .ok()?.as_f64().unwrap_or(0.0),
                        applied: js_sys::Reflect::get(&correction_data, &"applied".into())
                            .ok()?.as_bool().unwrap_or(false),
                    };
                    
                    return Some(CVResult::TemplateMatch {
                        field_id: field_id_str,
                        correction,
                    });
                }
            }
        }

        // Check if this is an SSIM result
        if let Ok(ssim) = js_sys::Reflect::get(data, &"ssim".into()) {
            if let Some(ssim_val) = ssim.as_f64() {
                let quality = js_sys::Reflect::get(data, &"quality".into())
                    .ok()?.as_string().unwrap_or("unknown".to_string());
                
                return Some(CVResult::SSIM {
                    assessment: QualityAssessment {
                        ssim: ssim_val,
                        quality,
                    },
                });
            }
        }

        // Check if this is field detection result
        if let Ok(fields_array) = js_sys::Reflect::get(data, &"fields".into()) {
            if let Ok(fields) = fields_array.dyn_into::<js_sys::Array>() {
                let mut field_boundaries = Vec::new();
                
                for i in 0..fields.length() {
                    if let Ok(field) = fields.get(i).dyn_into::<js_sys::Object>() {
                        if let (Ok(x), Ok(y), Ok(width), Ok(height), Ok(area), Ok(aspect_ratio)) = (
                            js_sys::Reflect::get(&field, &"x".into()).and_then(|v| Ok(v.as_f64().unwrap_or(0.0))),
                            js_sys::Reflect::get(&field, &"y".into()).and_then(|v| Ok(v.as_f64().unwrap_or(0.0))),
                            js_sys::Reflect::get(&field, &"width".into()).and_then(|v| Ok(v.as_f64().unwrap_or(0.0))),
                            js_sys::Reflect::get(&field, &"height".into()).and_then(|v| Ok(v.as_f64().unwrap_or(0.0))),
                            js_sys::Reflect::get(&field, &"area".into()).and_then(|v| Ok(v.as_f64().unwrap_or(0.0))),
                            js_sys::Reflect::get(&field, &"aspectRatio".into()).and_then(|v| Ok(v.as_f64().unwrap_or(0.0))),
                        ) {
                            field_boundaries.push(FieldBoundary {
                                x, y, width, height, area, aspect_ratio
                            });
                        }
                    }
                }
                
                return Some(CVResult::FieldDetection {
                    fields: field_boundaries,
                });
            }
        }

        None
    }

    /// Check if worker is available
    pub fn is_available(&self) -> bool {
        self.worker.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_correction() {
        let correction = AlignmentCorrection {
            x: 5.0,
            y: -3.0,
            confidence: 0.85,
            applied: true,
        };

        assert_eq!(correction.x, 5.0);
        assert_eq!(correction.y, -3.0);
        assert_eq!(correction.confidence, 0.85);
        assert!(correction.applied);
    }

    #[test]
    fn test_field_boundary() {
        let boundary = FieldBoundary {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 30.0,
            area: 3000.0,
            aspect_ratio: 3.33,
        };

        assert_eq!(boundary.width / boundary.height, 100.0 / 30.0);
        assert!(boundary.aspect_ratio > 3.0);
    }
}