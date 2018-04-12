use Color;
use glium;
use glium::backend::Facade;
use glium::Surface;
use super::model::{Patch, Vertex};

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
        let vertex_buffer = glium::VertexBuffer::empty_dynamic(
            display,
            SURFACE_TRIANGLELIST_VERTEX_COUNT).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let color = Color::white().to_gl();
        let draw_parameters = glium::DrawParameters {
            depth: glium::Depth { test: glium::DepthTest::IfLess, write: true, ..Default::default() },
            smooth: Some(glium::Smooth::Nicest),
            ..Default::default()
        };
        PatchRenderer { program, vertex_buffer, indices, modelview, color, draw_parameters }
    }

    pub fn set_modelview(&mut self, modelview: [[f32; 4]; 4]) {
        self.modelview = modelview;
    }

    pub fn set_patch(&mut self, patch: &Patch) {
        self.vertex_buffer.write(&patch.surface_trianglelist());
        self.color = patch.color.to_gl();
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        if self.color[3] > 0.0 {
            let uniforms = uniform! { modelview: self.modelview, uniformcolor: self.color };
            frame.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms, &self.draw_parameters).unwrap();
        }
    }
}

const SURFACE_TRIANGLELIST_VERTEX_COUNT: usize = 6;

impl Patch {
    fn surface_trianglelist(&self) -> Vec<Vertex> {
        let (left, right, bottom, top, far, _) = self.cage.limits();
        let lt_vertex = Vertex { position: [left, top, far] };
        let rt_vertex = Vertex { position: [right, top, far] };
        let rb_vertex = Vertex { position: [right, bottom, far] };
        let lb_vertex = Vertex { position: [left, bottom, far] };
        vec![lt_vertex, rt_vertex, lb_vertex, lb_vertex, rt_vertex, rb_vertex]
    }
}
