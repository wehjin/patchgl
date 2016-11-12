#[macro_use] extern crate glium;
extern crate xml;
extern crate cage;
extern crate patchgllib;

use cage::{Cage};
use patchgllib::parser;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

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


#[derive(Default)]
struct Patchwork {
    patch: Patch
}

impl Patchwork {
    fn from_xml(xml_string: &str) -> Self {
        let mut patchwork = Patchwork { patch: Default::default() };
        use xml::reader::{EventReader, XmlEvent};
        let parser = EventReader::from_str(xml_string);
        for event in parser {
            match event {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    if name.local_name == "patch" {
                        patchwork.patch = Patch::from_attributes(&attributes);
                    }
                }
                Err(event) => {
                    println!("Error: {}", event);
                    break;
                }
                _ => {}
            }
        }
        patchwork
    }
}

#[derive(Default)]
struct Patch {
    cage: Cage
}

impl Patch {
    fn from_attributes(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Self {
        let mut patch = Patch { ..Default::default() };
        for attribute in attributes {
            if attribute.name.local_name == "bounds" {
                let cage = parser::cage_from_string(&attribute.value);
                patch.cage = cage;
            }
        }
        patch
    }
    fn as_trianglelist(&self) -> Vec<Vertex> {
        let (left, right, bottom, top, _, _) = self.cage.limits();
        let lt_vertex = Vertex { position: [left, top] };
        let rt_vertex = Vertex { position: [right, top] };
        let rb_vertex = Vertex { position: [right, bottom] };
        let lb_vertex = Vertex { position: [left, bottom] };
        vec![lt_vertex, rt_vertex, lb_vertex, lb_vertex, rt_vertex, rb_vertex]
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
