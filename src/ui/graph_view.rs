use eframe::egui;

const LAYER_MARGIN: f32 = 5.0;

pub fn render_graph(painter: &mut egui::Painter, vertices: Vec<Vec<usize>>) {
    let clip_rect = painter.clip_rect();
    let layer_rects = calculate_layer_rects(&clip_rect, vertices.len());

    layer_rects.iter().enumerate().for_each(|(i, r)| {
        render_layer_box(painter, *r);
        render_layer_label(painter, *r, i);
        render_layer_vertices(painter, *r, &vertices[i]);
    });
}

fn calculate_layer_rects(painter_rect: &egui::Rect, layers: usize) -> Vec<egui::Rect> {
    let layout_origin = painter_rect.left_top();
    let layer_area_width = painter_rect.width() / (layers as f32);

    let layer_width = layer_area_width - 2.0 * LAYER_MARGIN;
    let layer_height = painter_rect.height() - 2.0 * LAYER_MARGIN;
    let layer_size = egui::Vec2::new(layer_width, layer_height);

    (0..layers)
        .map(|i| {
            let layer_origin = egui::Pos2::new(
                layout_origin.x + layer_area_width * (i as f32) + LAYER_MARGIN,
                layout_origin.y + LAYER_MARGIN,
            );
            egui::Rect::from_two_pos(layer_origin, layer_origin + layer_size)
        })
        .collect()
}

fn render_layer_box(painter: &mut egui::Painter, rect: egui::Rect) {
    painter.rect_stroke(
        rect,
        5.0,
        egui::Stroke::new(2.0, egui::Color32::DARK_GRAY),
        egui::StrokeKind::Middle,
    );
}

fn render_layer_label(painter: &mut egui::Painter, rect: egui::Rect, idx: usize) {
    painter.text(
        rect.center_top() + egui::Vec2::new(0.0, LAYER_MARGIN),
        egui::Align2::CENTER_TOP,
        format!("Layer {}", idx),
        egui::FontId::monospace(15.0),
        egui::Color32::DARK_GRAY,
    );
}

fn render_layer_vertices(painter: &mut egui::Painter, rect: egui::Rect, vertices: &[usize]) {
    let rect_top = rect.center_top();
    let vertex_spacing = rect.height() / (vertices.len() as f32 + 1.0);

    vertices.iter().enumerate().for_each(|(i, v)| {
        let position = rect_top + egui::Vec2::new(0.0, vertex_spacing * (i as f32 + 1.0));
        render_vertex(painter, position, *v);
    });
}

fn render_vertex(painter: &mut egui::Painter, position: egui::Pos2, idx: usize) {
    painter.circle_filled(position, 20.0, egui::Color32::DARK_GRAY);

    painter.text(
        position,
        egui::Align2::CENTER_CENTER,
        idx.to_string(),
        egui::FontId::monospace(20.0),
        egui::Color32::WHITE,
    );
}
