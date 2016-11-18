#[macro_use] extern crate glium;
extern crate xml;
extern crate cage;
extern crate patchgllib;
extern crate rusttype;
extern crate arrayvec;

use patchgllib::model::Patchwork;
use patchgllib::renderer::PatchRenderer;
use patchgllib::glyffin::QuipRenderer;
use patchgllib::glyffin;
use rusttype::gpu_cache::{Cache};
use rusttype::{point, vector, Scale, Rect};
use std::borrow::Cow;
use glium::glutin;

fn main() {
    let xml = include_str!("screen_with_square_patch.xml");
    let patchwork = Patchwork::from_xml(xml);

    use patchgllib::screen::Screen;
    let screen = Screen::new(&patchwork);
    let display = &screen.display;
    let dpi_factor = screen.dpi_factor();
    let (cache_width, cache_height) = (512 * dpi_factor as u32, 512 * dpi_factor as u32);
    let mut cache = Cache::new(cache_width, cache_height, 0.1, 0.1);

    let patch_renderer = PatchRenderer::new(patchwork, &display);

    let text: String = "I for one welcome our new robot overloads".into();
    let quip_renderer = QuipRenderer::new();
    let glyphs = glyffin::layout_paragraph(&quip_renderer.font, Scale::uniform(24.0 * dpi_factor), 320, &text);
    for glyph in &glyphs {
        cache.queue_glyph(0, glyph.clone());
    }
    let quip_program = program!( display, 140 => {
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
    let cache_tex = glium::texture::Texture2d::with_format(
        display,
        glium::texture::RawImage2d {
            data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
            width: cache_width,
            height: cache_height,
            format: glium::texture::ClientFormat::U8
        },
        glium::texture::UncompressedFloatFormat::U8,
        glium::texture::MipmapsOption::NoMipmap).unwrap();

    cache.cache_queued(|rect, data| {
        cache_tex.main_level().write(glium::Rect {
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

    let quip_vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
            colour: [f32; 4]
        }

        implement_vertex!(Vertex, position, tex_coords, colour);
        let colour = [0.0, 0.0, 0.0, 1.0];
        let origin = point(0.0, 0.0);
        let vertices: Vec<Vertex> = glyphs.iter().flat_map(|g| {
            if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
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

        glium::VertexBuffer::new(
            display,
            &vertices).unwrap()
    };

    let patchwork_uniforms = uniform! {
        modelview: patch_renderer.get_modelview(&display)
    };
    let quip_uniforms = uniform! {
        tex: cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        modelview: patch_renderer.get_modelview(&display)
    };

    loop {
        let mut target = display.draw();
        use glium::{Surface};
        target.clear_color(0.70, 0.80, 0.90, 1.0);

        target.draw(&patch_renderer.vertex_buffer,
                    &patch_renderer.indices,
                    &patch_renderer.program,
                    &patchwork_uniforms,
                    &Default::default()).unwrap();

        target.draw(&quip_vertex_buffer,
                    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &quip_program, &quip_uniforms,
                    &glium::DrawParameters {
                        blend: glium::Blend::alpha_blending(),
                        ..Default::default()
                    }).unwrap();

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | glutin::Event::Closed => return,
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
