use ::{Block, Sigil};
use ::{DirectorMsg, ScreenMsg, TouchMsg};
use ::rendering::{PatchRenderer, ShadowRenderer};
use ::rendering::model::Patch;
use glium::{Display, Frame, Surface};
use glium::backend::Facade;
use glium::glutin::{ContextBuilder, ControlFlow, Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowBuilder, WindowEvent};
use glium::glutin::{ElementState, MouseButton};
use glyffin::QuipRenderer;
use rusttype::Scale;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub const MAX_APPROACH: f32 = 32.0f32;
const SCREEN_APPROACH: f32 = MAX_APPROACH * 1.0625;

pub fn start(width: u32, height: u32, director: Sender<DirectorMsg>) {
    let (screen, screen_msg_receiver) = channel::<ScreenMsg>();
    director.send(DirectorMsg::ScreenReady(screen)).unwrap();

    let mut events_loop = EventsLoop::new();
    let (awaken_message_sender, awaken_message_receiver) = channel::<AwakenMessage>();
    spawn_awakener(&events_loop, awaken_message_sender, screen_msg_receiver);

    let mut local_screen = LocalScreen::new(width, height, &events_loop, director.clone());
    events_loop.run_forever(|ev| {
        match ev {
            Event::WindowEvent { event, .. } => process_window_event(event, &director, &mut local_screen),
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
    director.send(DirectorMsg::ScreenClosed).unwrap();
}

fn process_window_event(event: WindowEvent, director: &Sender<DirectorMsg>, local_screen: &mut LocalScreen) -> ControlFlow {
    match event {
        WindowEvent::Closed => ControlFlow::Break,
        WindowEvent::KeyboardInput { input: KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(keycode), .. }, .. } => {
            match keycode {
                VirtualKeyCode::Escape => ControlFlow::Break,
                _ => {
                    director.send(DirectorMsg::KeyPressed(keycode)).unwrap();
                    ControlFlow::Continue
                }
            }
        }
        WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. }, .. } => {
            ControlFlow::Break
        }

        WindowEvent::Resized(width, height) => {
            local_screen.on_dimensions(width, height);
            director.send(DirectorMsg::ScreenResized(width, height)).unwrap();
            ControlFlow::Continue
        }
        WindowEvent::Refresh => {
            local_screen.draw();
            ControlFlow::Continue
        }
        WindowEvent::CursorMoved { position, .. } => {
            local_screen.move_tracking(position);
            ControlFlow::Continue
        }
        WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
            match state {
                ElementState::Pressed => local_screen.begin_tracking(),
                ElementState::Released => local_screen.end_tracking(),
            }
            ControlFlow::Continue
        }
        _ => ControlFlow::Continue,
    }
}

pub struct LocalScreen<'a> {
    dimensions: (f32, f32),
    director: Sender<DirectorMsg>,
    blocks: HashMap<u64, Block>,
    patch_renderer: PatchRenderer,
    shadow_renderer: ShadowRenderer,
    quip_renderer: QuipRenderer<'a>,
    display: Display,
    status: ScreenStatus,
    cursor: (f64, f64),
    touch_destination: Option<u64>,
}

impl<'a> LocalScreen<'a> {
    fn new(width: u32, height: u32, events_loop: &EventsLoop, director: Sender<DirectorMsg>) -> Self {
        let display = get_display(width, height, events_loop);
        let modelview = get_modelview(width, height, &display);
        let dpi_factor = display.gl_window().hidpi_factor();
        let local_screen = LocalScreen {
            dimensions: (width as f32, height as f32),
            director,
            blocks: HashMap::<u64, Block>::new(),
            patch_renderer: PatchRenderer::new(&display, modelview),
            shadow_renderer: ShadowRenderer::new(&display, modelview),
            quip_renderer: QuipRenderer::new(dpi_factor, modelview, &display),
            display,
            status: ScreenStatus::Changed,
            cursor: (-1.0, -1.0),
            touch_destination: None,
        };
        local_screen
    }

    fn status(&self) -> ScreenStatus {
        self.status
    }

    fn send_touch(&self, touch_msg: TouchMsg) {
        self.director.send(DirectorMsg::TouchMsg(touch_msg)).unwrap();
    }

    fn begin_tracking(&mut self) {
        self.cancel_tracking();
        let (x, y) = self.cursor;
        let some_block = self.blocks.iter().find(|&(_, block)| {
            match block.sigil {
                Sigil::Touch(_) if block.is_hit(x as f32, y as f32) => true,
                _ => false,
            }
        });
        if let Some((_, &Block { sigil: Sigil::Touch(tag), .. })) = some_block {
            self.touch_destination = Some(tag);
            self.send_touch(TouchMsg::Begin(tag, x, y));
        }
    }

    fn move_tracking(&mut self, cursor: (f64, f64)) {
        self.cursor = cursor;
        if let Some(tag) = self.touch_destination {
            let (x, y) = self.cursor;
            self.send_touch(TouchMsg::Move(tag, x, y));
        }
    }

    fn cancel_tracking(&mut self) {
        if let Some(tag) = self.touch_destination {
            self.send_touch(TouchMsg::Cancel(tag));
        }
        self.touch_destination = None;
    }

    fn end_tracking(&mut self) {
        if let Some(tag) = self.touch_destination {
            let (x, y) = self.cursor;
            self.send_touch(TouchMsg::End(tag, x, y));
        }
        self.touch_destination = None;
    }

    fn on_dimensions(&mut self, width: u32, height: u32) {
        let modelview = get_modelview(width, height, &self.display);
        self.dimensions = (width as f32, height as f32);
        self.patch_renderer.set_modelview(modelview);
        self.shadow_renderer.set_modelview(modelview);
        self.quip_renderer.set_modelview(modelview);
        self.draw();
    }

    pub fn update(&mut self, screen_message: ScreenMsg) {
        match screen_message {
            ScreenMsg::AddBlock(id, block) => {
                let blocks = &mut self.blocks;
                blocks.insert(id, block);
                self.status = self.status.did_change()
            }
            ScreenMsg::Close => {
                self.status = self.status.will_close()
            }
        }
    }

    fn draw(&mut self) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.70, 0.80, 0.90, 1.0), 1.0);
        self.draw_patches(&mut target);
        self.draw_quips(&mut target);
        target.finish().unwrap();
        self.status = self.status.did_draw();
    }

    fn draw_quips(&mut self, target: &mut Frame) {
        let quip_renderer = &mut self.quip_renderer;
        let blocks = &self.blocks;
        let dpi_factor = self.display.gl_window().hidpi_factor();
        let display = &self.display;
        blocks.iter().for_each(|(_, block)| {
            if let Sigil::Paragraph { line_height, ref text, ref color, placement } = block.sigil {
                quip_renderer.layout_paragraph(
                    text,
                    block.anchor.into(),
                    Scale::uniform(line_height * dpi_factor),
                    block.width as u32,
                    block.approach,
                    color.to_gl(),
                    placement,
                    display,
                );
                quip_renderer.draw(target);
            }
        });
    }

    fn draw_patches(&mut self, target: &mut Frame) {
        let patch_renderer = &mut self.patch_renderer;
        let shadow_renderer = &mut self.shadow_renderer;
        let dimensions = self.dimensions;
        let blocks = &self.blocks;
        blocks.iter().for_each(|(_, block)| {
            if let Sigil::Color(color) = block.sigil {
                let patch = Patch::new(block.anchor.into(), block.width, block.height, block.approach, color);
                patch_renderer.set_patch(&patch);
                patch_renderer.draw(target);
                shadow_renderer.set_patch(&patch, dimensions);
                shadow_renderer.draw(target);
            }
        });
    }
}

pub enum AwakenMessage {
    ScreenMessage(ScreenMsg)
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

fn spawn_awakener(events_loop: &EventsLoop, awaken_message_sender: Sender<AwakenMessage>, screen_msg_receiver: Receiver<ScreenMsg>) {
    let events_loop_proxy = events_loop.create_proxy();
    thread::spawn(move || {
        let mut done = false;
        while !done {
            let result = screen_msg_receiver.recv();
            match result {
                Ok(msg) => {
                    awaken_message_sender.send(AwakenMessage::ScreenMessage(msg)).unwrap();
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
    let context_builder = ContextBuilder::new()
        .with_multisampling(4)
        .with_depth_buffer(24)
        .with_vsync(true);
    let window_builder = WindowBuilder::new()
        .with_dimensions(width, height)
        .with_title("PatchGL");
    Display::new(window_builder, context_builder, events_loop).unwrap()
}

fn get_modelview<F: Facade>(screen_width: u32, screen_height: u32, display: &F) -> [[f32; 4]; 4] {
    let (window_width, window_height) = display.get_context().get_framebuffer_dimensions();
    let screen_aspect = screen_width as f32 / screen_height as f32;
    let window_aspect = window_width as f32 / window_height as f32;
    let ndc_width = 2.0f32 * screen_aspect / window_aspect;
    const NDC_HEIGHT: f32 = 2.0f32;
    const NDC_APPROACH: f32 = 2.0f32;
    const NDC_APPROACH_PER_PIXEL: f32 = (1.0f32 / SCREEN_APPROACH) * NDC_APPROACH;
    [
        [1.0 / screen_width as f32 * ndc_width, 0.0, 0.0, 0.0],
        [0.0, -1.0 / screen_height as f32 * NDC_HEIGHT, 0.0, 0.0],
        [0.0, 0.0, -NDC_APPROACH_PER_PIXEL, 0.0],
        [-ndc_width / 2.0, NDC_HEIGHT / 2.0, NDC_APPROACH / 2.0 - NDC_APPROACH_PER_PIXEL, 1.0f32],
    ]
}
