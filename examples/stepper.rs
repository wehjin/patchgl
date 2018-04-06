extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

fn main() {
    use patchgl::window;
    window::start(640, 400, |window| {
        use patchgl::app::App;

        let app = App::new(update, draw);
        app.run("Stepper", Mdl::default(), window);
    });
}


#[derive(Clone, Eq, PartialEq, Debug)]
enum Msg {}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Mdl {}

use patchgl::flood::*;

fn draw(_mdl: &Mdl) -> Flood<Msg> {
    use patchgl::material::Palette;
    use patchgl::material::components::stepper::*;

    let palette = Palette::default();

    let stepper: Flood<Msg> = Stepper {
        palette: &palette,
        id: vec![1],
        active_index: 0,
        steps: vec![
            Step { label: "Fee" },
            Step { label: "Fi" },
            Step { label: "Fo" },
            Step { label: "Fum" },
        ],
    }.into();

    Flood::Color(palette.light_background_raised) + Padding::Behind(Length::CardApproach)
        + (Position::Top(Length::Spacing / 2), Flood::Color(palette.transparent))
        + (Position::Top(Length::Full * 0.15), stepper)
        + Padding::Uniform(Length::Spacing * 1.5)
        + (Stratum::JustBelow, Flood::Color(palette.light_background))
}

fn update(_mdl: &mut Mdl, _msg: Msg) {}
