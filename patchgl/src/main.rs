#[macro_use] extern crate glium;
extern crate xml;
extern crate cage;
extern crate patchgllib;
extern crate rusttype;
extern crate arrayvec;

use patchgllib::model::Patchwork;
use patchgllib::renderer::PatchRenderer;
use patchgllib::glyffin::QuipRenderer;
use rusttype::{Scale};
use std::borrow::Cow;
use glium::glutin;

fn main() {
    let xml = include_str!("screen_with_square_patch.xml");
    let patchwork = Patchwork::from_xml(xml);

    use patchgllib::screen::Screen;
    let screen = Screen::new(&patchwork);
    let display = &screen.display;

    let patch_renderer = PatchRenderer::new(&patchwork, &display);
    let patchwork_uniforms = uniform! {
        modelview: patch_renderer.get_modelview(&display)
    };

    let text: String = "I for one welcome our new robot overloads".into();
    let mut quip_renderer = QuipRenderer::new(&display, screen.get_dpi_factor());
    let (texture_width, texture_height) = quip_renderer.cache_dimensions;
    let texture = glium::texture::Texture2d::with_format(
        display,
        glium::texture::RawImage2d {
            data: Cow::Owned(vec![128u8; texture_width as usize * texture_height as usize]),
            width: texture_width,
            height: texture_height,
            format: glium::texture::ClientFormat::U8
        },
        glium::texture::UncompressedFloatFormat::U8,
        glium::texture::MipmapsOption::NoMipmap).unwrap();
    let vertices = quip_renderer.layout_paragraph(Scale::uniform(24.0 * screen.get_dpi_factor()), patchwork.width, &text, &texture);
    let quip_vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let quip_uniforms = uniform! {
        tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
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
                    &quip_renderer.program, &quip_uniforms,
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
