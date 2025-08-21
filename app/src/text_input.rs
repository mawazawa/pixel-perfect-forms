//
// ████████╗███████╗██╗  ██╗███████╗██████╗ ██╗   ██╗    ██╗███╗   ██╗████████╗██████╗  █████╗ ███████╗████████╗ █████╗ ██╗██╗     
// ╚══██╔══╝██╔════╝╚██╗██╔╝██╔════╝██╔══██╗╚██╗ ██╔╝    ██║████╗  ██║╚══██╔══╝██╔══██╗██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██║██║     
//    ██║   █████╗   ╚███╔╝ █████╗  ██████╔╝ ╚████╔╝     ██║██╔██╗ ██║   ██║   ██████╔╝███████║███████╗   ██║   ███████║██║██║     
//    ██║   ██╔══╝   ██╔██╗ ██╔══╝  ██╔══██╗  ╚██╔╝      ██║██║╚██╗██║   ██║   ██╔══██╗██╔══██║╚════██║   ██║   ██╔══██║██║██║     
//    ██║   ███████╗██╔╝ ██╗███████╗██║  ██║   ██║       ██║██║ ╚████║   ██║   ██║  ██║██║  ██║███████║   ██║   ██║  ██║██║███████╗
//    ╚═╝   ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝   ╚═╝       ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝╚══════╝
//                                                            app/src/text_input.rs

use yew::prelude::*;
use crate::coordinates::{PhysicalCoord, CoordinateSystem};
use crate::font_metrics::{FontMetricsCalculator, StandardFont, TextPosition};
use web_sys::HtmlInputElement;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub id: String,
    pub label: String,
    pub position: PhysicalCoord,
    pub width_mm: f64,
    pub height_mm: f64,
    pub on_input: Callback<String>,
    pub value: String,
    pub font: Option<StandardFont>,
    pub font_size_pt: Option<f64>,
    pub coord_system: Option<CoordinateSystem>,
}

pub struct TextInput {
    node_ref: NodeRef,
    font_calculator: Rc<RefCell<FontMetricsCalculator>>,
}

pub enum TextInputMsg {
    Input(String),
}

impl Component for TextInput {
    type Message = TextInputMsg;
    type Properties = TextInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            font_calculator: Rc::new(RefCell::new(FontMetricsCalculator::new())),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextInputMsg::Input(value) => {
                ctx.props().on_input.emit(value);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        
        // Use font metrics for precise positioning if coordinate system is available
        let style = if let Some(coord_system) = &props.coord_system {
            let font = props.font.clone().unwrap_or(StandardFont::Helvetica);
            let font_size = props.font_size_pt.unwrap_or(12.0);
            
            // Calculate precise text positioning
            if let Ok(mut calculator) = self.font_calculator.try_borrow_mut() {
                if let Some(text_pos) = calculator.calculate_text_position(
                    font,
                    font_size,
                    props.position,
                    props.height_mm,
                    coord_system,
                ) {
                    // Convert field dimensions to screen coordinates
                    let field_width_px = coord_system.mm_to_px(props.width_mm);
                    let field_height_px = coord_system.mm_to_px(props.height_mm);
                    let field_screen = coord_system.physical_to_screen(props.position);
                    
                    format!(
                        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; {}",
                        field_screen.x,
                        field_screen.y,
                        field_width_px,
                        field_height_px,
                        self.get_input_style(&text_pos)
                    )
                } else {
                    // Fallback to basic positioning
                    self.get_fallback_style(coord_system, props)
                }
            } else {
                // Fallback if calculator is borrowed
                self.get_fallback_style(coord_system, props)
            }
        } else {
            // Legacy positioning without coordinate system
            format!(
                "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
                props.position.x, props.position.y, props.width_mm, props.height_mm
            )
        };

        html! {
            <div class="form-field text-input" style={style}>
                <label for={props.id.clone()}>{props.label.clone()}</label>
                <input
                    ref={self.node_ref.clone()}
                    type="text"
                    id={props.id.clone()}
                    value={props.value.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        TextInputMsg::Input(input.value())
                    })}
                />
            </div>
        }
    }
}

impl TextInput {
    /// Generate input style with precise font metrics
    fn get_input_style(&self, text_pos: &TextPosition) -> String {
        format!(
            "border: none; background: transparent; padding: 0; margin: 0; {}",
            text_pos.to_css_style()
        )
    }

    /// Fallback styling without font metrics
    fn get_fallback_style(&self, coord_system: &CoordinateSystem, props: &TextInputProps) -> String {
        let field_width_px = coord_system.mm_to_px(props.width_mm);
        let field_height_px = coord_system.mm_to_px(props.height_mm);
        let field_screen = coord_system.physical_to_screen(props.position);
        
        format!(
            "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
            field_screen.x, field_screen.y, field_width_px, field_height_px
        )
    }
}
