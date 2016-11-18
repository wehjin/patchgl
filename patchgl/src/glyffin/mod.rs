use rusttype::{FontCollection, Font, Scale, Rect, point, vector, PositionedGlyph};
use rusttype::gpu_cache::Cache;
use glium;
use std::borrow::Cow;
use arrayvec;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    colour: [f32; 4]
}

pub struct QuipRenderer<'a> {
    pub font: Font<'a>,
    pub program: glium::Program,
    pub cache: Cache,
    pub cache_dimensions: (u32, u32),
}

impl<'a> QuipRenderer<'a> {
    pub fn layout_paragraph(&mut self, scale: Scale, width: u32, text: &str, texture: &glium::texture::Texture2d) -> Vec<Vertex> {
        let glyphs = layout_paragraph(&self.font, scale, width, text);
        for glyph in &glyphs {
            self.cache.queue_glyph(0, glyph.clone());
        }
        self.cache.cache_queued(|rect, data| {
            texture.main_level().write(glium::Rect {
                left: rect.min.x,
                bottom: rect.min.y,
                width: rect.width(),
                height: rect.height()
            }, glium::texture::RawImage2d {
                data: Cow::Borrowed(data),
                width: rect.width(),
                height: rect.height(),
                format: glium::texture::ClientFormat::U8
            });
        }).unwrap();

        implement_vertex!(Vertex, position, tex_coords, colour);
        let colour = [0.0, 0.0, 0.0, 1.0];
        let origin = point(0.0, 0.0);
        let vertices: Vec<Vertex> = glyphs.iter().flat_map(|g| {
            if let Ok(Some((uv_rect, screen_rect))) = self.cache.rect_for(0, g) {
                let gl_rect = Rect {
                    min: origin + vector(screen_rect.min.x as f32, screen_rect.min.y as f32),
                    max: origin + vector(screen_rect.max.x as f32, screen_rect.max.y as f32)
                };
                arrayvec::ArrayVec::<[Vertex; 6]>::from([
                    Vertex {
                        position: [gl_rect.min.x, gl_rect.max.y],
                        tex_coords: [uv_rect.min.x, uv_rect.max.y],
                        colour: colour
                    },
                    Vertex {
                        position: [gl_rect.min.x, gl_rect.min.y],
                        tex_coords: [uv_rect.min.x, uv_rect.min.y],
                        colour: colour
                    },
                    Vertex {
                        position: [gl_rect.max.x, gl_rect.min.y],
                        tex_coords: [uv_rect.max.x, uv_rect.min.y],
                        colour: colour
                    },
                    Vertex {
                        position: [gl_rect.max.x, gl_rect.min.y],
                        tex_coords: [uv_rect.max.x, uv_rect.min.y],
                        colour: colour
                    },
                    Vertex {
                        position: [gl_rect.max.x, gl_rect.max.y],
                        tex_coords: [uv_rect.max.x, uv_rect.max.y],
                        colour: colour
                    },
                    Vertex {
                        position: [gl_rect.min.x, gl_rect.max.y],
                        tex_coords: [uv_rect.min.x, uv_rect.max.y],
                        colour: colour
                    }])
            } else {
                arrayvec::ArrayVec::new()
            }
        }).collect();
        vertices
    }

    pub fn new(display: &glium::backend::glutin_backend::GlutinFacade, cache_dpi_factor: f32) -> Self {
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
        let (cache_width, cache_height) = (512 * cache_dpi_factor as u32, 512 * cache_dpi_factor as u32);
        QuipRenderer {
            font: font,
            program: program,
            cache: Cache::new(cache_width, cache_height, 0.1, 0.1),
            cache_dimensions: (cache_width, cache_height)
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
