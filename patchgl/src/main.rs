#[macro_use] extern crate glium;
extern crate xml;
extern crate cage;
extern crate patchgllib;

use patchgllib::model::Patchwork;

struct PatchRenderer {
    display: glium::backend::glutin_backend::GlutinFacade,
    program: glium::Program,
}

impl PatchRenderer {
    fn new() -> Self {
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
        PatchRenderer { display: display, program: program }
    }
}


fn main() {
    let xml = r#"
        <screen id="1" size="320x480">
            <patch id="2" bounds="0.25, 1, 0.0, -0.25, -0.5"/>
        </screen>
        "#;
    let patchwork = Patchwork::from_xml(xml);
    let patch = patchwork.patch;

    let patch_renderer = PatchRenderer::new();
    let vertex_buffer = glium::VertexBuffer::new(&patch_renderer.display, &patch.as_trianglelist()).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    loop {
        let mut target = patch_renderer.display.draw();
        use glium::{Surface};
        target.clear_color(0.70, 0.80, 0.90, 1.0);
        target.draw(&vertex_buffer, &indices, &patch_renderer.program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in patch_renderer.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn patch_renders() {}
}
