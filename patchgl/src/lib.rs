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

use model::{Patchwork, Patch};
use renderer::PatchRenderer;
use glyffin::QuipRenderer;
use rusttype::{Scale};
use glium::glutin;
use screen::Screen;
use glium::{Surface};
use glium::glutin::Event;
use glium::backend::glutin_backend::GlutinFacade;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::marker::Send;

pub enum ScreenMessage {
    Close
}

pub enum DirectorMessage {}

pub struct RemoteScreen {
    sender: Sender<ScreenMessage>,
    receiver: Receiver<DirectorMessage>,
    window_proxy: glium::glutin::WindowProxy,
}

impl RemoteScreen {
    pub fn close(&self) {
        self.sender.send(ScreenMessage::Close).unwrap();
        self.window_proxy.wakeup_event_loop()
    }
}

pub struct RemoteDirector {
    sender: Sender<DirectorMessage>,
    receiver: Receiver<ScreenMessage>,
}

impl RemoteDirector {
    pub fn receive_screen_message(&self) -> Option<ScreenMessage> {
        let result = self.receiver.try_recv();
        if result.is_ok() {
            Option::Some(result.unwrap())
        } else {
            Option::None
        }
    }
}


pub fn start<F>(width: u32, height: u32, on_start: F)
    where F: Fn(&RemoteScreen) -> () + Send + 'static
{
    let screen = Screen::new(width, height);
    let display_rc = screen.display.clone();
    let display: &GlutinFacade = &*display_rc;

    let (send_to_screen, receive_from_director) = channel::<ScreenMessage>();
    let (send_to_director, receive_from_screen) = channel::<DirectorMessage>();
    let director = RemoteDirector {
        sender: send_to_director,
        receiver: receive_from_director,
    };

    let window_proxy = (*display.get_window().unwrap()).create_window_proxy();
    thread::spawn(move || {
        let remote_screen = RemoteScreen {
            sender: send_to_screen,
            receiver: receive_from_screen,
            window_proxy: window_proxy,
        };
        on_start(&remote_screen)
    });

    let patchwork = Patchwork {
        patch: Patch::from_dimensions(width as f32, width as f32, 0f32),
        width: width,
        height: height,
    };
    let patch_renderer = PatchRenderer::new(&patchwork, display);
    let modelview = patch_renderer.get_modelview(display);

    let mut quip_renderer = QuipRenderer::new(screen.dpi_factor(), modelview, display);
    quip_renderer.layout_paragraph("I for one welcome our new robot overlords",
                                   Scale::uniform(24.0 * screen.dpi_factor()), screen.width, display);

    'start: loop {
        let mut target = display.draw();
        target.clear_color(0.70, 0.80, 0.90, 1.0);
        patch_renderer.draw(&mut target, &display);
        quip_renderer.draw(&mut target);
        target.finish().unwrap();

        for ev in display.wait_events() {
            match ev {
                Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | glutin::Event::Closed => {
                    break 'start
                }
                Event::Awakened => {
                    while let Some(ScreenMessage::Close) = director.receive_screen_message() {
                        break 'start
                    }
                }
                _ => ()
            }
        }
    }
}

pub fn go() {
    start(320, 480, |screen: &RemoteScreen| {
        use std::time::Duration;
        thread::sleep(Duration::from_secs(3));
        screen.close()
    });
}

#[cfg(test)]
mod tests {}