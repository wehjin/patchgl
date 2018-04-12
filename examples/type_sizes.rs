extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::flood::*;
use patchgl::traits::*;

fn main() {
    use patchgl::app;
    app::run(480, 800, "Type sizes", AppMdl::default());
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
        let text_color = palette.dark_background_text_primary;
        Flood::Color(palette.transparent)
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(14.1)), Flood::Text("Medium (All Caps) 14".to_uppercase(), text_color, Placement::Start))
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(12.0)), Flood::Text("Regular 12".into(), text_color, Placement::Start))
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(14.9)), Flood::Text("Regular 14".into(), text_color, Placement::Start))
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(14.8)), Flood::Text("Medium 14".into(), text_color, Placement::Start))
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(16.7)), Flood::Text("Regular 16".into(), text_color, Placement::Start))
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(20.6)), Flood::Text("Medium 20".into(), text_color, Placement::Start))
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(24.5)), Flood::Text("Regular 24".into(), text_color, Placement::Start))
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(34.4)), Flood::Text("Regular 34".into(), text_color, Placement::Start))
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(45.3)), Flood::Text("Regular 45".into(), text_color, Placement::Start))
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(56.2)), Flood::Text("Regular 56".into(), text_color, Placement::Start))
            + (Position::Top(Length::Spacing), Flood::Color(palette.transparent))
            + (Position::Top(Length::Pixels(112.1)), Flood::Text("Light 112".into(), text_color, Placement::Start))
            + Padding::Uniform(Length::Spacing)
            + Flood::Color(palette.dark_background)
    }
}