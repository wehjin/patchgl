#[macro_use]
extern crate glium;
extern crate xml;
extern crate cage;
extern crate patchgllib;
extern crate rusttype;
extern crate arrayvec;

use patchgllib::model::Patchwork;
use patchgllib::renderer::PatchRenderer;
use patchgllib::glyffin::QuipRenderer;
use rusttype::{Scale};
use glium::glutin;

fn main() {
    let xml = include_str!("screen_with_square_patch.xml");
    let patchwork = Patchwork::from_xml(xml);

    use patchgllib::screen::Screen;
    let screen = Screen::new(320, 480);
    let display = &screen.display;

    let patch_renderer = PatchRenderer::new(&patchwork, &display);
    let modelview = patch_renderer.get_modelview(&display);

    let mut quip_renderer = QuipRenderer::new(screen.dpi_factor(), modelview, display);
    quip_renderer.layout_paragraph("I for one welcome our new robot overlords",
                                   Scale::uniform(24.0 * screen.dpi_factor()), screen.width, display);

    loop {
        use glium::{Surface};

        let mut target = display.draw();
        target.clear_color(0.70, 0.80, 0.90, 1.0);

        patch_renderer.draw(&mut target, &display);
        quip_renderer.draw(&mut target, &display);

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
