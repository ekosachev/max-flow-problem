use eframe::{App, egui};

#[derive(Default)]
pub struct MaxFlowProblemApp;

impl App for MaxFlowProblemApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::bottom("parameters").show_inside(ui, |ui| self.parameters_view(ui));
        egui::CentralPanel::default().show_inside(ui, |ui| self.graph_view(ui));
    }
}

impl MaxFlowProblemApp {
    fn graph_view(&mut self, ui: &mut egui::Ui) {
        ui.label("Graph view");
    }

    fn parameters_view(&mut self, ui: &mut egui::Ui) {
        ui.label("Parameters view");
    }
}
