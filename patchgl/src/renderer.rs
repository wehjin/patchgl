use model::{Vertex, Patch};
use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::uniforms::Uniforms;
use glium::Surface;

pub struct PatchRenderer {
    pub program: glium::Program,
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub indices: glium::index::NoIndices,
    modelview: [[f32; 4]; 4],
}

impl PatchRenderer {
    pub fn new(patch: &Patch, display: &GlutinFacade, modelview: [[f32; 4]; 4]) -> Self {
        let vertex_shader_src = include_str!("shaders/patch_vertex_shader.glsl");
        let fragment_shader_src = include_str!("shaders/patch_fragment_shader.glsl");
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let vertex_buffer = glium::VertexBuffer::new(display, &patch.as_trianglelist()).unwrap();

        PatchRenderer {
            program: program,
            vertex_buffer: vertex_buffer,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            modelview: modelview,
        }
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        let uniforms = uniform! { modelview: self.modelview };
        frame.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms, &Default::default()).unwrap();
    }
}


