use ::{Block, ScreenMessage, ScreenStatus, Sigil};
use glium::{Display, Surface};
use glium::backend::Facade;
use glyffin::QuipRenderer;
use model::Patch;
use renderer::PatchRenderer;
use rusttype::Scale;
use std::collections::HashMap;

pub struct LocalScreen<'a> {
    blocks: HashMap<u64, Block>,
    patch_renderer: PatchRenderer,
    quip_renderer: QuipRenderer<'a>,
    display: Display,
    status: ScreenStatus,
}

impl<'a> LocalScreen<'a> {
    pub fn new(display: Display) -> Self {
        let (width, height) = display.get_framebuffer_dimensions();
        let modelview = get_modelview(width, height, &display);
        let dpi_factor = display.gl_window().hidpi_factor();
        LocalScreen {
            blocks: HashMap::<u64, Block>::new(),
            patch_renderer: PatchRenderer::new(&display, modelview),
            quip_renderer: QuipRenderer::new(dpi_factor, modelview, &display),
            display,
            status: ScreenStatus::Changed,
        }
    }

    pub fn status(&self) -> ScreenStatus {
        self.status
    }

    pub fn resize(&mut self, width: u32, height: u32) {
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

    pub fn draw(&mut self) {
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
