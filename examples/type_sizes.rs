extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::flood::*;
use patchgl::traits::*;

fn main() {
    use patchgl::app;
    app::run(480, 800, "Typography", AppMdl::default());
}

#[derive(Clone, PartialEq, Debug)]
struct AppMdl {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum AppMsg {}

impl Default for AppMdl {
    fn default() -> Self { AppMdl {} }
}

impl Update<AppMsg> for AppMdl {
    fn update(&mut self, msg: AppMsg) {
        match msg {}
    }
}

impl Draw<AppMsg> for AppMdl {
    fn draw(&self) -> Flood<AppMsg> {
        let palette = patchgl::material::Palette::default();
        Flood::Color(palette.light_background)
    }
}