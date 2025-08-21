//
// ██████╗ ██████╗  ██████╗  ██████╗ ███████╗██╗███╗   ██╗ ██████╗     ██╗   ██╗██╗
// ██╔══██╗██╔══██╗██╔═══██╗██╔═══██╗██╔════╝██║████╗  ██║██╔════╝     ██║   ██║██║
// ██████╔╝██████╔╝██║   ██║██║   ██║█████╗  ██║██╔██╗ ██║██║  ███╗    ██║   ██║██║
// ██╔═══╝ ██╔══██╗██║   ██║██║   ██║██╔══╝  ██║██║╚██╗██║██║   ██║    ██║   ██║██║
// ██║     ██║  ██║╚██████╔╝╚██████╔╝██║     ██║██║ ╚████║╚██████╔╝    ╚██████╔╝██║
// ╚═╝     ╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═══╝ ╚═════╝      ╚═════╝ ╚═╝
//                                                     app/src/proofing_ui.rs

use crate::calibration::CalibrationManager;
use crate::coordinates::{PhysicalCoord, ScreenCoord, US_LETTER_WIDTH_MM, US_LETTER_HEIGHT_MM};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProofingUIProps {
    pub calibration_manager: CalibrationManager,
}


pub struct ProofingUI {
    page_container_ref: NodeRef,
    current_page: usize,
    zoom_level: f64,
    show_rulers: bool,
    show_grid: bool,
}

pub enum ProofingMsg {
    SetPage(usize),
    ZoomIn,
    ZoomOut,
    ResetZoom,
    ToggleRulers,
    ToggleGrid,
}

impl Component for ProofingUI {
    type Message = ProofingMsg;
    type Properties = ProofingUIProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            page_container_ref: NodeRef::default(),
            current_page: 0,
            zoom_level: 1.0,
            show_rulers: true,
            show_grid: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProofingMsg::SetPage(page) => {
                self.current_page = page;
                true
            }
            ProofingMsg::ZoomIn => {
                self.zoom_level = (self.zoom_level * 1.25).min(4.0);
                true
            }
            ProofingMsg::ZoomOut => {
                self.zoom_level = (self.zoom_level / 1.25).max(0.25);
                true
            }
            ProofingMsg::ResetZoom => {
                self.zoom_level = 1.0;
                true
            }
            ProofingMsg::ToggleRulers => {
                self.show_rulers = !self.show_rulers;
                true
            }
            ProofingMsg::ToggleGrid => {
                self.show_grid = !self.show_grid;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let coord_system = ctx.props().calibration_manager.get_coordinate_system();
        
        html! {
            <div class="proofing-ui">
                { self.render_toolbar(ctx) }
                
                <div class="proofing-workspace">
                    { if self.show_rulers {
                        self.render_rulers(coord_system)
                    } else { html! {} }}
                    
                    <div 
                        class="page-container"
                        ref={self.page_container_ref.clone()}
                        style={format!("transform: scale({})", self.zoom_level)}
                    >
                        { self.render_page(ctx, coord_system) }
                        
                        { if self.show_grid {
                            self.render_grid(coord_system)
                        } else { html! {} }}
                    </div>
                </div>
                
                { self.render_status_bar(ctx) }
            </div>
        }
    }
}

impl ProofingUI {
    fn render_toolbar(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="proofing-toolbar">
                <div class="toolbar-section">
                    <h3>{"Pages"}</h3>
                    <div class="page-controls">
                        { for (0..3).map(|i| {
                            let is_active = i == self.current_page;
                            let set_page = ctx.link().callback(move |_| ProofingMsg::SetPage(i));
                            html! {
                                <button 
                                    class={if is_active { "page-button active" } else { "page-button" }}
                                    onclick={set_page}
                                >
                                    {format!("{}", i + 1)}
                                </button>
                            }
                        })}
                    </div>
                </div>
                
                <div class="toolbar-section">
                    <h3>{"Zoom"}</h3>
                    <div class="zoom-controls">
                        <button onclick={ctx.link().callback(|_| ProofingMsg::ZoomOut)}>
                            {"−"}
                        </button>
                        <span class="zoom-display">{format!("{:.0}%", self.zoom_level * 100.0)}</span>
                        <button onclick={ctx.link().callback(|_| ProofingMsg::ZoomIn)}>
                            {"+"}
                        </button>
                        <button 
                            class="reset-zoom"
                            onclick={ctx.link().callback(|_| ProofingMsg::ResetZoom)}
                        >
                            {"Reset"}
                        </button>
                    </div>
                </div>
                
                <div class="toolbar-section">
                    <h3>{"View"}</h3>
                    <div class="view-controls">
                        <label class="checkbox-label">
                            <input 
                                type="checkbox" 
                                checked={self.show_rulers}
                                onchange={ctx.link().callback(|_| ProofingMsg::ToggleRulers)}
                            />
                            {"Rulers"}
                        </label>
                        <label class="checkbox-label">
                            <input 
                                type="checkbox" 
                                checked={self.show_grid}
                                onchange={ctx.link().callback(|_| ProofingMsg::ToggleGrid)}
                            />
                            {"Grid"}
                        </label>
                    </div>
                </div>
            </div>
        }
    }

    fn render_rulers(&self, coord_system: Option<&crate::coordinates::CoordinateSystem>) -> Html {
        if let Some(coord_sys) = coord_system {
            let width_px = coord_sys.mm_to_px(US_LETTER_WIDTH_MM);
            let height_px = coord_sys.mm_to_px(US_LETTER_HEIGHT_MM);
            
            html! {
                <div class="rulers">
                    { self.render_horizontal_ruler(coord_sys, width_px) }
                    { self.render_vertical_ruler(coord_sys, height_px) }
                </div>
            }
        } else {
            html! {
                <div class="rulers-uncalibrated">
                    <p>{"Rulers require device calibration"}</p>
                </div>
            }
        }
    }

    fn render_horizontal_ruler(&self, coord_sys: &crate::coordinates::CoordinateSystem, width_px: f64) -> Html {
        let major_marks = (0..=(US_LETTER_WIDTH_MM as u32 / 10)).map(|cm| {
            let mm = cm as f64 * 10.0;
            let px = coord_sys.mm_to_px(mm);
            html! {
                <div 
                    class="ruler-mark major"
                    style={format!("left: {}px", px)}
                >
                    <span class="ruler-label">{format!("{}cm", cm)}</span>
                </div>
            }
        });

        html! {
            <div class="horizontal-ruler" style={format!("width: {}px", width_px)}>
                { for major_marks }
            </div>
        }
    }

    fn render_vertical_ruler(&self, coord_sys: &crate::coordinates::CoordinateSystem, height_px: f64) -> Html {
        let major_marks = (0..=(US_LETTER_HEIGHT_MM as u32 / 10)).map(|cm| {
            let mm = cm as f64 * 10.0;
            let px = coord_sys.mm_to_px(mm);
            html! {
                <div 
                    class="ruler-mark major"
                    style={format!("top: {}px", px)}
                >
                    <span class="ruler-label">{format!("{}cm", cm)}</span>
                </div>
            }
        });

        html! {
            <div class="vertical-ruler" style={format!("height: {}px", height_px)}>
                { for major_marks }
            </div>
        }
    }

    fn render_page(&self, _ctx: &Context<Self>, coord_system: Option<&crate::coordinates::CoordinateSystem>) -> Html {
        if let Some(coord_sys) = coord_system {
            let width_px = coord_sys.mm_to_px(US_LETTER_WIDTH_MM);
            let height_px = coord_sys.mm_to_px(US_LETTER_HEIGHT_MM);
            
            html! {
                <div 
                    class="form-page"
                    style={format!("width: {}px; height: {}px", width_px, height_px)}
                >
                    <div class="page-header">
                        {format!("FL-100 Form - Page {}", self.current_page + 1)}
                    </div>
                    
                    <div class="form-content">
                        { self.render_form_fields(coord_sys) }
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

    fn render_form_fields(&self, coord_sys: &crate::coordinates::CoordinateSystem) -> Html {
        // Sample form fields with physical coordinates
        let sample_fields = vec![
            ("petitioner_name", PhysicalCoord { x: 25.0, y: 45.0 }, "Petitioner Name"),
            ("respondent_name", PhysicalCoord { x: 25.0, y: 75.0 }, "Respondent Name"),
            ("case_number", PhysicalCoord { x: 150.0, y: 25.0 }, "Case Number"),
            ("county", PhysicalCoord { x: 25.0, y: 105.0 }, "County"),
        ];

        let field_elements = sample_fields.iter().map(|(id, coord, label)| {
            let screen_coord = coord_sys.physical_to_screen(*coord);
            html! {
                <div 
                    class="form-field"
                    id={id.to_string()}
                    style={format!(
                        "position: absolute; left: {}px; top: {}px; width: 120px; height: 20px",
                        screen_coord.x, screen_coord.y
                    )}
                >
                    <input 
                        type="text" 
                        placeholder={label.to_string()}
                        class="field-input"
                    />
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

    fn render_grid(&self, coord_system: Option<&crate::coordinates::CoordinateSystem>) -> Html {
        if let Some(coord_sys) = coord_system {
            let grid_size_mm = 5.0; // 5mm grid
            let width_px = coord_sys.mm_to_px(US_LETTER_WIDTH_MM);
            let height_px = coord_sys.mm_to_px(US_LETTER_HEIGHT_MM);
            let grid_size_px = coord_sys.mm_to_px(grid_size_mm);

            html! {
                <div 
                    class="grid-overlay"
                    style={format!(
                        "background-size: {}px {}px; width: {}px; height: {}px",
                        grid_size_px, grid_size_px, width_px, height_px
                    )}
                ></div>
            }
        } else {
            html! {}
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
                    <span class="status-value">{format!("{} of 3", self.current_page + 1)}</span>
                </div>
                
                <div class="status-section">
                    <span class="status-label">{"Zoom:"}</span>
                    <span class="status-value">{format!("{:.0}%", self.zoom_level * 100.0)}</span>
                </div>
            </div>
        }
    }
}