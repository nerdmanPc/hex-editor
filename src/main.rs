use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1360.0, 768.0)),
        ..Default::default()
    };
    eframe::run_native(
        "HexEditor",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<EditorInstance>::default()
        }),
    )
}

struct EditorInstance {
    lines: Vec<Vec<egui::Pos2>>,
    stroke: egui::Stroke,
}

impl Default for EditorInstance {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(25, 200, 100)),
        }
    }
}

impl eframe::App for EditorInstance {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let toolbox = egui::SidePanel::left("toolbox");
        toolbox.show(ctx, |ui| {
            self.draw_toolbox(ui)
        });
        let palette = egui::SidePanel::right("palette");
        palette.show(ctx, |ui| {
            self.draw_palette(ui)
        });
        let canvas = egui::CentralPanel::default();
        canvas.show(ctx, |ui| {
            self.draw_canvas(ui)
        });
    }
}

impl EditorInstance{

    fn draw_toolbox(&mut self, ui: &mut egui::Ui) {
        ui.label("Toolbox");
    }
    fn draw_palette(&mut self, ui: &mut egui::Ui) {
        ui.label("Palette");
    }
    fn draw_canvas(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.label("Canvas");
        let canvas_size = ui.available_size_before_wrap();
        let (mut response, painter) = ui.allocate_painter(canvas_size, egui::Sense::drag());
        let to_screen = emath::RectTransform::from_to(
            egui::Rect::from_min_size(
                egui::Pos2::ZERO, 
                response.rect.square_proportions()
            ),
            response.rect
        );
        let from_screen = to_screen.inverse();
        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }
        let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
            response.mark_changed();
        }
        let shapes = self.lines.iter()
            .filter(|line| line.len() >= 2)
            .map(|line| {
                let points = line.iter().map(|point| to_screen * (*point)).collect();
                egui::Shape::line(points, self.stroke)
            });
        painter.extend(shapes);
        response
    }
}
