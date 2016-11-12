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

        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let vertex_buffer = glium::VertexBuffer::new(&display, &patch.as_trianglelist()).unwrap();
        PatchRenderer {
            display: display,
            program: program,
            vertex_buffer: vertex_buffer,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
        }
    }
}


