
use eframe::glow;
use eframe::egui_glow;
use glow::HasContext;
use emath::Vec2;
pub struct Renderer {
    program: glow::Program,
    vertex_array: glow::VertexArray,
    vertex_buffer: glow::Buffer,
    vertex_count: i32,
    angle: f32,
}
impl Renderer {
    pub unsafe fn new(gl: &glow::Context) -> Self {

        let shader_version = egui_glow::ShaderVersion::get(gl);
        let program = gl.create_program().expect("Failed to create shader program!");

        let (vertex_shader_source, fragment_shader_source) = (
            include_str!("../shaders/vertex.glsl"),
            include_str!("../shaders/fragment.glsl")
        );
        
        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let compile_shaders = |(shader_type, shader_source): &(u32, &str)| {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Failed to create shader!");
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

        let (vertex_buffer, vertex_array) = create_vertex_array(gl);

        Self { 
            program, 
            vertex_array,
            vertex_buffer,
            vertex_count:0,
            angle: 0.0
        }
    }

    pub unsafe fn draw(&self, gl: &glow::Context) {
        gl.use_program(Some(self.program));
        gl.uniform_1_f32(
            gl.get_uniform_location(self.program, "u_angle").as_ref(),
            self.angle
        );
        gl.bind_vertex_array(Some(self.vertex_array));
        //gl.bind_vertex_buffer(0, Some(self.vertex_buffer), 0, 0);
        gl.draw_arrays(glow::TRIANGLE_STRIP, 0, self.vertex_count);
    }

    pub unsafe fn update_mesh(&mut self, gl: &glow::Context, mesh: &[[f32;2]]) {
        let ptr = mesh.as_ptr() as *const u8;
        let len = mesh.len() * 2 * core::mem::size_of::<f32>();
        let mesh_u8: &[u8] = core::slice::from_raw_parts(ptr, len);

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, mesh_u8, glow::DYNAMIC_DRAW);

        self.vertex_count = mesh.len() as i32;
    }

    pub fn rotate(&mut self, angle: Vec2) {
        self.angle += angle[0];
    }

    pub unsafe fn clear_resources(&self, gl: &glow::Context) {
        gl.delete_program(self.program);
        gl.delete_vertex_array(self.vertex_array);
    }
}

unsafe fn create_vertex_array(gl: &glow::Context, ) -> (glow::NativeBuffer, glow::NativeVertexArray) {
    let vbo = gl.create_buffer().expect("Failed to create VBO!");
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &[], glow::DYNAMIC_DRAW);

    let vao = gl.create_vertex_array().expect("Failed to create VAO!");
    gl.bind_vertex_array(Some((vao)));
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);
    (vbo, vao)
}