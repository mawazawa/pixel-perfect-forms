//
// ██████╗  ██████╗  ██████╗██╗   ██╗███╗   ███╗███████╗███╗   ██╗████████╗    ███╗   ███╗ █████╗ ███╗   ██╗ █████╗  ██████╗ ███████╗██████╗ 
// ██╔══██╗██╔═══██╗██╔════╝██║   ██║████╗ ████║██╔════╝████╗  ██║╚══██╔══╝    ████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝ ██╔════╝██╔══██╗
// ██║  ██║██║   ██║██║     ██║   ██║██╔████╔██║█████╗  ██╔██╗ ██║   ██║       ██╔████╔██║███████║██╔██╗ ██║███████║██║  ███╗█████╗  ██████╔╝
// ██║  ██║██║   ██║██║     ██║   ██║██║╚██╔╝██║██╔══╝  ██║╚██╗██║   ██║       ██║╚██╔╝██║██╔══██║██║╚██╗██║██╔══██║██║   ██║██╔══╝  ██╔══██╗
// ██████╔╝╚██████╔╝╚██████╗╚██████╔╝██║ ╚═╝ ██║███████╗██║ ╚████║   ██║       ██║ ╚═╝ ██║██║  ██║██║ ╚████║██║  ██║╚██████╔╝███████╗██║  ██║
// ╚═════╝  ╚═════╝  ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚══════╝╚═╝  ╚═══╝   ╚═╝       ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝
//                                                            app/src/document_manager.rs

use crate::calibration::CalibrationManager;
use crate::coordinates::{PhysicalCoord, US_LETTER_WIDTH_MM, US_LETTER_HEIGHT_MM};
use crate::overlay_manager::{OverlayManager, Overlay, OverlayType, Transform};
use yew::prelude::*;
use web_sys::HtmlElement;

#[derive(Clone, PartialEq)]
pub struct PageInfo {
    pub page_number: usize,
    pub title: String,
    pub fields: Vec<FormFieldInfo>,
}

#[derive(Clone, PartialEq)]
pub struct FormFieldInfo {
    pub id: String,
    pub field_type: FieldType,
    pub position: PhysicalCoord,
    pub size: PhysicalCoord,
    pub label: String,
}

#[derive(Clone, PartialEq)]
pub enum FieldType {
    TextInput,
    Checkbox,
    Signature,
    Date,
}

#[derive(Properties, PartialEq)]
pub struct DocumentManagerProps {
    pub calibration_manager: CalibrationManager,
}

pub struct DocumentManager {
    pages: Vec<PageInfo>,
    overlays: Vec<Overlay>,
    current_page: usize,
    scroll_container_ref: NodeRef,
    zoom_level: f64,
    show_rulers: bool,
    show_grid: bool,
    grid_size_mm: f64,
    snap_enabled: bool,
    snap_tolerance: f64,
    overlay_counter: usize,
}

pub enum DocumentMsg {
    NavigateToPage(usize),
    NextPage,
    PreviousPage,
    ScrollToPage(usize),
    ZoomIn,
    ZoomOut,
    ResetZoom,
    ToggleRulers,
    ToggleGrid,
    SetGridSize(f64),
    ToggleSnap,
    SetSnapTolerance(f64),
    UpdateOverlay(usize, Overlay),
    CreateOverlay(OverlayType, PhysicalCoord),
    DeleteOverlay(String),
    DuplicateOverlay(String),
}

impl Component for DocumentManager {
    type Message = DocumentMsg;
    type Properties = DocumentManagerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let pages = Self::initialize_fl100_pages();
        
        Self {
            pages,
            overlays: Vec::new(),
            current_page: 0,
            scroll_container_ref: NodeRef::default(),
            zoom_level: 1.0,
            show_rulers: true,
            show_grid: true,
            grid_size_mm: 5.0,
            snap_enabled: true,
            snap_tolerance: 2.0,
            overlay_counter: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DocumentMsg::NavigateToPage(page) => {
                if page < self.pages.len() && page != self.current_page {
                    self.current_page = page;
                    ctx.link().send_message(DocumentMsg::ScrollToPage(page));
                    true
                } else {
                    false
                }
            }
            DocumentMsg::NextPage => {
                if self.current_page < self.pages.len() - 1 {
                    ctx.link().send_message(DocumentMsg::NavigateToPage(self.current_page + 1));
                }
                false
            }
            DocumentMsg::PreviousPage => {
                if self.current_page > 0 {
                    ctx.link().send_message(DocumentMsg::NavigateToPage(self.current_page - 1));
                }
                false
            }
            DocumentMsg::ScrollToPage(page) => {
                self.smooth_scroll_to_page(page);
                false
            }
            DocumentMsg::ZoomIn => {
                self.zoom_level = (self.zoom_level * 1.25).min(4.0);
                true
            }
            DocumentMsg::ZoomOut => {
                self.zoom_level = (self.zoom_level / 1.25).max(0.25);
                true
            }
            DocumentMsg::ResetZoom => {
                self.zoom_level = 1.0;
                true
            }
            DocumentMsg::ToggleRulers => {
                self.show_rulers = !self.show_rulers;
                true
            }
            DocumentMsg::ToggleGrid => {
                self.show_grid = !self.show_grid;
                true
            }
            DocumentMsg::SetGridSize(size) => {
                self.grid_size_mm = size;
                true
            }
            DocumentMsg::ToggleSnap => {
                self.snap_enabled = !self.snap_enabled;
                true
            }
            DocumentMsg::SetSnapTolerance(tolerance) => {
                self.snap_tolerance = tolerance;
                true
            }
            DocumentMsg::UpdateOverlay(page_idx, overlay) => {
                if let Some(existing_overlay) = self.overlays.iter_mut()
                    .find(|o| o.id == overlay.id && o.page_index == page_idx) {
                    *existing_overlay = overlay;
                    true
                } else {
                    false
                }
            }
            DocumentMsg::CreateOverlay(overlay_type, position) => {
                self.overlay_counter += 1;
                let new_overlay = Overlay {
                    id: format!("overlay_{}", self.overlay_counter),
                    page_index: self.current_page,
                    overlay_type,
                    transform: Transform::default(),
                    position,
                    size: PhysicalCoord { x: 30.0, y: 20.0 },
                    visible: true,
                    selected: false,
                    z_index: self.overlay_counter as i32,
                };
                self.overlays.push(new_overlay);
                true
            }
            DocumentMsg::DeleteOverlay(overlay_id) => {
                if let Some(pos) = self.overlays.iter().position(|o| o.id == overlay_id) {
                    self.overlays.remove(pos);
                    true
                } else {
                    false
                }
            }
            DocumentMsg::DuplicateOverlay(overlay_id) => {
                if let Some(original) = self.overlays.iter().find(|o| o.id == overlay_id).cloned() {
                    self.overlay_counter += 1;
                    let mut duplicate = original;
                    duplicate.id = format!("overlay_{}", self.overlay_counter);
                    duplicate.transform.translate_x += 10.0;
                    duplicate.transform.translate_y += 10.0;
                    duplicate.z_index = self.overlay_counter as i32;
                    self.overlays.push(duplicate);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let coord_system = ctx.props().calibration_manager.get_coordinate_system();
        
        html! {
            <div class="document-manager">
                { self.render_toolbar(ctx) }
                
                <div class="document-workspace">
                    { if self.show_rulers && coord_system.is_some() {
                        self.render_rulers(coord_system.unwrap())
                    } else { html! {} }}
                    
                    <div 
                        class="document-scroll-container"
                        ref={self.scroll_container_ref.clone()}
                    >
                        <div 
                            class="document-pages"
                            style={format!("transform: scale({})", self.zoom_level)}
                        >
                            { for self.pages.iter().enumerate().map(|(idx, page)| {
                                self.render_page(ctx, idx, page, coord_system)
                            })}
                        </div>
                        
                        { if self.show_grid && coord_system.is_some() {
                            self.render_global_grid(coord_system.unwrap())
                        } else { html! {} }}
                    </div>
                </div>
                
                { self.render_page_navigator(ctx) }
                { self.render_status_bar(ctx) }
            </div>
        }
    }
}

impl DocumentManager {
    fn initialize_fl100_pages() -> Vec<PageInfo> {
        vec![
            PageInfo {
                page_number: 1,
                title: "FL-100 Page 1 - Petition for Dissolution".to_string(),
                fields: vec![
                    FormFieldInfo {
                        id: "petitioner_name".to_string(),
                        field_type: FieldType::TextInput,
                        position: PhysicalCoord { x: 25.0, y: 45.0 },
                        size: PhysicalCoord { x: 120.0, y: 20.0 },
                        label: "Petitioner Name".to_string(),
                    },
                    FormFieldInfo {
                        id: "respondent_name".to_string(),
                        field_type: FieldType::TextInput,
                        position: PhysicalCoord { x: 25.0, y: 75.0 },
                        size: PhysicalCoord { x: 120.0, y: 20.0 },
                        label: "Respondent Name".to_string(),
                    },
                    FormFieldInfo {
                        id: "case_number".to_string(),
                        field_type: FieldType::TextInput,
                        position: PhysicalCoord { x: 150.0, y: 25.0 },
                        size: PhysicalCoord { x: 80.0, y: 15.0 },
                        label: "Case Number".to_string(),
                    },
                ],
            },
            PageInfo {
                page_number: 2,
                title: "FL-100 Page 2 - Children Information".to_string(),
                fields: vec![
                    FormFieldInfo {
                        id: "child_1_name".to_string(),
                        field_type: FieldType::TextInput,
                        position: PhysicalCoord { x: 25.0, y: 50.0 },
                        size: PhysicalCoord { x: 100.0, y: 18.0 },
                        label: "Child 1 Name".to_string(),
                    },
                    FormFieldInfo {
                        id: "child_1_birthdate".to_string(),
                        field_type: FieldType::Date,
                        position: PhysicalCoord { x: 135.0, y: 50.0 },
                        size: PhysicalCoord { x: 70.0, y: 18.0 },
                        label: "Birth Date".to_string(),
                    },
                    FormFieldInfo {
                        id: "custody_arrangement".to_string(),
                        field_type: FieldType::Checkbox,
                        position: PhysicalCoord { x: 25.0, y: 85.0 },
                        size: PhysicalCoord { x: 15.0, y: 15.0 },
                        label: "Joint Custody".to_string(),
                    },
                ],
            },
            PageInfo {
                page_number: 3,
                title: "FL-100 Page 3 - Property & Financial".to_string(),
                fields: vec![
                    FormFieldInfo {
                        id: "separate_property".to_string(),
                        field_type: FieldType::Checkbox,
                        position: PhysicalCoord { x: 25.0, y: 40.0 },
                        size: PhysicalCoord { x: 15.0, y: 15.0 },
                        label: "Separate Property".to_string(),
                    },
                    FormFieldInfo {
                        id: "community_property".to_string(),
                        field_type: FieldType::Checkbox,
                        position: PhysicalCoord { x: 25.0, y: 65.0 },
                        size: PhysicalCoord { x: 15.0, y: 15.0 },
                        label: "Community Property".to_string(),
                    },
                    FormFieldInfo {
                        id: "petitioner_signature".to_string(),
                        field_type: FieldType::Signature,
                        position: PhysicalCoord { x: 25.0, y: 220.0 },
                        size: PhysicalCoord { x: 150.0, y: 30.0 },
                        label: "Petitioner Signature".to_string(),
                    },
                ],
            },
        ]
    }

    fn render_toolbar(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="document-toolbar">
                <div class="toolbar-section">
                    <h3>{"Navigation"}</h3>
                    <div class="nav-controls">
                        <button 
                            onclick={ctx.link().callback(|_| DocumentMsg::PreviousPage)}
                            disabled={self.current_page == 0}
                            class="nav-button"
                        >
                            {"◀ Prev"}
                        </button>
                        <span class="page-indicator">
                            {format!("Page {} of {}", self.current_page + 1, self.pages.len())}
                        </span>
                        <button 
                            onclick={ctx.link().callback(|_| DocumentMsg::NextPage)}
                            disabled={self.current_page >= self.pages.len() - 1}
                            class="nav-button"
                        >
                            {"Next ▶"}
                        </button>
                    </div>
                </div>
                
                <div class="toolbar-section">
                    <h3>{"Zoom"}</h3>
                    <div class="zoom-controls">
                        <button onclick={ctx.link().callback(|_| DocumentMsg::ZoomOut)}>
                            {"−"}
                        </button>
                        <span class="zoom-display">{format!("{:.0}%", self.zoom_level * 100.0)}</span>
                        <button onclick={ctx.link().callback(|_| DocumentMsg::ZoomIn)}>
                            {"+"}
                        </button>
                        <button 
                            class="reset-zoom"
                            onclick={ctx.link().callback(|_| DocumentMsg::ResetZoom)}
                        >
                            {"Reset"}
                        </button>
                    </div>
                </div>
                
                <div class="toolbar-section">
                    <h3>{"View Options"}</h3>
                    <div class="view-controls">
                        <label class="checkbox-label">
                            <input 
                                type="checkbox" 
                                checked={self.show_rulers}
                                onchange={ctx.link().callback(|_| DocumentMsg::ToggleRulers)}
                            />
                            {"Rulers"}
                        </label>
                        <label class="checkbox-label">
                            <input 
                                type="checkbox" 
                                checked={self.show_grid}
                                onchange={ctx.link().callback(|_| DocumentMsg::ToggleGrid)}
                            />
                            {"Grid"}
                        </label>
                        <label class="checkbox-label">
                            <input 
                                type="checkbox" 
                                checked={self.snap_enabled}
                                onchange={ctx.link().callback(|_| DocumentMsg::ToggleSnap)}
                            />
                            {"Snap"}
                        </label>
                    </div>
                </div>
                
                <div class="toolbar-section">
                    <h3>{"Grid Settings"}</h3>
                    <div class="grid-controls">
                        <button 
                            class={if self.grid_size_mm == 1.0 { "grid-size-button active" } else { "grid-size-button" }}
                            onclick={ctx.link().callback(|_| DocumentMsg::SetGridSize(1.0))}
                        >
                            {"1mm"}
                        </button>
                        <button 
                            class={if self.grid_size_mm == 5.0 { "grid-size-button active" } else { "grid-size-button" }}
                            onclick={ctx.link().callback(|_| DocumentMsg::SetGridSize(5.0))}
                        >
                            {"5mm"}
                        </button>
                        <button 
                            class={if self.grid_size_mm == 10.0 { "grid-size-button active" } else { "grid-size-button" }}
                            onclick={ctx.link().callback(|_| DocumentMsg::SetGridSize(10.0))}
                        >
                            {"10mm"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }

    fn render_page_navigator(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="page-navigator">
                { for (0..self.pages.len()).map(|i| {
                    let is_current = i == self.current_page;
                    let page_info = &self.pages[i];
                    html! {
                        <div 
                            class={if is_current { "page-thumb current" } else { "page-thumb" }}
                            onclick={ctx.link().callback(move |_| DocumentMsg::NavigateToPage(i))}
                        >
                            <div class="page-thumb-number">{i + 1}</div>
                            <div class="page-thumb-title">{&page_info.title}</div>
                        </div>
                    }
                })}
            </div>
        }
    }

    fn render_page(&self, ctx: &Context<Self>, page_idx: usize, page: &PageInfo, coord_system: Option<&crate::coordinates::CoordinateSystem>) -> Html {
        if let Some(coord_sys) = coord_system {
            let width_px = coord_sys.mm_to_px(US_LETTER_WIDTH_MM);
            let height_px = coord_sys.mm_to_px(US_LETTER_HEIGHT_MM);
            
            html! {
                <div 
                    class="document-page"
                    style={format!("width: {}px; height: {}px", width_px, height_px)}
                    data-page={page_idx.to_string()}
                >
                    <div class="page-header">
                        <h2>{&page.title}</h2>
                    </div>
                    
                    <div class="page-content">
                        { self.render_form_fields(coord_sys, &page.fields) }
                        
                        <OverlayManager 
                            calibration_manager={ctx.props().calibration_manager.clone()}
                            page_index={page_idx}
                            overlays={self.overlays.clone()}
                            on_overlay_change={ctx.link().callback(|(page_idx, overlay)| {
                                DocumentMsg::UpdateOverlay(page_idx, overlay)
                            })}
                            snap_enabled={self.snap_enabled}
                            snap_tolerance={self.snap_tolerance}
                            grid_size_mm={self.grid_size_mm}
                        />
                    </div>
                    
                    <div class="page-footer">
                        {"California Judicial Council Form FL-100"}
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="uncalibrated-page">
                    <div class="calibration-prompt">
                        <h3>{"Device Calibration Required"}</h3>
                        <p>{"Please calibrate your device to view pixel-perfect forms."}</p>
                    </div>
                </div>
            }
        }
    }

    fn render_form_fields(&self, coord_sys: &crate::coordinates::CoordinateSystem, fields: &[FormFieldInfo]) -> Html {
        let field_elements = fields.iter().map(|field| {
            let screen_coord = coord_sys.physical_to_screen(field.position);
            let width_px = coord_sys.mm_to_px(field.size.x);
            let height_px = coord_sys.mm_to_px(field.size.y);
            
            html! {
                <div 
                    class="form-field"
                    id={field.id.clone()}
                    style={format!(
                        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px",
                        screen_coord.x, screen_coord.y, width_px, height_px
                    )}
                >
                    { match field.field_type {
                        FieldType::TextInput => html! {
                            <input 
                                type="text" 
                                placeholder={field.label.clone()}
                                class="field-input text-input"
                            />
                        },
                        FieldType::Checkbox => html! {
                            <input 
                                type="checkbox" 
                                class="field-input checkbox-input"
                            />
                        },
                        FieldType::Date => html! {
                            <input 
                                type="date" 
                                class="field-input date-input"
                            />
                        },
                        FieldType::Signature => html! {
                            <div class="field-input signature-input">
                                <span class="signature-placeholder">{"Click to sign"}</span>
                            </div>
                        },
                    }}
                    <div class="field-overlay"></div>
                </div>
            }
        });

        html! {
            <div class="form-fields">
                { for field_elements }
            </div>
        }
    }


    fn render_rulers(&self, coord_sys: &crate::coordinates::CoordinateSystem) -> Html {
        let width_px = coord_sys.mm_to_px(US_LETTER_WIDTH_MM);
        let height_px = coord_sys.mm_to_px(US_LETTER_HEIGHT_MM);
        
        html! {
            <div class="rulers">
                { self.render_horizontal_ruler(coord_sys, width_px) }
                { self.render_vertical_ruler(coord_sys, height_px) }
            </div>
        }
    }

    fn render_horizontal_ruler(&self, coord_sys: &crate::coordinates::CoordinateSystem, width_px: f64) -> Html {
        let marks = (0..=(US_LETTER_WIDTH_MM as u32)).filter_map(|mm| {
            let px = coord_sys.mm_to_px(mm as f64);
            let is_major = mm % 10 == 0;
            let is_minor = mm % 5 == 0;
            
            if is_major || is_minor {
                Some(html! {
                    <div 
                        class={format!("ruler-mark {}", if is_major { "major" } else { "minor" })}
                        style={format!("left: {}px", px)}
                    >
                        { if is_major {
                            html! { <span class="ruler-label">{format!("{}cm", mm / 10)}</span> }
                        } else { html! {} }}
                    </div>
                })
            } else {
                None
            }
        });

        html! {
            <div class="horizontal-ruler" style={format!("width: {}px", width_px)}>
                { for marks }
            </div>
        }
    }

    fn render_vertical_ruler(&self, coord_sys: &crate::coordinates::CoordinateSystem, height_px: f64) -> Html {
        let marks = (0..=(US_LETTER_HEIGHT_MM as u32)).filter_map(|mm| {
            let px = coord_sys.mm_to_px(mm as f64);
            let is_major = mm % 10 == 0;
            let is_minor = mm % 5 == 0;
            
            if is_major || is_minor {
                Some(html! {
                    <div 
                        class={format!("ruler-mark {}", if is_major { "major" } else { "minor" })}
                        style={format!("top: {}px", px)}
                    >
                        { if is_major {
                            html! { <span class="ruler-label">{format!("{}cm", mm / 10)}</span> }
                        } else { html! {} }}
                    </div>
                })
            } else {
                None
            }
        });

        html! {
            <div class="vertical-ruler" style={format!("height: {}px", height_px)}>
                { for marks }
            </div>
        }
    }

    fn render_global_grid(&self, coord_sys: &crate::coordinates::CoordinateSystem) -> Html {
        let width_px = coord_sys.mm_to_px(US_LETTER_WIDTH_MM);
        let height_px = coord_sys.mm_to_px(US_LETTER_HEIGHT_MM) * self.pages.len() as f64;
        let grid_size_px = coord_sys.mm_to_px(self.grid_size_mm);

        html! {
            <div 
                class="global-grid-overlay"
                style={format!(
                    "background-size: {}px {}px; width: {}px; height: {}px; opacity: {}",
                    grid_size_px, grid_size_px, width_px, height_px,
                    if self.snap_enabled { "0.8" } else { "0.3" }
                )}
            ></div>
        }
    }

    fn render_status_bar(&self, ctx: &Context<Self>) -> Html {
        let coord_system = ctx.props().calibration_manager.get_coordinate_system();
        
        html! {
            <div class="status-bar">
                <div class="status-section">
                    <span class="status-label">{"Calibration:"}</span>
                    <span class={if coord_system.is_some() { "status-good" } else { "status-warning" }}>
                        { if coord_system.is_some() { "✓ Active" } else { "⚠ Required" }}
                    </span>
                </div>
                
                { if let Some(coord_sys) = coord_system {
                    let calibration = coord_sys.get_calibration();
                    html! {
                        <div class="status-section">
                            <span class="status-label">{"DPI:"}</span>
                            <span class="status-value">{format!("{:.1}", calibration.scale_factor * 25.4)}</span>
                        </div>
                    }
                } else { html! {} }}
                
                <div class="status-section">
                    <span class="status-label">{"Page:"}</span>
                    <span class="status-value">{format!("{} of {}", self.current_page + 1, self.pages.len())}</span>
                </div>
                
                <div class="status-section">
                    <span class="status-label">{"Zoom:"}</span>
                    <span class="status-value">{format!("{:.0}%", self.zoom_level * 100.0)}</span>
                </div>
                
                <div class="status-section">
                    <span class="status-label">{"Grid:"}</span>
                    <span class="status-value">{format!("{}mm", self.grid_size_mm)}</span>
                </div>
                
                <div class="status-section">
                    <span class="status-label">{"Snap:"}</span>
                    <span class={if self.snap_enabled { "status-good" } else { "status-disabled" }}>
                        { if self.snap_enabled { "ON" } else { "OFF" }}
                    </span>
                </div>
            </div>
        }
    }

    fn smooth_scroll_to_page(&self, page: usize) {
        if let Some(container) = self.scroll_container_ref.cast::<HtmlElement>() {
            let page_height = 800.0; // Approximate page height in pixels
            let target_scroll = page as f64 * page_height;
            
            // Use simple scroll for now (smooth scrolling requires more web-sys features)
            container.set_scroll_top(target_scroll as i32);
        }
    }
}