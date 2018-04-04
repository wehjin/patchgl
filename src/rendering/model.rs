use cage::Cage;
use Color;
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
    pub cage: Cage,
    pub color: Color,
}

impl Patch {
    pub fn new((x, y): (f32, f32), width: f32, height: f32, z: f32, color: Color) -> Self {
        Patch { cage: Cage::from((x, x + width, y, y + height, z, z)), color }
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
        let (left, right, bottom, top, far, _) = self.cage.limits();
        let lt_vertex = Vertex { position: [left, top, far] };
        let rt_vertex = Vertex { position: [right, top, far] };
        let rb_vertex = Vertex { position: [right, bottom, far] };
        let lb_vertex = Vertex { position: [left, bottom, far] };
        vec![lt_vertex, rt_vertex, lb_vertex, lb_vertex, rt_vertex, rb_vertex]
    }
    pub fn vertex_count() -> usize {
        6
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);
