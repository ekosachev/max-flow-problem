use eframe::egui;

const LAYER_MARGIN: f32 = 5.0;

pub fn render_graph(
    painter: &mut egui::Painter,
    vertices: Vec<Vec<usize>>,
    edges: Vec<[usize; 3]>,
) {
    let clip_rect = painter.clip_rect();
    let layer_rects = calculate_layer_rects(&clip_rect, vertices.len());
    let vertex_positions: Vec<egui::Pos2> = layer_rects
        .iter()
        .enumerate()
        .flat_map(|(i, r)| calculate_vertex_positions(*r, &vertices[i]))
        .collect();

    layer_rects.iter().enumerate().for_each(|(i, r)| {
        render_layer_box(painter, *r);
        render_layer_label(painter, *r, i);
        render_edges(painter, &vertices[i], &vertex_positions, &edges);
        render_layer_vertices(painter, &vertex_positions, &vertices[i]);
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

fn calculate_vertex_positions(rect: egui::Rect, vertices: &[usize]) -> Vec<egui::Pos2> {
    let rect_top = rect.center_top();
    let vertex_spacing = rect.height() / (vertices.len() as f32 + 1.0);

    vertices
        .iter()
        .enumerate()
        .map(|(i, _v)| rect_top + egui::Vec2::new(0.0, vertex_spacing * (i as f32 + 1.0)))
        .collect()
}

fn render_layer_vertices(
    painter: &mut egui::Painter,
    positions: &[egui::Pos2],
    vertices: &[usize],
) {
    vertices
        .iter()
        .for_each(|v| render_vertex(painter, positions[*v], *v));
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

fn render_edges(
    painter: &mut egui::Painter,
    layer_vertices: &[usize],
    vertex_positions: &[egui::Pos2],
    edges: &[[usize; 3]],
) {
    edges
        .iter()
        .filter(|e| layer_vertices.contains(&e[0]))
        .for_each(|e| {
            render_edge(painter, vertex_positions[e[0]], vertex_positions[e[1]]);
        });
}

fn render_edge(painter: &mut egui::Painter, start_pos: egui::Pos2, end_pos: egui::Pos2) {
    painter.line_segment(
        [start_pos, end_pos],
        egui::Stroke::new(2.0, egui::Color32::DARK_GRAY),
    );
}
