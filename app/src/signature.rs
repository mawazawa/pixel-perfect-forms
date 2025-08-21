//
// ███████╗██╗ ██████╗ ███╗   ██╗ █████╗ ███████╗████████╗███████╗██████╗     ██████╗ ███████╗██╗   ██╗██╗██████╗ ███████╗███████╗
// ██╔════╝██║██╔════╝ ████╗  ██║██╔══██╗██╔════╝╚══██╔══╝██╔════╝██╔══██╗    ██╔══██╗██╔════╝██║   ██║██║██╔══██╗██╔════╝██╔════╝
// ███████╗██║██║      ██╔██╗ ██║███████║███████╗   ██║   █████╗  ██████╔╝    ██████╔╝█████╗  ██║   ██║██║██████╔╝█████╗  █████╗  
// ╚════██║██║██║      ██║╚██╗██║██╔══██║╚════██║   ██║   ██╔══╝  ██╔══██╗    ██╔══██╗██╔══╝  ╚██╗ ██╔╝██║██╔══██╗██╔══╝  ██╔══╝  
// ███████║██║╚██████╗ ██║ ╚████║██║  ██║███████║   ██║   ███████╗██║  ██║    ██║  ██║███████╗ ╚████╔╝ ██║██║  ██║███████╗███████╗
// ╚══════╝╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝    ╚═╝  ╚═╝╚══════╝  ╚═══╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝
//                                                            app/src/signature.rs

use yew::prelude::*;
use crate::coordinates::PhysicalCoord;

#[derive(Properties, PartialEq)]
pub struct SignatureProps {
    pub id: String,
    pub label: String,
    pub position: PhysicalCoord,
    pub width: f64,
    pub height: f64,
    pub on_sign: Callback<String>, // Base64 encoded signature data
    pub signature_data: Option<String>,
}

pub struct Signature {
    node_ref: NodeRef,
    canvas_ref: NodeRef,
    is_drawing: bool,
}

pub enum SignatureMsg {
    StartDrawing(MouseEvent),
    Draw(MouseEvent),
    StopDrawing,
    Clear,
}

impl Component for Signature {
    type Message = SignatureMsg;
    type Properties = SignatureProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            is_drawing: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignatureMsg::StartDrawing(e) => {
                self.is_drawing = true;
                // In a real implementation, we would capture the starting point here
                // For now, we'll just acknowledge the event
                e.prevent_default();
                true
            }
            SignatureMsg::Draw(e) => {
                if self.is_drawing {
                    // In a real implementation, we would draw on the canvas here
                    // For now, we'll just acknowledge the event
                    e.prevent_default();
                }
                true
            }
            SignatureMsg::StopDrawing => {
                self.is_drawing = false;
                // In a real implementation, we would capture the signature data here
                // and emit it via the on_sign callback
                true
            }
            SignatureMsg::Clear => {
                // In a real implementation, we would clear the canvas here
                ctx.props().on_sign.emit(String::new());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let position = ctx.props().position;
        let width = ctx.props().width;
        let height = ctx.props().height;
        
        let style = format!(
            "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
            position.x, position.y, width, height
        );

        html! {
            <div class="form-field signature-field" style={style}>
                <label for={ctx.props().id.clone()}>{ctx.props().label.clone()}</label>
                <canvas
                    ref={self.canvas_ref.clone()}
                    id={ctx.props().id.clone()}
                    width={width.to_string()}
                    height={height.to_string()}
                    onmousedown={ctx.link().callback(|e: MouseEvent| SignatureMsg::StartDrawing(e))}
                    onmousemove={ctx.link().callback(|e: MouseEvent| SignatureMsg::Draw(e))}
                    onmouseup={ctx.link().callback(|_| SignatureMsg::StopDrawing)}
                    onmouseleave={ctx.link().callback(|_| SignatureMsg::StopDrawing)}
                />
                <button onclick={ctx.link().callback(|_| SignatureMsg::Clear)}>
                    {"Clear"}
                </button>
            </div>
        }
    }
}
