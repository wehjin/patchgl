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
    use ix::{Readable, BasicSequence, Reading};

    let director_commands = ix::from_value(Command::Close).delay(3000);

    let xml = include_str!("screen_with_square_patch.xml");
    let patchwork = Patchwork::from_xml(xml);

    let screen = Screen::new(320, 480);
    let display_rc = screen.display.clone();
    let display = &*display_rc;

    let user_commands = {
        let display_rc2 = display_rc.clone();
        ix::from_on_sequence(Box::new(move || {
            let display_rc3 = display_rc2.clone();
            Box::new(BasicSequence {
                on_next: Box::new(move || {
                    for ev in display_rc3.wait_events() {
                        match ev {
                            Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | glutin::Event::Closed => {
                                return Reading::Next(Command::Close)
                            },
                            _ => ()
                        }
                    }
                    Reading::Complete
                }),
                on_stop: Box::new(|| {}),
            })
        }))
    };


    let patch_renderer = PatchRenderer::new(&patchwork, display);
    let modelview = patch_renderer.get_modelview(display);

    let mut quip_renderer = QuipRenderer::new(screen.dpi_factor(), modelview, display);
    quip_renderer.layout_paragraph("I for one welcome our new robot overlords",
                                   Scale::uniform(24.0 * screen.dpi_factor()), screen.width, display);

    let mut command_sequence = user_commands.sequence();

    loop {
        let mut target = display.draw();
        target.clear_color(0.70, 0.80, 0.90, 1.0);
        patch_renderer.draw(&mut target, &display);
        quip_renderer.draw(&mut target);
        target.finish().unwrap();

        while let Reading::Next(command) = command_sequence.next() {
            match command {
                Command::Close => return
            }
        }
    }
}

#[cfg(test)]
mod tests {}