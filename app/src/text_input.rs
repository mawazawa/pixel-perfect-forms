//
// ████████╗███████╗██╗  ██╗███████╗██████╗ ██╗   ██╗    ██╗███╗   ██╗████████╗██████╗  █████╗ ███████╗████████╗ █████╗ ██╗██╗     
// ╚══██╔══╝██╔════╝╚██╗██╔╝██╔════╝██╔══██╗╚██╗ ██╔╝    ██║████╗  ██║╚══██╔══╝██╔══██╗██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██║██║     
//    ██║   █████╗   ╚███╔╝ █████╗  ██████╔╝ ╚████╔╝     ██║██╔██╗ ██║   ██║   ██████╔╝███████║███████╗   ██║   ███████║██║██║     
//    ██║   ██╔══╝   ██╔██╗ ██╔══╝  ██╔══██╗  ╚██╔╝      ██║██║╚██╗██║   ██║   ██╔══██╗██╔══██║╚════██║   ██║   ██╔══██║██║██║     
//    ██║   ███████╗██╔╝ ██╗███████╗██║  ██║   ██║       ██║██║ ╚████║   ██║   ██║  ██║██║  ██║███████║   ██║   ██║  ██║██║███████╗
//    ╚═╝   ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝   ╚═╝       ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝╚══════╝
//                                                            app/src/text_input.rs

use yew::prelude::*;
use crate::coordinates::PhysicalCoord;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub id: String,
    pub label: String,
    pub position: PhysicalCoord,
    pub width: f64,
    pub height: f64,
    pub on_input: Callback<String>,
    pub value: String,
}

pub struct TextInput {
    node_ref: NodeRef,
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
        let position = ctx.props().position;
        let width = ctx.props().width;
        let height = ctx.props().height;
        
        let style = format!(
            "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
            position.x, position.y, width, height
        );

        html! {
            <div class="form-field text-input" style={style}>
                <label for={ctx.props().id.clone()}>{ctx.props().label.clone()}</label>
                <input
                    ref={self.node_ref.clone()}
                    type="text"
                    id={ctx.props().id.clone()}
                    value={ctx.props().value.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        TextInputMsg::Input(input.value())
                    })}
                />
            </div>
        }
    }
}
