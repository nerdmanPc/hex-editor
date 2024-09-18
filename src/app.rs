use {
    eframe::{
        egui_glow::{self, Painter}, glow::{self, HasContext}, App, CreationContext, Frame
    }, egui::{
        mutex::Mutex, CentralPanel, Color32, Context, PaintCallback, RequestRepaintInfo, Response, Shape, SidePanel, Stroke, Ui 
    }, emath::{
        Pos2, Rect, RectTransform, Vec2
    }, std::{sync::Arc, u32}
};

mod grid; use grid::Grid;

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
            let ctrl_pressed = ctx.input(|input|{
                input.key_down( egui::Key::Space)
            });
            self.draw_viewport(ui, ctrl_pressed)
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
            grid: Grid::make_hex((0, 0), 8),
            color: Color32::from_rgb(25, 200, 100),
            renderer: Arc::new(Mutex::new(unsafe{Renderer::new(&gl)})),
        }
    }
    fn draw_toolbox(&mut self, ui: &mut Ui) {
        ui.label("Toolbox");
    }
    fn draw_palette(&mut self, ui: &mut Ui) {
        ui.label("Palette");
    }
    fn draw_viewport(&mut self, ui: &mut Ui, ctrl_pressed: bool) -> Response {

        ui.label("Viewport");
        let viewport_size = ui.available_size_before_wrap();
        let (mut response, painter) = ui.allocate_painter(viewport_size, egui::Sense::click_and_drag());
 
        let to_screen = canvas_to_ui(&response.rect);
        let from_screen = to_screen.inverse();

        if let (Some(screen_pos), false) = (response.interact_pointer_pos(), ctrl_pressed) {
            let canvas_pos: [f32; 2]  = (from_screen * screen_pos).into();
            let cell = self.grid.sample_cell(canvas_pos);
            self.grid.paint_cell(cell, self.color);
            response.mark_changed();
        } else {
            self.renderer.lock().rotate(response.drag_delta() * 0.0);
        }

        let renderer_clone = self.renderer.clone();

        let draw_contents = move |_info: RequestRepaintInfo, painter: Painter| {
            unsafe {renderer_clone.lock().draw(painter.gl());}
        };

        let draw_contents = PaintCallback{
            rect: response.rect,
            callback: Arc::new(draw_contents)
        };
        //draw shapes
        /*let shapes = self.grid.cells().map( |(&hex, &color)| {
            let points = self.grid.polygon_corners(hex);
            let points = points.map( |[point_x, point_y]| {
                to_screen * Pos2::new(point_x, point_y)
            }).collect();
            Shape::convex_polygon(points, color, Stroke{color: color, width: 1.0})
        });*/
        //painter.extend(shapes);
        painter.add(draw_contents);
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

struct Renderer {
    program: glow::Program,
    vertex_array: glow::VertexArray,
    angle: f32,
}

impl Renderer {
    pub unsafe fn new(gl: &glow::Context) -> Self {
        use glow::HasContext;
        let shader_version = egui_glow::ShaderVersion::get(gl);
        let program = gl.create_program().expect("Failed to create shader program!");

        let (vertex_shader_source, fragment_shader_source) = (
            include_str!("shaders/vertex.glsl"),
            include_str!("shaders/fragment.glsl")
        );
        
        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let compile_shaders = |(shader_type, shader_source): &(u32, &str)| {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Failed to create shader handle!");
            gl.shader_source(
                shader,
                &format!(
                    "{}\n{}",
                    shader_version.version_declaration(),
                    shader_source
                ),
            );
            gl.compile_shader(shader);
            assert!(
                gl.get_shader_compile_status(shader),
                "Failed to compile shader module {shader_type}!: {}",
                gl.get_shader_info_log(shader)
            );
            gl.attach_shader(program, shader);
            shader
        };

        let shaders: Vec<_> = shader_sources
            .iter()
            .map(compile_shaders)
            .collect();

        gl.link_program(program);
        assert!(
            gl.get_program_link_status(program),
            "{}", gl.get_program_info_log(program)
        );
        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }
        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array!");
        Self { 
            program, 
            vertex_array,
            angle: 1.0
        }
    }

    pub unsafe fn draw(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        gl.use_program(Some((self.program)));
        gl.uniform_1_f32(
            gl.get_uniform_location(self.program, "u_angle").as_ref(),
            self.angle
        );
        gl.bind_vertex_array(Some(self.vertex_array));
        gl.draw_arrays(glow::TRIANGLES, 0, 3);
    }

    pub fn rotate(&mut self, angle: Vec2) {
        self.angle += angle[0];
    }

    pub unsafe fn clear_resources(&self, gl: &glow::Context) {
        gl.delete_program(self.program);
        gl.delete_vertex_array(self.vertex_array);
    }
}