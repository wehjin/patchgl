extern crate arrayvec;
extern crate cage;
#[macro_use]
extern crate glium;
extern crate rusttype;
extern crate unicode_normalization;
extern crate xml;

pub use base::{Color, WebColor};
use glium::backend::Facade;
use glium::glutin::{Event, EventsLoopProxy, KeyboardInput, VirtualKeyCode, WindowEvent};
use glium::Surface;
use glyffin::QuipRenderer;
use model::Patch;
use renderer::PatchRenderer;
use rusttype::Scale;
pub use sigil::Sigil;
use std::collections::HashMap;
use std::marker::Send;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub mod parser;
pub mod model;
pub mod renderer;
pub mod glyffin;
pub mod base;
pub mod ix;
mod sigil;

#[derive(Clone, Copy)]
pub struct Anchor {
    pub x: f32,
    pub y: f32,
}

impl Anchor {
    pub fn top_left() -> Self { Anchor { x: 0.0, y: 0.0 } }
}

pub struct Block {
    pub sigil: Sigil,
    pub width: f32,
    pub height: f32,
    pub approach: f32,
    pub anchor: Anchor,
}

pub enum ScreenMessage {
    AddBlock(u64, Block),
    Close,
}

pub enum DirectorMessage {}

pub struct RemoteScreen {
    sender: Sender<ScreenMessage>,
    _receiver: Receiver<DirectorMessage>,
    events_loop_proxy: glium::glutin::EventsLoopProxy,
}

impl RemoteScreen {
    pub fn add_block(&self, id: u64, block: Block) {
        self.sender.send(ScreenMessage::AddBlock(id, block)).expect("send add-block");
        self.events_loop_proxy.wakeup().expect("wakeup after add-block");
    }
    pub fn close(&self) {
        self.sender.send(ScreenMessage::Close).expect("send close");
        self.events_loop_proxy.wakeup().expect("wakeup after close");
    }
}

pub struct RemoteDirector {
    _sender: Sender<DirectorMessage>,
    receiver: Receiver<ScreenMessage>,
}

impl RemoteDirector {
    pub fn new<F>(events_loop_proxy: EventsLoopProxy, on_start: F) -> Self
        where F: Fn(&RemoteScreen) -> () + Send + 'static
    {
        let (send_to_screen, receive_from_director) = channel::<ScreenMessage>();
        let (send_to_director, receive_from_screen) = channel::<DirectorMessage>();
        let director = RemoteDirector {
            _sender: send_to_director,
            receiver: receive_from_director,
        };
        thread::spawn(move || {
            let remote_screen = RemoteScreen {
                sender: send_to_screen,
                _receiver: receive_from_screen,
                events_loop_proxy,
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
    let mut events_loop = glium::glutin::EventsLoop::new();
    let context_builder = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(true);
    let window_builder = glium::glutin::WindowBuilder::new()
        .with_dimensions(width, height)
        .with_title("PatchGl");
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();
    let dpi_factor = display.gl_window().hidpi_factor();
    let director = RemoteDirector::new(events_loop.create_proxy(), on_start);
    let modelview = get_modelview(width, height, &display);

    let mut patch_renderer = PatchRenderer::new(&display, modelview);
    let mut quip_renderer = QuipRenderer::new(dpi_factor, modelview, &display);
    let mut blocks = HashMap::<u64, Block>::new();

    let mut done = false;
    while !done {
        let mut target = display.draw();
        target.clear_color_and_depth((0.70, 0.80, 0.90, 1.0), 1.0);
        for (_, block) in &blocks {
            match block.sigil {
                Sigil::FilledRectangle(color) => {
                    let patch = Patch::new(block.width, block.height, block.approach, color);
                    patch_renderer.set_patch(&patch);
                    patch_renderer.draw(&mut target);
                }
                _ => ()
            }
        }
        for (_, block) in &blocks {
            match block.sigil {
                Sigil::Paragraph { line_height, ref text } => {
                    quip_renderer.layout_paragraph(text,
                                                   Scale::uniform(line_height * dpi_factor),
                                                   block.width as u32,
                                                   block.approach,
                                                   &display);
                    quip_renderer.draw(&mut target);
                }
                _ => ()
            }
        }

        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            println!("{:?}", ev);
            match ev {
                Event::WindowEvent { event: WindowEvent::Closed, .. }
                | Event::WindowEvent {
                    event: WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape), ..
                        }, ..
                    }, ..
                }
                => {
                    done = true;
                }
                Event::Awakened => {
                    while let Some(screen_message) = director.receive_screen_message() {
                        match screen_message {
                            ScreenMessage::Close => {
                                done = true;
                            }
                            ScreenMessage::AddBlock(id, block) => {
                                blocks.insert(id, block);
                            }
                        }
                    }
                }
                _ => ()
            }
        });
    }
}

fn get_modelview<F: Facade>(screen_width: u32, screen_height: u32, display: &F) -> [[f32; 4]; 4] {
    let (window_width, window_height) = display.get_context().get_framebuffer_dimensions();
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