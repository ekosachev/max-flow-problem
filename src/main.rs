use crate::app::MaxFlowProblemApp;

mod app;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Max Flow Problem",
        native_options,
        Box::new(|_cc| Ok(Box::new(MaxFlowProblemApp::default()))),
    );
}
