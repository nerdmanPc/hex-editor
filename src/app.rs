use {
    emath::{
        Pos2,
        Rect,
        RectTransform,
    },
    egui::{
        Ui,
        Sense,
        Shape,
        Stroke,
        Color32,
        Context,
        SidePanel,
        Response,
        CentralPanel,
    },
    eframe::{
        App,
        Frame,
    },
};

mod grid; use grid::Grid;

pub struct Editor {
    grid: Grid,
    color: Color32,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            grid: Grid::make_hex((0, 0), 8),
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
        
        let to_screen = canvas_to_ui(&response.rect);
        let from_screen = to_screen.inverse();

        if let Some(screen_pos) = response.interact_pointer_pos() {
            let canvas_pos: [f32; 2]  = (from_screen * screen_pos).into();
            let cell = self.grid.sample_cell(canvas_pos);
            self.grid.paint_cell(cell, self.color);
            response.mark_changed();
        }

        let shapes = self.grid.cells().map( |(&hex, &color)| {
                let points = self.grid.polygon_corners(hex);
                let points = points.map( |[point_x, point_y]| {
                    to_screen * Pos2::new(point_x, point_y)
                }).collect();
                Shape::convex_polygon(points, color, Stroke{color: color, width: 1.0})
            }
        );
        painter.extend(shapes);
        response
    }
}


fn canvas_to_ui(ui_rect: &Rect) -> RectTransform {
    RectTransform::from_to(
        canvas_rect(ui_rect),
        *ui_rect,
    )
}

fn _ui_to_canvas(ui_rect: &Rect) -> RectTransform {
    RectTransform::from_to(
        *ui_rect,
        canvas_rect(ui_rect),
    )
}

fn canvas_rect(ui_rect: &Rect) -> Rect {
    Rect::from_center_size(
        Pos2::ZERO, 
        ui_rect.square_proportions() * 2_f32
    )
}