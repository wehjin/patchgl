use cage::{Cage};
use parser;
use xml;

#[derive(Default)]
pub struct Patchwork {
    pub patch: Patch,
    pub width: u32,
    pub height: u32,
}

impl Patchwork {
    pub fn from_xml(xml_string: &str) -> Self {
        let mut patchwork = Patchwork { ..Default::default() };
        use xml::reader::{EventReader, XmlEvent};
        let parser = EventReader::from_str(xml_string);
        for event in parser {
            match event {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    if name.local_name == "patch" {
                        patchwork.patch = Patch::from_attributes(&attributes);
                    } else if name.local_name == "screen" {
                        patchwork.width = 320u32;
                        patchwork.height = 480u32;
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
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

#[derive(Default)]
pub struct Patch {
    cage: Cage
}

impl Patch {
    pub fn from_dimensions(width: f32, height: f32, depth: f32) -> Self {
        Patch { cage: Cage::from((0f32, width, 0f32, height, depth, depth)) }
    }
    pub fn from_attributes(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Self {
        let mut patch = Patch { ..Default::default() };
        for attribute in attributes {
            if attribute.name.local_name == "bounds" {
                let cage = parser::cage_from_string(&attribute.value);
                patch.cage = cage;
            }
        }
        patch
    }
    pub fn as_trianglelist(&self) -> Vec<Vertex> {
        let (left, right, bottom, top, _, _) = self.cage.limits();
        let lt_vertex = Vertex { position: [left, top] };
        let rt_vertex = Vertex { position: [right, top] };
        let rb_vertex = Vertex { position: [right, bottom] };
        let lb_vertex = Vertex { position: [left, bottom] };
        vec![lt_vertex, rt_vertex, lb_vertex, lb_vertex, rt_vertex, rb_vertex]
    }
    pub fn vertex_count() -> usize {
        6
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);
