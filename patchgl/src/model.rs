use cage::Cage;
use parser;
use xml;

#[derive(Default)]
pub struct Patchwork {
    pub patch: Patch
}

impl Patchwork {
    pub fn from_xml(xml_string: &str) -> Self {
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
pub struct Patch {
    cage: Cage
}

impl Patch {
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
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);
