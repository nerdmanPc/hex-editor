use eframe::egui;

mod model;

use emath::Pos2;
use model::{Grid, Hex, LayoutTool};

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
    map: Grid,
    color: egui::Color32,
}

impl Default for EditorInstance {
    fn default() -> Self {
        Self {
            map: Grid::make_hex(Hex::new(0, 0), 8),
            color: egui::Color32::from_rgb(25, 200, 100),
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
        

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            let Pos2 { x:canvas_x, y:canvas_y} = canvas_pos;
            let cell = self.map.sample_cell(canvas_x as f64, canvas_y as f64);
            //print!("pointer_pos: {:?}\ncanvas_pos: {:?}\ncell: {:?}\n", pointer_pos, canvas_pos, cell);
            self.map.paint_cell(cell, self.color);
            response.mark_changed();
        }

        let shapes = self.map.cells().map( |(&hex, &color)| {
                let points = LayoutTool::polygon_corners(self.map.layout(), hex);
                let points = points.iter().map( |point| {
                    let point = Pos2::new(point.x as f32, point.y as f32);
                    to_screen * point
                }).collect();
                egui::Shape::convex_polygon(points, color, egui::Stroke::NONE)
            });
        painter.extend(shapes);
        response
    }
}
