#[macro_use]
extern crate glium;
extern crate xml;
extern crate cage;
extern crate rusttype;
extern crate unicode_normalization;
extern crate arrayvec;

pub mod parser;
pub mod model;
pub mod renderer;
pub mod glyffin;
pub mod screen;
pub mod base;
pub mod ix;

use model::Patchwork;
use renderer::PatchRenderer;
use glyffin::QuipRenderer;
use rusttype::{Scale};
use glium::glutin;
use screen::Screen;
use glium::{Surface};
use glium::glutin::Event;

#[derive(Clone, Copy)]
enum Command {
    Close
}

pub fn go() {
    use ix::Readable;

    let director_readable = ix::from_value(Command::Close).delay(3000);
    let xml = include_str!("screen_with_square_patch.xml");
    let patchwork = Patchwork::from_xml(xml);

    let screen = Screen::new(320, 480);
    let display = &screen.display;

    let patch_renderer = PatchRenderer::new(&patchwork, &display);
    let modelview = patch_renderer.get_modelview(&display);

    let mut quip_renderer = QuipRenderer::new(screen.dpi_factor(), modelview, display);
    quip_renderer.layout_paragraph("I for one welcome our new robot overlords",
                                   Scale::uniform(24.0 * screen.dpi_factor()), screen.width, display);

    let mut command_sequence = director_readable.sequence();

    loop {
        let mut target = display.draw();
        target.clear_color(0.70, 0.80, 0.90, 1.0);

        patch_renderer.draw(&mut target, &display);
        quip_renderer.draw(&mut target);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | glutin::Event::Closed => return,
                _ => ()
            }
        }

        use ix::Reading;
        while let Reading::Next(command) = command_sequence.next() {
            match command {
                Command::Close => return
            }
        }
    }
}

#[cfg(test)]
mod tests {}