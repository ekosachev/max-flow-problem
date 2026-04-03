use eframe::egui;

const LAYER_MARGIN: f32 = 5.0;

pub fn render_graph(painter: &mut egui::Painter) {
    let clip_rect = painter.clip_rect();
    let layer_rects = calculate_layer_rects(&clip_rect, 5);

    layer_rects.iter().enumerate().for_each(|(i, r)| {
        render_layer_box(painter, *r);
        render_layer_label(painter, *r, i);
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
