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
pub mod base;
pub mod ix;

use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::marker::Send;
use glium::glutin;
use glium::{Surface};
use glium::glutin::Event;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::WindowProxy;
use glium::glutin::WindowBuilder;
use glium::backend::glutin_backend::WinRef;
use glium::DisplayBuild;
use model::Patch;
use renderer::PatchRenderer;
use glyffin::QuipRenderer;
use rusttype::{Scale};

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
    pub fn new<F>(window_proxy: WindowProxy, on_start: F) -> Self
        where F: Fn(&RemoteScreen) -> () + Send + 'static
    {
        let (send_to_screen, receive_from_director) = channel::<ScreenMessage>();
        let (send_to_director, receive_from_screen) = channel::<DirectorMessage>();
        let director = RemoteDirector {
            sender: send_to_director,
            receiver: receive_from_director,
        };
        thread::spawn(move || {
            let remote_screen = RemoteScreen {
                sender: send_to_screen,
                receiver: receive_from_screen,
                window_proxy: window_proxy,
            };
            on_start(&remote_screen)
        });
        director
    }
    pub fn receive_screen_message(&self) -> Option<ScreenMessage> {
        let result = self.receiver.try_recv();
        if result.is_ok() {
            Option::Some(result.unwrap())
        } else {
            Option::None
        }
    }
}


pub fn run<F>(width: u32, height: u32, on_start: F)
    where F: Fn(&RemoteScreen) -> () + Send + 'static
{
    let display = WindowBuilder::new().with_dimensions(width, height)
                                      .with_title("PatchGl")
                                      .with_vsync()
                                      .build_glium().unwrap();
    let window: WinRef = display.get_window().unwrap();
    let dpi_factor = window.hidpi_factor();

    let director = RemoteDirector::new(window.create_window_proxy(), on_start);
    let modelview = get_modelview(width, height, &display);

    let patch = Patch::from_dimensions(width as f32, width as f32, 0f32);
    let mut patch_renderer = PatchRenderer::new(&display, modelview);

    let mut quip_renderer = QuipRenderer::new(dpi_factor, modelview, &display);
    quip_renderer.layout_paragraph("I for one welcome our new robot overlords",
                                   Scale::uniform(24.0 * dpi_factor), width, &display);

    'draw: loop {
        let mut target = display.draw();
        target.clear_color(0.70, 0.80, 0.90, 1.0);

        patch_renderer.set_patch(&patch);
        patch_renderer.draw(&mut target);

        quip_renderer.draw(&mut target);
        target.finish().unwrap();

        for ev in display.wait_events() {
            match ev {
                Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | glutin::Event::Closed => {
                    break 'draw
                }
                Event::Awakened => {
                    while let Some(ScreenMessage::Close) = director.receive_screen_message() {
                        break 'draw
                    }
                }
                _ => continue 'draw
            }
        }
    }
}

fn get_modelview(screen_width: u32, screen_height: u32, display: &GlutinFacade) -> [[f32; 4]; 4] {
    let (window_width, window_height) = display.get_framebuffer_dimensions();
    let screen_aspect = screen_width as f32 / screen_height as f32;
    let window_aspect = window_width as f32 / window_height as f32;
    let ndc_width = 2.0f32 * screen_aspect / window_aspect;
    let ndc_height = 2.0f32;
    [
        [1.0 / screen_width as f32 * ndc_width, 0.0, 0.0, 0.0],
        [0.0, -1.0 / screen_height as f32 * ndc_height, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-ndc_width / 2.0, ndc_height / 2.0, 0.0, 1.0f32],
    ]
}

#[cfg(test)]
mod tests {}