extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::flood::*;
use patchgl::material::components::step::*;

fn main() {
    use patchgl::window;
    window::start(320, 400, |window| {
        use patchgl::app::App;

        let app = App::new(update, draw);
        app.run("Stepper", Mdl::default(), window);
    });
}


#[derive(Clone, Eq, PartialEq, Debug)]
enum Msg {}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Mdl {}

fn draw(_mdl: &Mdl) -> Flood<Msg> {
    use patchgl::material::Palette;

    let palette = Palette::default();

    let content: Flood<Msg> = Step {
        id: vec![1],
        label: "First this",
        index: 0,
        condition: StepCondition::Active,
        palette: &palette,
    }.into();

    content
        + Padding::Dual(Length::Spacing * 1.5, Length::Full * 0.45)
        + (Stratum::JustBelow, Flood::Color(palette.light_background))
}

fn update(_mdl: &mut Mdl, _msg: Msg) {}
