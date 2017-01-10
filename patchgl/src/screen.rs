use glium;
use glium::DisplayBuild;

pub struct Screen {
    pub display: glium::backend::glutin_backend::GlutinFacade,
    pub width: u32,
    pub height: u32
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        Screen {
            display: glium::glutin::WindowBuilder::new()
                .with_dimensions(width, height)
                .with_title("PatchGl")
                .with_vsync()
                .build_glium().unwrap(),
            width: width,
            height: height
        }
    }

    pub fn dpi_factor(&self) -> f32 {
        self.display.get_window().unwrap().hidpi_factor()
    }
}