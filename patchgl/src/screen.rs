use model::Patchwork;
use glium;
use glium::DisplayBuild;

pub struct Screen {
    pub display: glium::backend::glutin_backend::GlutinFacade
}

impl Screen {
    pub fn new(patchwork: &Patchwork) -> Self {
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(patchwork.width as u32, patchwork.height as u32)
            .with_title("PatchGl")
            .with_vsync()
            .build_glium().unwrap();
        Screen {
            display: display
        }
    }

    pub fn dpi_factor(&self) -> f32 {
        self.display.get_window().unwrap().hidpi_factor()
    }
}