use glium;
use model::{Patch, Vertex};

pub struct PatchRenderer {
    pub display: glium::backend::glutin_backend::GlutinFacade,
    pub program: glium::Program,
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub indices: glium::index::NoIndices,
}

impl PatchRenderer {
    pub fn new(patch: Patch) -> Self {
        use glium::{DisplayBuild};
        let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

        let vertex_shader_src = include_str!("shaders/patch_vertex_shader.glsl");
        let fragment_shader_src = include_str!("shaders/patch_fragment_shader.glsl");

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let vertex_buffer = glium::VertexBuffer::new(&display, &patch.as_trianglelist()).unwrap();

        PatchRenderer {
            display: display,
            program: program,
            vertex_buffer: vertex_buffer,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        }
    }
}


