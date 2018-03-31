use Color;
use glium;
use glium::backend::Facade;
use glium::Surface;
use model::{Patch, Vertex};

pub struct PatchRenderer {
    pub program: glium::Program,
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub indices: glium::index::NoIndices,
    modelview: [[f32; 4]; 4],
    color: [f32; 4],
    draw_parameters: glium::DrawParameters<'static>,
}

impl PatchRenderer {
    pub fn new<F: Facade>(display: &F, modelview: [[f32; 4]; 4]) -> Self {
        let vertex_shader_src = include_str!("shaders/patch_vertex_shader.glsl");
        let fragment_shader_src = include_str!("shaders/patch_fragment_shader.glsl");
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
        PatchRenderer {
            program,
            vertex_buffer: glium::VertexBuffer::empty_dynamic(display, Patch::vertex_count()).unwrap(),
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            modelview,
            color: Color::white().to_gl(),
            draw_parameters: glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    pub fn set_modelview(&mut self, modelview: [[f32; 4]; 4]) {
        self.modelview = modelview;
    }

    pub fn set_patch(&mut self, patch: &Patch) {
        self.vertex_buffer.write(&patch.as_trianglelist());
        self.color = patch.color.to_gl();
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        let uniforms = uniform! { modelview: self.modelview, uniformcolor: self.color };
        frame.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms, &self.draw_parameters).unwrap();
    }
}


