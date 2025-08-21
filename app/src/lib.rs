// 
// ███╗   ██╗███████╗██╗    ██╗    ██╗     ██╗██████╗      ██╗     ██╗██████╗ 
// ████╗  ██║██╔════╝██║    ██║    ██║     ██║██╔══██╗     ██║     ██║██╔══██╗
// ██╔██╗ ██║█████╗  ██║ █╗ ██║    ██║     ██║██████╔╝     ██║     ██║██████╔╝
// ██║╚██╗██║██╔══╝  ██║███╗██║    ██║     ██║██╔══██╗     ██║     ██║██╔══██╗
// ██║ ╚████║███████╗╚███╔███╔╝    ███████╗██║██║  ██║     ███████╗██║██║  ██║
// ╚═╝  ╚═══╝╚══════╝ ╚══╝╚══╝     ╚══════╝╚═╝╚═╝  ╚═╝     ╚══════╝╚═╝╚═╝  ╚═╝
//                                                            app/src/lib.rs

mod coordinates;
mod calibration;
mod calibration_wizard;
mod proofing_ui;
mod text_input;
mod checkbox;
mod signature;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use calibration::CalibrationManager;
use calibration_wizard::CalibrationWizard;
use proofing_ui::ProofingUI;

// Use wee_alloc as the global allocator for smaller WASM size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub struct App {
    show_calibration: bool,
    calibration_manager: CalibrationManager,
}

pub enum AppMsg {
    ShowCalibration,
    HideCalibration,
    CalibrationComplete,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let calibration_manager = CalibrationManager::new();
        let needs_calibration = calibration_manager.needs_recalibration();
        
        Self {
            show_calibration: needs_calibration,
            calibration_manager,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ShowCalibration => {
                self.show_calibration = true;
                true
            }
            AppMsg::HideCalibration => {
                self.show_calibration = false;
                true
            }
            AppMsg::CalibrationComplete => {
                self.calibration_manager = CalibrationManager::new();
                self.show_calibration = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.show_calibration {
            html! {
                <main class="app calibration-mode">
                    <CalibrationWizard 
                        on_complete={ctx.link().callback(|_| AppMsg::CalibrationComplete)}
                        on_cancel={ctx.link().callback(|_| AppMsg::HideCalibration)}
                    />
                </main>
            }
        } else {
            html! {
                <main class="app">
                    <header class="app-header">
                        <h1>{"FL-100 Pixel‑Perfect WASM Forms"}</h1>
                        <div class="header-controls">
                            <button 
                                class="calibration-button"
                                onclick={ctx.link().callback(|_| AppMsg::ShowCalibration)}
                            >
                                {"📏 Calibrate Device"}
                            </button>
                        </div>
                    </header>
                    
                    <ProofingUI calibration_manager={self.calibration_manager.clone()} />
                </main>
            }
        }
    }
}

pub fn start_app() {
    yew::Renderer::<App>::new().render();
}

#[wasm_bindgen(start)]  
pub fn wasm_main() {
    console_error_panic_hook::set_once();
    start_app();
}
