extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::Color;
use patchgl::window;
use patchgl::flood::*;

fn main() {
    window::start(320, 400, |window| {
        use patchgl::app::App;

        let app = App::new(update, draw);
        app.run("Timeout", Model::default(), window);
    });
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Msg {
    Next
}

#[derive(Clone, PartialEq, Debug)]
struct Model {
    pub colors: [Color; 3],
    pub active_color: usize,
    pub timer_version_counter: VersionCounter,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            colors: [Color::blue(), Color::red(), Color::green()],
            active_color: 0,
            timer_version_counter: VersionCounter::enabled(),
        }
    }
}

fn update(model: &mut Model, _msg: Msg) {
    model.active_color += 1;
    model.timer_version_counter.bump();
}

fn draw(model: &Model) -> Flood<Msg> {
    let color_index = model.active_color % model.colors.len();
    let color = model.colors[color_index].clone();
    let panel = Flood::Color(color);
    let timeout = Timeout {
        id: 55,
        msg: Msg::Next,
        duration: Duration::Seconds(1),
    };
    let versioned_timeout = Version::from((timeout, model.timer_version_counter.clone()));
    let sensor = Sensor::Timeout(versioned_timeout);
    panel + sensor + Padding::Dual(Length::Full * 0.25, Length::Full * 0.25)
}

