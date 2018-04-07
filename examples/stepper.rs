extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

fn main() {
    use patchgl::window;
    window::start(768, 768, |window| {
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
        active_index: 1,
        active_content: Flood::Color(palette.primary),
        steps: vec![
            Step { label: "Fee" },
            Step { label: "Fi" },
            Step { label: "Fo" },
            Step { label: "Fum" },
        ],
    }.into();
    stepper
        + Padding::Uniform(Length::Spacing * 1.5)
        + (Stratum::JustBelow, Flood::Color(palette.light_background))
}

fn update(_mdl: &mut Mdl, _msg: Msg) {}
