//
// ██████╗ ██╗   ██╗███████╗██████╗ ██╗      █████╗ ██╗   ██╗    ███╗   ███╗ █████╗ ███╗   ██╗ █████╗  ██████╗ ███████╗██████╗ 
// ██╔═══██╗██║   ██║██╔════╝██╔══██╗██║     ██╔══██╗╚██╗ ██╔╝    ████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝ ██╔════╝██╔══██╗
// ██║   ██║██║   ██║█████╗  ██████╔╝██║     ███████║ ╚████╔╝     ██╔████╔██║███████║██╔██╗ ██║███████║██║  ███╗█████╗  ██████╔╝
// ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗██║     ██╔══██║  ╚██╔╝      ██║╚██╔╝██║██╔══██║██║╚██╗██║██╔══██║██║   ██║██╔══╝  ██╔══██╗
// ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║███████╗██║  ██║   ██║       ██║ ╚═╝ ██║██║  ██║██║ ╚████║██║  ██║╚██████╔╝███████╗██║  ██║
//  ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝   ╚═╝       ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝
//                                                            app/src/overlay_manager.rs

use crate::coordinates::{PhysicalCoord, ScreenCoord};
use crate::calibration::CalibrationManager;
use yew::prelude::*;
use web_sys::{MouseEvent, HtmlElement};
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq)]
pub struct Overlay {
    pub id: String,
    pub page_index: usize,
    pub overlay_type: OverlayType,
    pub transform: Transform,
    pub position: PhysicalCoord,
    pub size: PhysicalCoord,
    pub visible: bool,
    pub selected: bool,
    pub z_index: i32,
}

#[derive(Clone, PartialEq)]
pub enum OverlayType {
    TextBox,
    Image,
    Shape,
    Annotation,
    Measurement,
}

#[derive(Clone, PartialEq)]
pub struct Transform {
    pub translate_x: f64,
    pub translate_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotation: f64,
    pub skew_x: f64,
    pub skew_y: f64,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
        }
    }
}

impl Transform {
    pub fn to_css_string(&self) -> String {
        format!(
            "translate({}px, {}px) scale({}, {}) rotate({}deg) skew({}deg, {}deg)",
            self.translate_x,
            self.translate_y,
            self.scale_x,
            self.scale_y,
            self.rotation,
            self.skew_x,
            self.skew_y
        )
    }

    pub fn apply_translation(&mut self, delta_x: f64, delta_y: f64) {
        self.translate_x += delta_x;
        self.translate_y += delta_y;
    }

    pub fn apply_scaling(&mut self, scale_factor: f64, origin_x: f64, origin_y: f64) {
        // Apply scaling around a specific origin point
        let dx = origin_x - self.translate_x;
        let dy = origin_y - self.translate_y;
        
        self.scale_x *= scale_factor;
        self.scale_y *= scale_factor;
        
        // Adjust translation to maintain origin point
        self.translate_x = origin_x - dx * scale_factor;
        self.translate_y = origin_y - dy * scale_factor;
    }

    pub fn apply_rotation(&mut self, angle_deg: f64, origin_x: f64, origin_y: f64) {
        self.rotation += angle_deg;
        
        // Normalize rotation to 0-360 degrees
        while self.rotation >= 360.0 {
            self.rotation -= 360.0;
        }
        while self.rotation < 0.0 {
            self.rotation += 360.0;
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct OverlayManagerProps {
    pub calibration_manager: CalibrationManager,
    pub page_index: usize,
    pub overlays: Vec<Overlay>,
    pub on_overlay_change: Callback<(usize, Overlay)>,
    pub snap_enabled: bool,
    pub snap_tolerance: f64,
    pub grid_size_mm: f64,
}

pub struct OverlayManager {
    dragging_overlay: Option<String>,
    drag_start_pos: Option<(f64, f64)>,
    transform_mode: TransformMode,
    selected_overlay: Option<String>,
    container_ref: NodeRef,
}

#[derive(Clone, PartialEq)]
pub enum TransformMode {
    Move,
    Scale,
    Rotate,
    None,
}

pub enum OverlayManagerMsg {
    StartDrag(String, MouseEvent),
    Drag(MouseEvent),
    EndDrag,
    SelectOverlay(String),
    DeselectAll,
    SetTransformMode(TransformMode),
    DeleteOverlay(String),
    DuplicateOverlay(String),
    CreateOverlay(OverlayType, PhysicalCoord),
    UpdateTransform(String, Transform),
}

impl Component for OverlayManager {
    type Message = OverlayManagerMsg;
    type Properties = OverlayManagerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dragging_overlay: None,
            drag_start_pos: None,
            transform_mode: TransformMode::Move,
            selected_overlay: None,
            container_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            OverlayManagerMsg::StartDrag(overlay_id, event) => {
                self.dragging_overlay = Some(overlay_id.clone());
                self.selected_overlay = Some(overlay_id);
                self.drag_start_pos = Some((event.client_x() as f64, event.client_y() as f64));
                event.prevent_default();
                true
            }
            OverlayManagerMsg::Drag(event) => {
                if let (Some(overlay_id), Some((start_x, start_y))) = 
                    (&self.dragging_overlay, &self.drag_start_pos) {
                    
                    let current_x = event.client_x() as f64;
                    let current_y = event.client_y() as f64;
                    let delta_x = current_x - start_x;
                    let delta_y = current_y - start_y;
                    
                    // Find the overlay and update its transform
                    if let Some(overlay) = ctx.props().overlays.iter()
                        .find(|o| o.id == *overlay_id && o.page_index == ctx.props().page_index) {
                        
                        let mut new_overlay = overlay.clone();
                        
                        match self.transform_mode {
                            TransformMode::Move => {
                                new_overlay.transform.apply_translation(delta_x, delta_y);
                                
                                // Apply snapping if enabled
                                if ctx.props().snap_enabled {
                                    new_overlay.transform = self.apply_snapping(
                                        &new_overlay.transform,
                                        ctx.props().grid_size_mm,
                                        ctx.props().snap_tolerance,
                                        &ctx.props().calibration_manager
                                    );
                                }
                            }
                            TransformMode::Scale => {
                                let scale_factor = 1.0 + (delta_x + delta_y) / 200.0;
                                new_overlay.transform.apply_scaling(
                                    scale_factor.max(0.1).min(5.0),
                                    new_overlay.position.x,
                                    new_overlay.position.y
                                );
                            }
                            TransformMode::Rotate => {
                                let angle = (delta_x + delta_y) / 5.0;
                                new_overlay.transform.apply_rotation(
                                    angle,
                                    new_overlay.position.x,
                                    new_overlay.position.y
                                );
                            }
                            TransformMode::None => {}
                        }
                        
                        ctx.props().on_overlay_change.emit((ctx.props().page_index, new_overlay));
                    }
                    
                    self.drag_start_pos = Some((current_x, current_y));
                    true
                } else {
                    false
                }
            }
            OverlayManagerMsg::EndDrag => {
                self.dragging_overlay = None;
                self.drag_start_pos = None;
                true
            }
            OverlayManagerMsg::SelectOverlay(overlay_id) => {
                self.selected_overlay = Some(overlay_id);
                true
            }
            OverlayManagerMsg::DeselectAll => {
                self.selected_overlay = None;
                true
            }
            OverlayManagerMsg::SetTransformMode(mode) => {
                self.transform_mode = mode;
                true
            }
            OverlayManagerMsg::DeleteOverlay(overlay_id) => {
                // This would be handled by parent component
                false
            }
            OverlayManagerMsg::DuplicateOverlay(overlay_id) => {
                // This would be handled by parent component
                false
            }
            OverlayManagerMsg::CreateOverlay(overlay_type, position) => {
                // This would be handled by parent component
                false
            }
            OverlayManagerMsg::UpdateTransform(overlay_id, transform) => {
                if let Some(overlay) = ctx.props().overlays.iter()
                    .find(|o| o.id == overlay_id && o.page_index == ctx.props().page_index) {
                    
                    let mut new_overlay = overlay.clone();
                    new_overlay.transform = transform;
                    ctx.props().on_overlay_change.emit((ctx.props().page_index, new_overlay));
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let coord_system = ctx.props().calibration_manager.get_coordinate_system();
        
        html! {
            <div 
                class="overlay-manager"
                ref={self.container_ref.clone()}
                onmousemove={ctx.link().callback(OverlayManagerMsg::Drag)}
                onmouseup={ctx.link().callback(|_| OverlayManagerMsg::EndDrag)}
                onmouseleave={ctx.link().callback(|_| OverlayManagerMsg::EndDrag)}
            >
                { self.render_transform_toolbar(ctx) }
                
                <div class="overlay-container">
                    { if let Some(coord_sys) = coord_system {
                        self.render_overlays(ctx, coord_sys)
                    } else {
                        html! { <div class="overlays-disabled">{"Overlays require calibration"}</div> }
                    }}
                </div>
                
                { if let Some(selected_id) = &self.selected_overlay {
                    self.render_transform_controls(ctx, selected_id)
                } else {
                    html! {}
                }}
            </div>
        }
    }
}

impl OverlayManager {
    fn render_transform_toolbar(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="transform-toolbar">
                <div class="toolbar-group">
                    <h4>{"Transform Mode"}</h4>
                    <div class="mode-buttons">
                        <button 
                            class={if self.transform_mode == TransformMode::Move { "mode-button active" } else { "mode-button" }}
                            onclick={ctx.link().callback(|_| OverlayManagerMsg::SetTransformMode(TransformMode::Move))}
                            title="Move"
                        >
                            {"↔️"}
                        </button>
                        <button 
                            class={if self.transform_mode == TransformMode::Scale { "mode-button active" } else { "mode-button" }}
                            onclick={ctx.link().callback(|_| OverlayManagerMsg::SetTransformMode(TransformMode::Scale))}
                            title="Scale"
                        >
                            {"🔍"}
                        </button>
                        <button 
                            class={if self.transform_mode == TransformMode::Rotate { "mode-button active" } else { "mode-button" }}
                            onclick={ctx.link().callback(|_| OverlayManagerMsg::SetTransformMode(TransformMode::Rotate))}
                            title="Rotate"
                        >
                            {"🔄"}
                        </button>
                    </div>
                </div>
                
                <div class="toolbar-group">
                    <h4>{"Create Overlay"}</h4>
                    <div class="create-buttons">
                        <button 
                            class="create-button"
                            onclick={ctx.link().callback(|_| {
                                OverlayManagerMsg::CreateOverlay(
                                    OverlayType::TextBox, 
                                    PhysicalCoord { x: 50.0, y: 50.0 }
                                )
                            })}
                            title="Text Box"
                        >
                            {"📝"}
                        </button>
                        <button 
                            class="create-button"
                            onclick={ctx.link().callback(|_| {
                                OverlayManagerMsg::CreateOverlay(
                                    OverlayType::Shape, 
                                    PhysicalCoord { x: 50.0, y: 50.0 }
                                )
                            })}
                            title="Shape"
                        >
                            {"⬜"}
                        </button>
                        <button 
                            class="create-button"
                            onclick={ctx.link().callback(|_| {
                                OverlayManagerMsg::CreateOverlay(
                                    OverlayType::Annotation, 
                                    PhysicalCoord { x: 50.0, y: 50.0 }
                                )
                            })}
                            title="Annotation"
                        >
                            {"💬"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }

    fn render_overlays(&self, ctx: &Context<Self>, coord_sys: &crate::coordinates::CoordinateSystem) -> Html {
        let page_overlays: Vec<_> = ctx.props().overlays.iter()
            .filter(|overlay| overlay.page_index == ctx.props().page_index && overlay.visible)
            .collect();

        let overlay_elements = page_overlays.iter().map(|overlay| {
            let screen_pos = coord_sys.physical_to_screen(overlay.position);
            let width_px = coord_sys.mm_to_px(overlay.size.x);
            let height_px = coord_sys.mm_to_px(overlay.size.y);
            
            let is_selected = self.selected_overlay.as_ref() == Some(&overlay.id);
            let transform_style = overlay.transform.to_css_string();
            
            let overlay_id = overlay.id.clone();
            
            html! {
                <div 
                    key={overlay.id.clone()}
                    class={format!(
                        "overlay {} {} {}", 
                        self.overlay_type_class(&overlay.overlay_type),
                        if is_selected { "selected" } else { "" },
                        if self.dragging_overlay.as_ref() == Some(&overlay.id) { "dragging" } else { "" }
                    )}
                    style={format!(
                        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; transform: {}; z-index: {}",
                        screen_pos.x, screen_pos.y, width_px, height_px, transform_style, overlay.z_index
                    )}
                    onmousedown={ctx.link().callback(move |e: MouseEvent| {
                        OverlayManagerMsg::StartDrag(overlay_id.clone(), e)
                    })}
                    onclick={ctx.link().callback({
                        let overlay_id = overlay.id.clone();
                        move |_| OverlayManagerMsg::SelectOverlay(overlay_id.clone())
                    })}
                >
                    { self.render_overlay_content(overlay) }
                    
                    { if is_selected {
                        self.render_selection_handles()
                    } else {
                        html! {}
                    }}
                </div>
            }
        });

        html! {
            <div class="overlays-container">
                { for overlay_elements }
            </div>
        }
    }

    fn render_overlay_content(&self, overlay: &Overlay) -> Html {
        match overlay.overlay_type {
            OverlayType::TextBox => html! {
                <div class="overlay-textbox">
                    <input type="text" placeholder="Enter text..." class="overlay-text-input" />
                </div>
            },
            OverlayType::Image => html! {
                <div class="overlay-image">
                    <div class="image-placeholder">{"📷 Image"}</div>
                </div>
            },
            OverlayType::Shape => html! {
                <div class="overlay-shape">
                    <div class="shape-rect"></div>
                </div>
            },
            OverlayType::Annotation => html! {
                <div class="overlay-annotation">
                    <div class="annotation-bubble">{"💬"}</div>
                </div>
            },
            OverlayType::Measurement => html! {
                <div class="overlay-measurement">
                    <div class="measurement-line"></div>
                    <div class="measurement-label">{"25.4mm"}</div>
                </div>
            },
        }
    }

    fn render_selection_handles(&self) -> Html {
        html! {
            <div class="selection-handles">
                <div class="handle top-left"></div>
                <div class="handle top-right"></div>
                <div class="handle bottom-left"></div>
                <div class="handle bottom-right"></div>
                <div class="handle top-center"></div>
                <div class="handle bottom-center"></div>
                <div class="handle middle-left"></div>
                <div class="handle middle-right"></div>
            </div>
        }
    }

    fn render_transform_controls(&self, ctx: &Context<Self>, selected_id: &str) -> Html {
        if let Some(overlay) = ctx.props().overlays.iter()
            .find(|o| o.id == *selected_id && o.page_index == ctx.props().page_index) {
            
            html! {
                <div class="transform-controls">
                    <h4>{"Transform Controls"}</h4>
                    
                    <div class="control-group">
                        <label>{"Position"}</label>
                        <div class="input-row">
                            <input 
                                type="number" 
                                value={format!("{:.1}", overlay.transform.translate_x)} 
                                placeholder="X"
                                class="transform-input"
                                step="0.1"
                            />
                            <input 
                                type="number" 
                                value={format!("{:.1}", overlay.transform.translate_y)} 
                                placeholder="Y"
                                class="transform-input"
                                step="0.1"
                            />
                        </div>
                    </div>
                    
                    <div class="control-group">
                        <label>{"Scale"}</label>
                        <div class="input-row">
                            <input 
                                type="number" 
                                value={format!("{:.2}", overlay.transform.scale_x)} 
                                placeholder="Scale X"
                                class="transform-input"
                                step="0.01"
                                min="0.1"
                                max="5.0"
                            />
                            <input 
                                type="number" 
                                value={format!("{:.2}", overlay.transform.scale_y)} 
                                placeholder="Scale Y"
                                class="transform-input"
                                step="0.01"
                                min="0.1"
                                max="5.0"
                            />
                        </div>
                    </div>
                    
                    <div class="control-group">
                        <label>{"Rotation"}</label>
                        <input 
                            type="number" 
                            value={format!("{:.1}", overlay.transform.rotation)} 
                            placeholder="Degrees"
                            class="transform-input"
                            step="1"
                            min="0"
                            max="360"
                        />
                    </div>
                    
                    <div class="control-actions">
                        <button 
                            class="action-button delete"
                            onclick={ctx.link().callback({
                                let overlay_id = selected_id.to_string();
                                move |_| OverlayManagerMsg::DeleteOverlay(overlay_id.clone())
                            })}
                        >
                            {"🗑️ Delete"}
                        </button>
                        <button 
                            class="action-button duplicate"
                            onclick={ctx.link().callback({
                                let overlay_id = selected_id.to_string();
                                move |_| OverlayManagerMsg::DuplicateOverlay(overlay_id.clone())
                            })}
                        >
                            {"📋 Duplicate"}
                        </button>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }

    fn overlay_type_class(&self, overlay_type: &OverlayType) -> &'static str {
        match overlay_type {
            OverlayType::TextBox => "overlay-textbox-type",
            OverlayType::Image => "overlay-image-type",
            OverlayType::Shape => "overlay-shape-type",
            OverlayType::Annotation => "overlay-annotation-type",
            OverlayType::Measurement => "overlay-measurement-type",
        }
    }

    fn apply_snapping(
        &self, 
        transform: &Transform, 
        grid_size_mm: f64, 
        tolerance_mm: f64,
        calibration_manager: &CalibrationManager
    ) -> Transform {
        if let Some(coord_sys) = calibration_manager.get_coordinate_system() {
            let tolerance_px = coord_sys.mm_to_px(tolerance_mm);
            let grid_size_px = coord_sys.mm_to_px(grid_size_mm);
            
            let mut snapped_transform = transform.clone();
            
            // Snap X coordinate
            let snap_x = (transform.translate_x / grid_size_px).round() * grid_size_px;
            if (transform.translate_x - snap_x).abs() <= tolerance_px {
                snapped_transform.translate_x = snap_x;
            }
            
            // Snap Y coordinate  
            let snap_y = (transform.translate_y / grid_size_px).round() * grid_size_px;
            if (transform.translate_y - snap_y).abs() <= tolerance_px {
                snapped_transform.translate_y = snap_y;
            }
            
            snapped_transform
        } else {
            transform.clone()
        }
    }
}