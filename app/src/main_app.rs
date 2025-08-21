//
// ███╗   ███╗ █████╗ ██╗███╗   ██╗     █████╗ ██████╗ ██████╗ 
// ████╗ ████║██╔══██╗██║████╗  ██║    ██╔══██╗██╔══██╗██╔══██╗
// ██╔████╔██║███████║██║██╔██╗ ██║    ███████║██████╔╝██████╔╝
// ██║╚██╔╝██║██╔══██║██║██║╚██╗██║    ██╔══██║██╔═══╝ ██╔═══╝ 
// ██║ ╚═╝ ██║██║  ██║██║██║ ╚████║    ██║  ██║██║     ██║     
// ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝    ╚═╝  ╚═╝╚═╝     ╚═╝     
//                                          app/src/main_app.rs

use crate::calibration::CalibrationManager;
use crate::document_manager::DocumentManager;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainAppProps {
    #[prop_or_default]
    pub calibration_manager: Option<CalibrationManager>,
}

pub struct MainApp {
    calibration_manager: CalibrationManager,
}

impl Component for MainApp {
    type Message = ();
    type Properties = MainAppProps;

    fn create(ctx: &Context<Self>) -> Self {
        let calibration_manager = ctx.props().calibration_manager
            .clone()
            .unwrap_or_else(CalibrationManager::new);
            
        Self {
            calibration_manager,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="main-app">
                <DocumentManager calibration_manager={self.calibration_manager.clone()} />
            </div>
        }
    }
}
