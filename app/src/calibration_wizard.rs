//
// ██╗    ██╗██╗███████╗ █████╗ ██████╗ ██████╗     ██╗   ██╗██╗
// ██║    ██║██║╚══███╔╝██╔══██╗██╔══██╗██╔══██╗    ██║   ██║██║
// ██║ █╗ ██║██║  ███╔╝ ███████║██████╔╝██║  ██║    ██║   ██║██║
// ██║███╗██║██║ ███╔╝  ██╔══██║██╔══██╗██║  ██║    ██║   ██║██║
// ╚███╔███╔╝██║███████╗██║  ██║██║  ██║██████╔╝    ╚██████╔╝██║
//  ╚══╝╚══╝ ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝      ╚═════╝ ╚═╝
//                                          app/src/calibration_wizard.rs

use crate::calibration::{CalibrationManager, CalibrationStep, CalibrationState};
use yew::prelude::*;
use web_sys::{HtmlInputElement, MouseEvent};

#[derive(Properties, PartialEq)]
pub struct CalibrationWizardProps {
    pub on_complete: Callback<()>,
    pub on_cancel: Option<Callback<()>>,
}

pub struct CalibrationWizard {
    manager: CalibrationManager,
    measurement_input: NodeRef,
    measuring_start_x: Option<f64>,
    measuring_end_x: Option<f64>,
}

pub enum CalibrationMsg {
    StartCalibration,
    NextStep,
    PreviousStep,
    ProcessInput,
    StartMeasuring(MouseEvent),
    EndMeasuring(MouseEvent),
    CompleteCalibration,
    Cancel,
    ResetCalibration,
}

impl Component for CalibrationWizard {
    type Message = CalibrationMsg;
    type Properties = CalibrationWizardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            manager: CalibrationManager::new(),
            measurement_input: NodeRef::default(),
            measuring_start_x: None,
            measuring_end_x: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CalibrationMsg::StartCalibration => {
                self.manager.start_calibration();
                true
            }
            CalibrationMsg::NextStep => {
                match self.manager.get_state().step {
                    CalibrationStep::Instructions => {
                        // Move to measuring step
                        self.manager.state.step = CalibrationStep::Measuring;
                    }
                    _ => {}
                }
                true
            }
            CalibrationMsg::PreviousStep => {
                match self.manager.get_state().step {
                    CalibrationStep::Instructions => {
                        self.manager.state.step = CalibrationStep::Welcome;
                    }
                    CalibrationStep::Measuring => {
                        self.manager.state.step = CalibrationStep::Instructions;
                    }
                    CalibrationStep::Validation => {
                        self.manager.state.step = CalibrationStep::Measuring;
                    }
                    _ => {}
                }
                true
            }
            CalibrationMsg::ProcessInput => {
                if let Some(input) = self.measurement_input.cast::<HtmlInputElement>() {
                    if let Ok(pixels) = input.value().parse::<f64>() {
                        match self.manager.process_measurement(pixels) {
                            Ok(_) => {
                                self.manager.state.error_message = None;
                            }
                            Err(error) => {
                                self.manager.state.error_message = Some(error);
                            }
                        }
                    } else {
                        self.manager.state.error_message = Some("Please enter a valid number".to_string());
                    }
                }
                true
            }
            CalibrationMsg::StartMeasuring(event) => {
                self.measuring_start_x = Some(event.client_x() as f64);
                self.measuring_end_x = None;
                true
            }
            CalibrationMsg::EndMeasuring(event) => {
                if let Some(start_x) = self.measuring_start_x {
                    let end_x = event.client_x() as f64;
                    let measured_pixels = (end_x - start_x).abs();
                    
                    match self.manager.process_measurement(measured_pixels) {
                        Ok(_) => {
                            self.manager.state.error_message = None;
                        }
                        Err(error) => {
                            self.manager.state.error_message = Some(error);
                        }
                    }
                    
                    self.measuring_start_x = None;
                    self.measuring_end_x = Some(end_x);
                }
                true
            }
            CalibrationMsg::CompleteCalibration => {
                match self.manager.complete_calibration() {
                    Ok(_) => {
                        ctx.props().on_complete.emit(());
                    }
                    Err(error) => {
                        self.manager.state.error_message = Some(error);
                    }
                }
                true
            }
            CalibrationMsg::Cancel => {
                if let Some(on_cancel) = &ctx.props().on_cancel {
                    on_cancel.emit(());
                }
                false
            }
            CalibrationMsg::ResetCalibration => {
                self.manager.reset_calibration();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = self.manager.get_state();
        
        html! {
            <div class="calibration-wizard">
                { self.render_header() }
                
                <div class="calibration-content">
                    { match state.step {
                        CalibrationStep::Welcome => self.render_welcome(ctx),
                        CalibrationStep::Instructions => self.render_instructions(ctx),
                        CalibrationStep::Measuring => self.render_measuring(ctx),
                        CalibrationStep::Validation => self.render_validation(ctx),
                        CalibrationStep::Complete => self.render_complete(ctx),
                    }}
                </div>
                
                { self.render_error_message() }
                { self.render_navigation(ctx) }
            </div>
        }
    }
}

impl CalibrationWizard {
    fn render_header(&self) -> Html {
        html! {
            <header class="calibration-header">
                <h1>{"Device Calibration"}</h1>
                <p class="subtitle">{"Ensure pixel-perfect accuracy for form rendering"}</p>
            </header>
        }
    }

    fn render_welcome(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="welcome-step">
                <div class="step-icon">{"📏"}</div>
                <h2>{"Welcome to Device Calibration"}</h2>
                <p class="description">
                    {"This calibration process ensures that forms rendered on your screen 
                     match their physical dimensions with sub-millimeter accuracy."}
                </p>
                <div class="benefits">
                    <h3>{"Why calibrate?"}</h3>
                    <ul>
                        <li>{"🎯 Pixel-perfect form accuracy (<0.25mm)"}</li>
                        <li>{"📄 Consistent print-to-screen matching"}</li>
                        <li>{"⚖️ Legal document compliance"}</li>
                        <li>{"🖥️ Optimized for your specific display"}</li>
                    </ul>
                </div>
                <button 
                    class="primary-button"
                    onclick={ctx.link().callback(|_| CalibrationMsg::StartCalibration)}
                >
                    {"Start Calibration"}
                </button>
            </div>
        }
    }

    fn render_instructions(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="instructions-step">
                <div class="step-icon">{"📐"}</div>
                <h2>{"Calibration Instructions"}</h2>
                <div class="instructions">
                    <p><strong>{"You will need:"}</strong></p>
                    <ul>
                        <li>{"A physical ruler (preferably 10cm/100mm)"}</li>
                        <li>{"Good lighting"}</li>
                        <li>{"A steady hand"}</li>
                    </ul>
                    
                    <p><strong>{"Steps:"}</strong></p>
                    <ol>
                        <li>{"Place your ruler against the screen"}</li>
                        <li>{"Align the 0mm mark with the left edge of the ruler graphic"}</li>
                        <li>{"Click and drag to measure exactly 100mm"}</li>
                        <li>{"We'll calculate your display's pixel density"}</li>
                    </ol>
                    
                    <div class="tip">
                        <strong>{"💡 Tip:"}</strong> {" For best results, ensure your browser 
                        is at 100% zoom and your display brightness is comfortable."}
                    </div>
                </div>
            </div>
        }
    }

    fn render_measuring(&self, ctx: &Context<Self>) -> Html {
        let start_measuring = ctx.link().callback(CalibrationMsg::StartMeasuring);
        let end_measuring = ctx.link().callback(CalibrationMsg::EndMeasuring);
        let process_input = ctx.link().callback(|_| CalibrationMsg::ProcessInput);

        html! {
            <div class="measuring-step">
                <div class="step-icon">{"🎯"}</div>
                <h2>{"Measure 100mm"}</h2>
                
                <div class="ruler-container">
                    <div class="ruler-instructions">
                        {"Place your physical ruler here and drag from 0mm to 100mm"}
                    </div>
                    
                    <div 
                        class="digital-ruler"
                        onmousedown={start_measuring}
                        onmouseup={end_measuring}
                    >
                        <div class="ruler-markings">
                            { for (0..=10).map(|i| html! {
                                <div class="ruler-mark" style={format!("left: {}%", i * 10)}>
                                    <span class="ruler-label">{format!("{}cm", i)}</span>
                                </div>
                            })}
                        </div>
                        <div class="ruler-line"></div>
                    </div>
                </div>
                
                <div class="manual-input">
                    <p>{"Or enter the pixel measurement manually:"}</p>
                    <div class="input-group">
                        <input 
                            ref={self.measurement_input.clone()}
                            type="number" 
                            placeholder="Pixels for 100mm"
                            min="50"
                            max="1000"
                        />
                        <button onclick={process_input}>{"Process"}</button>
                    </div>
                </div>
                
                { if let Some(pixels) = self.manager.get_state().measured_pixels {
                    html! {
                        <div class="measurement-result">
                            <p>{format!("Measured: {:.1} pixels for 100mm", pixels)}</p>
                            { if let Some(dpi) = self.manager.get_estimated_dpi() {
                                html! { <p>{format!("Estimated DPI: {:.1}", dpi)}</p> }
                            } else { html! {} }}
                        </div>
                    }
                } else { html! {} }}
            </div>
        }
    }

    fn render_validation(&self, ctx: &Context<Self>) -> Html {
        let state = self.manager.get_state();
        let confidence_percent = (state.confidence_score * 100.0) as u32;
        let confidence_class = if state.confidence_score >= 0.8 { "excellent" }
            else if state.confidence_score >= 0.6 { "good" }
            else { "poor" };

        html! {
            <div class="validation-step">
                <div class="step-icon">{"✅"}</div>
                <h2>{"Validation Results"}</h2>
                
                <div class="calibration-summary">
                    { if let Some(pixels) = state.measured_pixels {
                        html! {
                            <div class="measurement-summary">
                                <h3>{"Your Measurement"}</h3>
                                <p>{format!("{:.1} pixels = 100mm", pixels)}</p>
                                { if let Some(dpi) = self.manager.get_estimated_dpi() {
                                    html! { <p>{format!("Display DPI: {:.1}", dpi)}</p> }
                                } else { html! {} }}
                            </div>
                        }
                    } else { html! {} }}
                    
                    <div class={format!("confidence-score {}", confidence_class)}>
                        <h3>{"Confidence Score"}</h3>
                        <div class="score-display">
                            <span class="score-value">{confidence_percent}{"%"}</span>
                            <div class="score-bar">
                                <div 
                                    class="score-fill" 
                                    style={format!("width: {}%", confidence_percent)}
                                ></div>
                            </div>
                        </div>
                        <p class="score-description">
                            { match confidence_class {
                                "excellent" => "Excellent! Your calibration is very accurate.",
                                "good" => "Good calibration. Should work well for most forms.",
                                "poor" => "Low confidence. Consider re-measuring for better accuracy.",
                                _ => "Unknown confidence level."
                            }}
                        </p>
                    </div>
                </div>
                
                <div class="action-buttons">
                    { if state.confidence_score >= 0.5 {
                        html! {
                            <button 
                                class="primary-button"
                                onclick={ctx.link().callback(|_| CalibrationMsg::CompleteCalibration)}
                            >
                                {"Accept Calibration"}
                            </button>
                        }
                    } else { html! {} }}
                    
                    <button 
                        class="secondary-button"
                        onclick={ctx.link().callback(|_| CalibrationMsg::PreviousStep)}
                    >
                        {"Re-measure"}
                    </button>
                </div>
            </div>
        }
    }

    fn render_complete(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="complete-step">
                <div class="step-icon">{"🎉"}</div>
                <h2>{"Calibration Complete!"}</h2>
                <p class="success-message">
                    {"Your device has been successfully calibrated. 
                     Forms will now render with pixel-perfect accuracy."}
                </p>
                
                <div class="next-steps">
                    <h3>{"What's next?"}</h3>
                    <ul>
                        <li>{"✅ Load FL-100 form template"}</li>
                        <li>{"✅ Begin form overlay positioning"}</li>
                        <li>{"✅ Verify print accuracy"}</li>
                    </ul>
                </div>
            </div>
        }
    }

    fn render_error_message(&self) -> Html {
        if let Some(error) = &self.manager.get_state().error_message {
            html! {
                <div class="error-message">
                    <span class="error-icon">{"⚠️"}</span>
                    <span class="error-text">{error}</span>
                </div>
            }
        } else {
            html! {}
        }
    }

    fn render_navigation(&self, ctx: &Context<Self>) -> Html {
        let state = self.manager.get_state();
        
        html! {
            <nav class="calibration-navigation">
                { match state.step {
                    CalibrationStep::Welcome => html! {},
                    CalibrationStep::Complete => html! {},
                    _ => html! {
                        <div class="nav-buttons">
                            <button 
                                class="nav-button back"
                                onclick={ctx.link().callback(|_| CalibrationMsg::PreviousStep)}
                            >
                                {"← Back"}
                            </button>
                            
                            { match state.step {
                                CalibrationStep::Instructions => html! {
                                    <button 
                                        class="nav-button next"
                                        onclick={ctx.link().callback(|_| CalibrationMsg::NextStep)}
                                    >
                                        {"Next →"}
                                    </button>
                                },
                                _ => html! {}
                            }}
                        </div>
                    }
                }}
                
                { if let Some(on_cancel) = &ctx.props().on_cancel {
                    html! {
                        <button 
                            class="cancel-button"
                            onclick={ctx.link().callback(|_| CalibrationMsg::Cancel)}
                        >
                            {"Cancel"}
                        </button>
                    }
                } else { html! {} }}
            </nav>
        }
    }
}