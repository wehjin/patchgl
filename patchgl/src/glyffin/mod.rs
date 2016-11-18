use rusttype::{FontCollection, Font, Scale, point, PositionedGlyph};
use glium;

pub struct QuipRenderer<'a> {
    pub font: Font<'a>,
    pub program: glium::Program
}

impl<'a> QuipRenderer<'a> {
    pub fn new(display: &glium::backend::glutin_backend::GlutinFacade) -> Self {
        let font_data = include_bytes!("Arial Unicode.ttf");
        let font = FontCollection::from_bytes(font_data as &[u8]).into_font().unwrap();
        let program = program!( display, 140 => {
                vertex: "
                    #version 140
                    uniform mat4 modelview;
                    in vec2 position;
                    in vec2 tex_coords;
                    in vec4 colour;
                    out vec2 v_tex_coords;
                    out vec4 v_colour;
                    void main() {
                        gl_Position = modelview * vec4(position, 0.0, 1.0);
                        v_tex_coords = tex_coords;
                        v_colour = colour;
                    }
                ",

                fragment: "
                    #version 140
                    uniform sampler2D tex;
                    in vec2 v_tex_coords;
                    in vec4 v_colour;
                    out vec4 f_colour;
                    void main() {
                        f_colour = v_colour * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);
                    }
                "
            }).unwrap();
        QuipRenderer {
            font: font,
            program: program
        }
    }
}

pub fn layout_paragraph<'a>(font: &'a Font, scale: Scale, width: u32, text: &str) -> Vec<PositionedGlyph<'a>> {
    use unicode_normalization::UnicodeNormalization;
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;
    for c in text.nfc() {
        if c.is_control() {
            match c {
                '\r' => {
                    caret = point(0.0, caret.y + advance_height);
                }
                '\n' => {},
                _ => {}
            }
            continue;
        }
        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph = glyph.into_unpositioned().positioned(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }
    result
}
