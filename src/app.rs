use {
    eframe::{
        egui_glow::{self, Painter}, glow::{self}, App, CreationContext, Frame
    }, egui::{
        mutex::Mutex, CentralPanel, Color32, Context, PaintCallback, Response, Ui 
    }, emath::{
        Pos2, Rect, RectTransform
    }
};
use std::sync::Arc;

mod grid; use grid::Grid;
mod renderer; use renderer::Renderer;

struct ArcMutex<T>(pub Arc<Mutex<T>>);
impl<T> ArcMutex<T> {
    pub fn new(item: T) -> Self {
        Self(Arc::new(Mutex::new(item)))
    }
    pub fn payload(&self) -> &Arc<Mutex<T>> {
        let &ArcMutex(payload) = &self;
        payload
    }
}

pub struct Editor {
    grid: Grid,
    color: Color32,
    renderer: Arc<Mutex<Renderer>>,
}

impl App for Editor {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        /*let toolbox = SidePanel::left("toolbox");
        toolbox.show(ctx, |ui| {
            self.draw_toolbox(ui)
        });
        let palette = SidePanel::right("palette");
        palette.show(ctx, |ui| {
            self.draw_palette(ui)
        });*/
        let canvas = CentralPanel::default();
        canvas.show(ctx, |ui| {
            self.draw_viewport(ui)
        });
    }
    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            //This function is only called when no resource is needed
            unsafe {self.renderer.lock().clear_resources(gl)}
        }
    }
}

impl Editor {

    pub fn new<'a>(cc: &'a CreationContext) -> Self {
        let gl = cc.gl.as_ref().expect("Aplication initialization failed!");
        //Memory and resource allocation issues likely come from here
        Self {
            grid: Grid::default(),
            color: Color32::from_rgb(25, 200, 100),
            renderer: Arc::new(Mutex::new(unsafe{Renderer::new(&gl)})),
        }
    }
    fn _draw_toolbox(&mut self, ui: &mut Ui) {
        ui.label("Toolbox");
    }
    fn _draw_palette(&mut self, ui: &mut Ui) {
        ui.label("Palette");
    }
    fn draw_viewport(&mut self, ui: &mut Ui) -> Response {

        ui.label("Viewport");
        let viewport_size = ui.available_size_before_wrap();
        let (mut response, painter) = ui.allocate_painter(viewport_size, egui::Sense::click_and_drag());
        //let (_rect, mut response) = ui.allocate_exact_size(viewport_size, egui::Sense::click_and_drag());
 
        let frustum_to_ui = canvas_to_ui(&response.rect);
        let ui_to_frustum = frustum_to_ui.inverse();

        let space_pressed = ui.ctx().input(|input|{
            input.key_down( egui::Key::Space)
        });
        
        if let (Some(screen_pos), false) = (response.interact_pointer_pos(), space_pressed) {
            let canvas_pos: [f32; 2]  = (ui_to_frustum * screen_pos).into();
            let cell = self.grid.sample_cell(canvas_pos);
            self.grid.paint_cell(cell, self.color);

            let mesh = self.grid.build_mesh();
            let renderer_handle = self.renderer.clone();
            let update_mesh_fn = move |_info, painter: &Painter| {
                unsafe {renderer_handle.lock().update_mesh(painter.gl(), &mesh);}
            };
            let update_mesh_fn = egui_glow::CallbackFn::new(update_mesh_fn);
            let update_mesh_fn = PaintCallback{
                rect: response.rect,
                callback: Arc::new(update_mesh_fn)
            };
            painter.add(update_mesh_fn);

            response.mark_changed();
        } /*else*/ {
            self.renderer.lock().rotate(response.drag_delta() * 0.01);
            response.mark_changed();
        }


        let renderer_handle = self.renderer.clone();
        let draw_contents_fn = move |_info, painter: &Painter| {
            unsafe {renderer_handle.lock().draw(painter.gl());}
        };
        let draw_contents_fn = egui_glow::CallbackFn::new(draw_contents_fn);
        let draw_contents_cb = PaintCallback{
            rect: response.rect,
            callback: Arc::new(draw_contents_fn)
        };
        painter.add(draw_contents_cb);
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
