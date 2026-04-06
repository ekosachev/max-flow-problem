use std::collections::HashSet;

use eframe::{App, egui};

use crate::ui::graph_view::GraphWindow;

#[derive(Default)]
pub struct MaxFlowProblemApp {
    global_state: GlobalState,
    graph_view: GraphWindow,
}

struct GlobalState {
    constraint_matrix: Vec<Vec<usize>>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            constraint_matrix: vec![
                vec![0, 1, 4, 0],
                vec![0, 0, 0, 3],
                vec![0, 0, 0, 3],
                vec![0, 0, 0, 0],
            ],
        }
    }
}

impl GlobalState {
    pub fn graph_layers(&self) -> Vec<Vec<usize>> {
        let mut layers: Vec<Vec<usize>> = vec![];
        let mut frontier = HashSet::<usize>::new();
        frontier.insert(0);

        loop {
            layers.push(frontier.drain().collect());

            layers.last().unwrap().iter().for_each(|u| {
                self.constraint_matrix[*u]
                    .iter()
                    .enumerate()
                    .filter(|(_v, c)| **c > 0)
                    .for_each(|(v, _c)| {
                        frontier.insert(v);
                    })
            });

            if frontier.is_empty() {
                break;
            }
        }

        layers
    }

    pub fn graph_edges(&self) -> Vec<[usize; 3]> {
        self.constraint_matrix
            .iter()
            .enumerate()
            .flat_map(|(u, row)| {
                row.iter()
                    .enumerate()
                    .skip(u)
                    .filter_map(|(v, c)| if *c == 0 { None } else { Some([u, v, *c]) })
                    .collect::<Vec<[usize; 3]>>()
            })
            .collect()
    }
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
            self.global_state.graph_layers(),
            self.global_state.graph_edges(),
        );
    }

    fn parameters_view(&mut self, ui: &mut egui::Ui) {
        ui.label("Parameters view");
    }
}
