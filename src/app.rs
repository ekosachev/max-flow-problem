use eframe::{App, egui};

use crate::ui::graph_view::render_graph;

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
        let (_response, mut painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::click());

        render_graph(
            &mut painter,
            vec![vec![0], vec![1, 2, 3], vec![4, 5], vec![6]],
        );
    }

    fn parameters_view(&mut self, ui: &mut egui::Ui) {
        ui.label("Parameters view");
    }
}
