use glium;
use model::{Patchwork, Vertex};

pub struct PatchRenderer {
    pub program: glium::Program,
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub indices: glium::index::NoIndices,
    pub patchwork: Patchwork,
}

impl PatchRenderer {
    pub fn new(patchwork: Patchwork, display: &glium::backend::glutin_backend::GlutinFacade) -> Self {
        let vertex_shader_src = include_str!("shaders/patch_vertex_shader.glsl");
        let fragment_shader_src = include_str!("shaders/patch_fragment_shader.glsl");
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let vertex_buffer = glium::VertexBuffer::new(display, &patchwork.patch.as_trianglelist()).unwrap();

        PatchRenderer {
            program: program,
            vertex_buffer: vertex_buffer,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            patchwork: patchwork
        }
    }

    pub fn get_modelview(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> [[f32; 4]; 4] {
        let screen_width = self.patchwork.width;
        let screen_height = self.patchwork.height;
        let screen_aspect = self.patchwork.aspect_ratio();
        let (window_width, window_height) = display.get_framebuffer_dimensions();
        let window_aspect = window_width as f32 / window_height as f32;
        let ndc_width = 2.0f32 * screen_aspect / window_aspect;
        let ndc_height = 2.0f32;
        [
            [1.0 / screen_width * ndc_width, 0.0, 0.0, 0.0],
            [0.0, -1.0 / screen_height * ndc_height, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-ndc_width / 2.0, ndc_height / 2.0, 0.0, 1.0f32],
        ]
    }
}


