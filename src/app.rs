use eframe::{App, egui};

use crate::ui::graph_view::GraphWindow;

#[derive(Default)]
pub struct MaxFlowProblemApp {
    graph_view: GraphWindow,
}

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

        self.graph_view.render_graph(
            &mut painter,
            vec![vec![0], vec![1, 2, 3], vec![4, 5], vec![6]],
            vec![
                [0, 2, 1],
                [0, 1, 1],
                [2, 3, 1],
                [2, 6, 1],
                [1, 4, 1],
                [3, 5, 1],
                [4, 5, 1],
                [5, 6, 1],
            ],
        );
    }

    fn parameters_view(&mut self, ui: &mut egui::Ui) {
        ui.label("Parameters view");
    }
}
