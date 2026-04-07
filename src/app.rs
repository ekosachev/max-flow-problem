use std::collections::HashSet;

use eframe::{App, egui};

use crate::ui::graph_view::GraphWindow;

#[derive(Default)]
pub struct MaxFlowProblemApp {
    global_state: GlobalState,
    graph_view: GraphWindow,
    action: Option<Action>,
}

struct GlobalState {
    constraint_matrix: Vec<Vec<usize>>,
}

pub enum Action {
    AddNodeToLayer(usize),
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
            layers.push(frontier.drain().collect::<Vec<usize>>());

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

        layers.iter_mut().for_each(|l| l.sort());
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

    pub fn add_node_to_layer(&mut self, layer_id: usize) {
        if layer_id == 0 {
            return;
        }

        let layers = self.graph_layers();

        let parent = layers[layer_id - 1][0];

        self.constraint_matrix
            .iter_mut()
            .enumerate()
            .for_each(|(u, outbound_capacities)| {
                outbound_capacities.push(if u != parent { 0 } else { 1 })
            });

        self.constraint_matrix
            .push(vec![0; self.constraint_matrix.len() + 1]);
    }
}

impl App for MaxFlowProblemApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::bottom("parameters").show_inside(ui, |ui| self.parameters_view(ui));
        egui::CentralPanel::default().show_inside(ui, |ui| self.graph_view(ui));
    }

    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(action) = &self.action {
            match action {
                Action::AddNodeToLayer(layer_id) => self.global_state.add_node_to_layer(*layer_id),
            }
        }
    }
}

impl MaxFlowProblemApp {
    fn graph_view(&mut self, ui: &mut egui::Ui) {
        let graph_size = ui.available_size_before_wrap();
        self.action = self.graph_view.render_graph(
            ui,
            graph_size,
            self.global_state.graph_layers(),
            self.global_state.graph_edges(),
        );
    }

    fn parameters_view(&mut self, ui: &mut egui::Ui) {
        ui.label("Parameters view");
    }
}
