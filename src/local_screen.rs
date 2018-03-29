use ::{Block, RemoteDirector, ScreenMessage, Sigil};
use glium::{Display, Surface};
use glium::backend::Facade;
use glium::glutin::{ContextBuilder, ControlFlow, Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowBuilder, WindowEvent};
use glyffin::QuipRenderer;
use model::Patch;
use renderer::PatchRenderer;
use rusttype::Scale;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;


pub struct LocalScreen<'a> {
    blocks: HashMap<u64, Block>,
    patch_renderer: PatchRenderer,
    quip_renderer: QuipRenderer<'a>,
    display: Display,
    status: ScreenStatus,
}

pub enum AwakenMessage {
    ScreenMessage(ScreenMessage)
}

impl<'a> LocalScreen<'a> {
    pub fn start(width: u32, height: u32, remote_directory: RemoteDirector) {
        let mut events_loop = EventsLoop::new();
        let (awaken_message_sender, awaken_message_receiver) = channel::<AwakenMessage>();
        start_events_loop_awakener(&events_loop, awaken_message_sender, remote_directory.screen_message_receiver);

        let mut local_screen = LocalScreen::new(width, height, &events_loop);
        events_loop.run_forever(|ev| {
            match ev {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Closed | WindowEvent::KeyboardInput {
                            input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..
                        } => {
                            ControlFlow::Break
                        }
                        WindowEvent::Resized(width, height) => {
                            local_screen.on_dimensions(width, height);
                            ControlFlow::Continue
                        }
                        WindowEvent::Refresh => {
                            local_screen.draw();
                            ControlFlow::Continue
                        }
                        _ => {
                            ControlFlow::Continue
                        }
                    }
                }
                Event::Awakened => {
                    while let Ok(AwakenMessage::ScreenMessage(screen_message)) = awaken_message_receiver.try_recv() {
                        local_screen.update(screen_message);
                    }
                    match local_screen.status() {
                        ScreenStatus::Unchanged => ControlFlow::Continue,
                        ScreenStatus::Changed => {
                            local_screen.draw();
                            ControlFlow::Continue
                        }
                        ScreenStatus::WillClose => ControlFlow::Break,
                    }
                }
                _ => ControlFlow::Continue
            }
        });
    }

    fn new(width: u32, height: u32, events_loop: &EventsLoop) -> Self {
        let display = get_display(width, height, events_loop);
        let modelview = get_modelview(width, height, &display);
        let dpi_factor = display.gl_window().hidpi_factor();
        let local_screen = LocalScreen {
            blocks: HashMap::<u64, Block>::new(),
            patch_renderer: PatchRenderer::new(&display, modelview),
            quip_renderer: QuipRenderer::new(dpi_factor, modelview, &display),
            display,
            status: ScreenStatus::Changed,
        };
        local_screen
    }

    fn status(&self) -> ScreenStatus {
        self.status
    }

    fn on_dimensions(&mut self, width: u32, height: u32) {
        let modelview = get_modelview(width, height, &self.display);
        self.patch_renderer.set_modelview(modelview);
        self.quip_renderer.set_modelview(modelview);
        self.draw();
    }

    pub fn update(&mut self, screen_message: ScreenMessage) {
        match screen_message {
            ScreenMessage::AddBlock(id, block) => {
                let blocks = &mut self.blocks;
                blocks.insert(id, block);
                self.status = self.status.did_change()
            }
            ScreenMessage::Close => {
                self.status = self.status.will_close()
            }
        }
    }

    fn draw(&mut self) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.70, 0.80, 0.90, 1.0), 1.0);
        {
            let patch_renderer = &mut self.patch_renderer;
            let blocks = &self.blocks;
            blocks.iter().for_each(|(_, block)| {
                match block.sigil {
                    Sigil::FilledRectangle(color) => {
                        let patch = Patch::new(block.width, block.height, block.approach, color);
                        patch_renderer.set_patch(&patch);
                        patch_renderer.draw(&mut target);
                    }
                    _ => ()
                }
            });
        }
        {
            let quip_renderer = &mut self.quip_renderer;
            let blocks = &self.blocks;
            let dpi_factor = self.display.gl_window().hidpi_factor();
            let display = &self.display;
            blocks.iter().for_each(|(_, block)| {
                match block.sigil {
                    Sigil::Paragraph { line_height, ref text } => {
                        quip_renderer.layout_paragraph(
                            text,
                            Scale::uniform(line_height * dpi_factor),
                            block.width as u32,
                            block.approach,
                            display,
                        );
                        quip_renderer.draw(&mut target);
                    }
                    _ => ()
                }
            });
        }
        target.finish().unwrap();
        self.status = self.status.did_draw();
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ScreenStatus {
    Unchanged,
    Changed,
    WillClose,
}

impl ScreenStatus {
    fn will_close(&self) -> Self {
        ScreenStatus::WillClose
    }
    fn did_change(&self) -> Self {
        if *self == ScreenStatus::WillClose {
            ScreenStatus::WillClose
        } else {
            ScreenStatus::Changed
        }
    }
    fn did_draw(&self) -> Self {
        if *self == ScreenStatus::WillClose {
            ScreenStatus::WillClose
        } else {
            ScreenStatus::Unchanged
        }
    }
}

fn start_events_loop_awakener(events_loop: &EventsLoop, awaken_message_sender: Sender<AwakenMessage>, screen_message_receiver: Receiver<ScreenMessage>) {
    let events_loop_proxy = events_loop.create_proxy();
    thread::spawn(move || {
        let mut done = false;
        while !done {
            match screen_message_receiver.recv() {
                Ok(screen_message) => {
                    awaken_message_sender.send(AwakenMessage::ScreenMessage(screen_message)).unwrap();
                    events_loop_proxy.wakeup().expect("Wakeup after AwakenMessage");
                }
                Err(_) => {
                    done = true;
                }
            }
        }
    });
}

fn get_display(width: u32, height: u32, events_loop: &EventsLoop) -> Display {
    let context_builder = ContextBuilder::new().with_depth_buffer(24).with_vsync(true);
    let window_builder = WindowBuilder::new().with_dimensions(width, height).with_title("PatchGL");
    Display::new(window_builder, context_builder, events_loop).unwrap()
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
