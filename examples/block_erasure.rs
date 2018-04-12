extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::material::components::button;
use patchgl::Color;
use patchgl::flood::*;
use patchgl::traits::*;
use patchgl::window;

fn main() {
    window::start(320, 400, |window| {
        use patchgl::app::App;

        let app = App::new(AppMdl::update, AppMdl::draw);
        app.run("Block Erasure", AppMdl::default(), window);
    });
}

#[derive(Clone, PartialEq, Debug)]
struct AppMdl {
    pub button_mdl: button::Model,
    pub state: State,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    MultiBlock,
    SingleBlock,
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum AppMsg {
    Toggle,
    ButtonMsg(button::Msg),
}

impl Default for AppMdl {
    fn default() -> Self {
        AppMdl {
            button_mdl: button::Model::default(),
            state: State::MultiBlock,
        }
    }
}

impl Update<AppMsg> for AppMdl {
    fn update(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::Toggle => {
                self.state = match self.state {
                    State::MultiBlock => State::SingleBlock,
                    State::SingleBlock => State::MultiBlock,
                }
            }
            AppMsg::ButtonMsg(msg) => {
                button::update(&mut self.button_mdl, msg);
            }
        }
    }
}

impl Draw<AppMsg> for AppMdl {
    fn draw(&self) -> Flood<AppMsg> {
        let palette = patchgl::material::Palette::default();
        let button = button::flood(AppMsg::ButtonMsg, button::Button {
            id: 11,
            kind: button::Kind::ColoredFlat("Toggle".into()),
            model: self.button_mdl.clone(),
            click_msg: AppMsg::Toggle,
            palette: &palette,
        });
        match self.state {
            State::MultiBlock => {
                Flood::Color(Color::blue())
                    + (Position::Right(Length::Full / 2), Flood::Color(Color::grey()))
                    + (Position::Right(Length::Full / 3), Flood::Color(Color::green()))
                    + (Position::Right(Length::Full / 4), Flood::Color(Color::red()))
                    + (Position::Right(Length::Full / 5), Flood::Color(Color::black()))
                    + Padding::Uniform(Length::Full * 0.25)
                    + (Position::Bottom(Length::Spacing * 3), button)
                    + Flood::Color(Color::white())
            }
            State::SingleBlock => {
                Flood::Color(Color::custom_white(0.8))
                    + Padding::Uniform(Length::Full * 0.3)
                    + (Position::Bottom(Length::Spacing * 3), button)
                    + Flood::Color(Color::custom_white(0.2))
            }
        }
    }
}