use arrayvec;
use arrayvec::ArrayVec;
use glium;
use glium::backend::Facade;
use rusttype::{Font, FontCollection, PositionedGlyph, VMetrics};
use rusttype::{Point, point, Rect, Scale, vector};
use rusttype::gpu_cache::Cache;
use std::borrow::Cow;
use unicode_normalization::UnicodeNormalization;


pub const Z_FACTOR: f32 = -0.001;

pub struct QuipRenderer<'a> {
    pub font: Font<'a>,
    pub program: glium::Program,
    pub cache: Cache,
    pub cache_dimensions: (u32, u32),
    texture: glium::texture::Texture2d,
    modelview: [[f32; 4]; 4],
    vertex_buffer: glium::VertexBuffer<Vertex>,
    draw_parameters: glium::DrawParameters<'static>,
}

impl<'a> QuipRenderer<'a> {
    pub fn set_modelview(&mut self, modelview: [[f32; 4]; 4]) {
        self.modelview = modelview;
    }

    pub fn layout_paragraph<F: Facade>(&mut self, text: &str, (x, y): (f32, f32), scale: Scale, width: u32, approach: f32, colour: [f32; 4], display: &F) {
        let glyphs = layout_glyphs(&self.font, scale, width, text);
        for glyph in &glyphs {
            self.cache.queue_glyph(0, glyph.clone());
        }
        let texture = &self.texture;
        self.cache.cache_queued(|rect, data| {
            texture.main_level().write(glium::Rect {
                left: rect.min.x,
                bottom: rect.min.y,
                width: rect.width(),
                height: rect.height(),
            }, glium::texture::RawImage2d {
                data: Cow::Borrowed(data),
                width: rect.width(),
                height: rect.height(),
                format: glium::texture::ClientFormat::U8,
            });
        }).expect("cache_queued");

        implement_vertex!(Vertex, position, tex_coords, colour);
        let origin = point(x, y);
        let z = approach * Z_FACTOR;
        let vertices: Vec<Vertex> = glyphs.iter().flat_map(|g| {
            if let Ok(Some((uv_rect, screen_rect))) = self.cache.rect_for(0, g) {
                let gl_rect = Rect {
                    min: origin + vector(screen_rect.min.x as f32, screen_rect.min.y as f32),
                    max: origin + vector(screen_rect.max.x as f32, screen_rect.max.y as f32),
                };
                layout_vertices(z, &uv_rect, &gl_rect, &colour)
            } else {
                arrayvec::ArrayVec::new()
            }
        }).collect();
        self.vertex_buffer = glium::VertexBuffer::new(display, &vertices).expect("VertextBuffer::new");
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        use glium::Surface;
        let sampler = self.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);
        let uniforms = uniform! { tex: sampler, modelview: self.modelview };
        frame.draw(&self.vertex_buffer,
                   glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                   &self.program,
                   &uniforms,
                   &self.draw_parameters)
             .expect("frame.draw");
    }

    pub fn new<F: Facade>(cache_dpi_factor: f32, modelview: [[f32; 4]; 4], display: &F) -> Self {
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
                format: glium::texture::ClientFormat::U8,
            },
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap).unwrap();
        QuipRenderer {
            font: FontCollection::from_bytes(include_bytes!("Arial Unicode.ttf") as &[u8]).into_font().unwrap(),
            program,
            cache: Cache::new(cache_width, cache_height, 0.1, 0.1),
            cache_dimensions: (cache_width, cache_height),
            texture,
            modelview,
            vertex_buffer: glium::VertexBuffer::new(display, &[]).unwrap(),
            draw_parameters: glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
        }
    }
}

pub fn layout_glyphs<'a>(font: &'a Font, scale: Scale, width: u32, text: &str) -> Vec<PositionedGlyph<'a>> {
    let mut glyph_writer = GlyphWriter::new(&font.v_metrics(scale));
    let mut last_glyph_id = None;
    for c in text.nfc() {
        if c.is_control() {
            match c {
                '\r' => glyph_writer.advance_line(),
                '\n' => (),
                _ => (),
            }
            continue;
        }
        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };
        let kerning = if let Some(id) = last_glyph_id.take() {
            font.pair_kerning(scale, id, base_glyph.id())
        } else {
            0.0
        };
        last_glyph_id = Some(base_glyph.id());

        glyph_writer.advance_right(kerning);
        let mut glyph = base_glyph.scaled(scale).positioned(glyph_writer.position());
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                glyph_writer.advance_right(-kerning);
                glyph_writer.advance_line();
                glyph = glyph.into_unpositioned().positioned(glyph_writer.position());
                last_glyph_id = None;
            }
        }
        glyph_writer.advance_right(glyph.unpositioned().h_metrics().advance_width);
        glyph_writer.add_glyph(glyph);
    }
    glyph_writer.take_glyphs()
}

struct GlyphWriter<'a> {
    line_stride: f32,
    caret: Point<f32>,
    line: Vec<PositionedGlyph<'a>>,
    page: Vec<Vec<PositionedGlyph<'a>>>,
}

impl<'a> GlyphWriter<'a> {
    fn new(v_metrics: &VMetrics) -> Self {
        let line_stride = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        let caret = point(0.0, v_metrics.ascent);
        GlyphWriter { line_stride, caret, line: Vec::new(), page: Vec::new() }
    }
    fn position(&self) -> Point<f32> {
        self.caret
    }
    fn advance_line(&mut self) {
        self.caret = point(0.0, self.caret.y + self.line_stride);
        let mut line = Vec::new();
        line.append(&mut self.line);
        self.page.push(line);
    }
    fn advance_right(&mut self, amount: f32) {
        self.caret.x += amount;
    }
    fn add_glyph(&mut self, glyph: PositionedGlyph<'a>) {
        self.line.push(glyph);
    }
    fn take_glyphs(&mut self) -> Vec<PositionedGlyph<'a>> {
        let mut lines = Vec::new();
        lines.append(&mut self.page);
        if !self.line.is_empty() {
            let mut line = Vec::new();
            line.append(&mut self.line);
            lines.push(line);
        }
        lines.into_iter().fold(Vec::new(), |mut all, more| {
            all.extend(more);
            all
        })
    }
}

fn layout_vertices(z: f32, uv_rect: &Rect<f32>, gl_rect: &Rect<f32>, colour: &[f32; 4]) -> ArrayVec<[Vertex; 6]> {
    ArrayVec::<[Vertex; 6]>::from([
        Vertex {
            position: [gl_rect.min.x, gl_rect.max.y, z],
            tex_coords: [uv_rect.min.x, uv_rect.max.y],
            colour: *colour,
        },
        Vertex {
            position: [gl_rect.min.x, gl_rect.min.y, z],
            tex_coords: [uv_rect.min.x, uv_rect.min.y],
            colour: *colour,
        },
        Vertex {
            position: [gl_rect.max.x, gl_rect.min.y, z],
            tex_coords: [uv_rect.max.x, uv_rect.min.y],
            colour: *colour,
        },
        Vertex {
            position: [gl_rect.max.x, gl_rect.min.y, z],
            tex_coords: [uv_rect.max.x, uv_rect.min.y],
            colour: *colour,
        },
        Vertex {
            position: [gl_rect.max.x, gl_rect.max.y, z],
            tex_coords: [uv_rect.max.x, uv_rect.max.y],
            colour: *colour,
        },
        Vertex {
            position: [gl_rect.min.x, gl_rect.max.y, z],
            tex_coords: [uv_rect.min.x, uv_rect.max.y],
            colour: *colour,
        }])
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    colour: [f32; 4],
}
