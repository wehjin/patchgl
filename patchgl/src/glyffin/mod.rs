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
    texture: glium::texture::Texture2d,
    modelview: [[f32; 4]; 4],
    vertex_buffer: glium::VertexBuffer<Vertex>,
}

impl<'a> QuipRenderer<'a> {
    pub fn layout_paragraph(&mut self, text: &str, scale: Scale, width: u32,
                            display: &glium::backend::glutin_backend::GlutinFacade) {
        let glyphs = layout_paragraph(&self.font, scale, width, text);
        for glyph in &glyphs {
            self.cache.queue_glyph(0, glyph.clone());
        }
        let texture = &self.texture;
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
        self.vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    }

    pub fn draw(&self, frame: &mut glium::Frame, display: &glium::backend::glutin_backend::GlutinFacade) {
        use glium::Surface;
        let sampler = self.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);
        let uniforms = uniform! { tex: sampler, modelview: self.modelview };
        frame.draw(&self.vertex_buffer,
                   glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                   &self.program,
                   &uniforms,
                   &glium::DrawParameters { blend: glium::Blend::alpha_blending(), ..Default::default() })
             .unwrap();
    }

    pub fn new(cache_dpi_factor: f32,
               modelview: [[f32; 4]; 4],
               display: &glium::backend::glutin_backend::GlutinFacade) -> Self {
        let (vertex_shader, fragment_shader) = (include_str!("quip_vertex_shader.glsl"),
                                                include_str!("quip_fragment_shader.glsl"));
        let program = program!(display, 140 => {vertex: vertex_shader, fragment: fragment_shader}).unwrap();
        let (cache_width, cache_height) = (512 * cache_dpi_factor as u32, 512 * cache_dpi_factor as u32);
        let texture = glium::texture::Texture2d::with_format(
            display,
            glium::texture::RawImage2d {
                data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
                width: cache_width,
                height: cache_height,
                format: glium::texture::ClientFormat::U8
            },
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap).unwrap();
        QuipRenderer {
            font: FontCollection::from_bytes(include_bytes!("Arial Unicode.ttf") as &[u8]).into_font().unwrap(),
            program: program,
            cache: Cache::new(cache_width, cache_height, 0.1, 0.1),
            cache_dimensions: (cache_width, cache_height),
            texture: texture,
            modelview: modelview,
            vertex_buffer: glium::VertexBuffer::new(display, &[]).unwrap()
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
