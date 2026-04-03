use eframe::egui;

pub fn render_graph(painter: &mut egui::Painter) {
    painter.text(
        painter.clip_rect().center(),
        egui::Align2::CENTER_CENTER,
        "graph",
        egui::FontId::monospace(20.0),
        egui::Color32::WHITE,
    );
}
