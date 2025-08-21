//
// ██████╗██╗  ██╗███████╗ ██████╗██╗  ██╗███████╗ █████╗ ██╗██╗     ██████╗ ███████╗███████╗██╗  ██╗███████╗██████╗ 
// ██╔════╝██║  ██║██╔════╝██╔════╝██║ ██╔╝██╔════╝██╔══██╗██║██║     ██╔══██╗██╔════╝██╔════╝██║  ██║██╔════╝██╔══██╗
// ██║     ███████║█████╗  ██║     █████╔╝ █████╗  ███████║██║██║     ██████╔╝█████╗  █████╗  ███████║█████╗  ██████╔╝
// ██║     ██╔══██║██╔══╝  ██║     ██╔═██╗ ██╔══╝  ██╔══██║██║██║     ██╔══██╗██╔══╝  ██╔══╝  ██╔══██║██╔══╝  ██╔══██╗
// ╚██████╗██║  ██║███████╗╚██████╗██║  ██╗███████╗██║  ██║██║██████╗██████╔╝██║     ███████╗██║  ██║███████╗██║  ██║
//  ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝╚══════╝╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝
//                                                            app/src/checkbox.rs

use yew::prelude::*;
use crate::coordinates::PhysicalCoord;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    pub id: String,
    pub label: String,
    pub position: PhysicalCoord,
    pub on_toggle: Callback<bool>,
    pub checked: bool,
}

pub struct Checkbox {
    node_ref: NodeRef,
}

pub enum CheckboxMsg {
    Toggle(bool),
}

impl Component for Checkbox {
    type Message = CheckboxMsg;
    type Properties = CheckboxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CheckboxMsg::Toggle(checked) => {
                ctx.props().on_toggle.emit(checked);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let position = ctx.props().position;
        
        let style = format!(
            "position: absolute; left: {}px; top: {}px;",
            position.x, position.y
        );

        html! {
            <div class="form-field checkbox" style={style}>
                <input
                    ref={self.node_ref.clone()}
                    type="checkbox"
                    id={ctx.props().id.clone()}
                    checked={ctx.props().checked}
                    onclick={ctx.link().callback(|e: MouseEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        CheckboxMsg::Toggle(input.checked())
                    })}
                />
                <label for={ctx.props().id.clone()}>{ctx.props().label.clone()}</label>
            </div>
        }
    }
}
