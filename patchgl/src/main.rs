#[macro_use] extern crate glium;
extern crate xml;

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

struct Patch {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Patch {
    fn new() -> Self {
        let mut patch = Patch { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
        let patch_xml = r#"
        <screen id="1" size="320x480">
            <patch id="2" bounds="0.25, 1, -0.25, -0.5"/>
        </screen>
        "#;
        use xml::reader::{EventReader, XmlEvent};
        let parser = EventReader::from_str(patch_xml);
        for event in parser {
            match event {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    println!("{}", name);
                    if name.local_name == "patch" {
                        for attribute in attributes {
                            println!("{}={}", attribute.name, attribute.value);
                            if attribute.name.local_name == "bounds" {
                                let values: Vec<&str> = attribute.value.split(',').collect();
                                patch.top = values[0].trim().parse::<f32>().unwrap();
                                patch.right = values[1].trim().parse::<f32>().unwrap();
                                patch.bottom = values[2].trim().parse::<f32>().unwrap();
                                patch.left = values[3].trim().parse::<f32>().unwrap();
                            }
                        }
                    }
                }
                Err(event) => {
                    println!("Error: {}", event);
                    break;
                }
                _ => {}
            }
        }
        patch
    }
    fn as_vertices(&self) -> Vec<Vertex> {
        let lt_vertex = Vertex { position: [self.left, self.top] };
        let rt_vertex = Vertex { position: [self.right, self.top] };
        let rb_vertex = Vertex { position: [self.right, self.bottom] };
        let lb_vertex = Vertex { position: [self.left, self.bottom] };
        vec![lt_vertex, rt_vertex, lb_vertex, rb_vertex]
    }
}

fn main() {
    use glium::{Surface};

    let patch_renderer = PatchRenderer::new();
    let patch = Patch::new();

    let vertex_buffer = glium::VertexBuffer::new(&patch_renderer.display, &patch.as_vertices()).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    loop {
        let mut target = patch_renderer.display.draw();
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
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn patch_renders() {}
}
