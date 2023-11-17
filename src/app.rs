
use emath::{
    Pos2,
    Rect,
    RectTransform,
};

use egui::{
    Ui,
    Sense,
    Shape,
    Stroke,
    Color32,
    Context,
    SidePanel,
    Response,
    CentralPanel,
};

use eframe::{
    App,
    Frame,
};

use crate::model::{Grid, point};

pub struct Editor {
    map: Grid,
    color: Color32,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            map: Grid::make_hex((0, 0), 8),
            color: Color32::from_rgb(25, 200, 100),
        }
    }
}

impl App for Editor {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let toolbox = SidePanel::left("toolbox");
        toolbox.show(ctx, |ui| {
            self.draw_toolbox(ui)
        });
        let palette = SidePanel::right("palette");
        palette.show(ctx, |ui| {
            self.draw_palette(ui)
        });
        let canvas = CentralPanel::default();
        canvas.show(ctx, |ui| {
            self.draw_canvas(ui)
        });
    }
}

impl Editor {

    fn draw_toolbox(&mut self, ui: &mut Ui) {
        ui.label("Toolbox");
    }
    fn draw_palette(&mut self, ui: &mut Ui) {
        ui.label("Palette");
    }
    fn draw_canvas(&mut self, ui: &mut Ui) -> Response {
        ui.label("Canvas");
        let canvas_size = ui.available_size_before_wrap();
        let (mut response, painter) = ui.allocate_painter(canvas_size, Sense::drag());
        
        let to_screen = RectTransform::from_to(
            Rect::from_min_size(
                Pos2::ZERO, 
                response.rect.square_proportions()
            ),
            response.rect
        );
        let from_screen = to_screen.inverse();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos: [f32; 2]  = (from_screen * pointer_pos).into();
            let cell = self.map.sample_cell(canvas_pos);
            self.map.paint_cell(cell, self.color);
            response.mark_changed();
        }

        let shapes = self.map.cells().map( |(&hex, &color)| {
                let points = self.map.polygon_corners(hex);
                let points = points.map( |[point_x, point_y]| {
                    to_screen * Pos2::new(point_x, point_y)
                }).collect();
                Shape::convex_polygon(points, color, Stroke::NONE)
            });
        painter.extend(shapes);
        response
    }
}
