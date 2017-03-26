use glium;
use glium::DisplayBuild;
use std::rc::Rc;

pub struct Screen {
    pub display: Rc<glium::backend::glutin_backend::GlutinFacade>,
    pub width: u32,
    pub height: u32
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        Screen {
            display: Rc::new(glium::glutin::WindowBuilder::new()
                .with_dimensions(width, height)
                .with_title("PatchGl")
                .with_vsync()
                .build_glium().unwrap()),
            width: width,
            height: height
        }
    }

    pub fn dpi_factor(&self) -> f32 {
        self.display.get_window().unwrap().hidpi_factor()
    }
}