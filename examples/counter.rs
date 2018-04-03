extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::app::App;
use patchgl::app::Palette;
use patchgl::button;
use patchgl::flood::*;
use patchgl::window;

fn main() {
    window::start(640, 400, |window| {
        let app = App::new(update, draw);
        app.run(Model::default(), window);
    });
}

fn update(model: &mut Model, msg: AppMsg) {
    match msg {
        AppMsg::Up => {
            model.count += 1;
        }
        AppMsg::Down => {
            model.count -= 1;
        }
        AppMsg::Reset => {
            model.count = 0;
        }
        AppMsg::UpButtonMsg(button_msg) => {
            if let Some(button::Note::Clicked(_)) = button::update(&mut model.up_button, button_msg) {
                update(model, AppMsg::Up);
            }
        }
        AppMsg::DownButtonMsg(button_msg) => {
            if let Some(button::Note::Clicked(_)) = button::update(&mut model.down_button, button_msg) {
                update(model, AppMsg::Down);
            }
        }
        AppMsg::ResetButtonMsg(button_msg) => {
            if let Some(button::Note::Clicked(_)) = button::update(&mut model.reset_button, button_msg) {
                update(model, AppMsg::Reset);
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Model {
    pub count: i32,
    pub up_button: button::Model,
    pub down_button: button::Model,
    pub reset_button: button::Model,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            count: 0,
            up_button: button::Model::default(),
            down_button: button::Model::default(),
            reset_button: button::Model::default(),
        }
    }
}

enum AppMsg {
    Up,
    Down,
    Reset,
    UpButtonMsg(button::Msg),
    DownButtonMsg(button::Msg),
    ResetButtonMsg(button::Msg),
}

fn draw(model: &Model, palette: &Palette) -> Flood<AppMsg> {
    let edge_padding = Padding::Uniform(Length::Spacing);
    let background = Flood::Color(palette.light_background);

    let text = format!("{:+}", model.count);
    let body = Flood::Text(text, palette.primary, Placement::Center);
    let bottom_bar = {
        let mut buttons = Vec::new();
        buttons.push(button::flood(AppMsg::DownButtonMsg, palette, button::Button {
            id: 32,
            kind: button::Kind::ColoredFlat("Down".into()),
            model: model.down_button,
        }));
        buttons.push(button::flood(AppMsg::ResetButtonMsg, palette, button::Button {
            id: 33,
            kind: button::Kind::PlainFlat("Reset".into()),
            model: model.reset_button,
        }));
        buttons.push(button::flood(AppMsg::UpButtonMsg, palette, button::Button {
            id: 34,
            kind: button::Kind::ColoredFlat("Up".into()),
            model: model.up_button,
        }));
        draw_bar(buttons, palette)
    };
    body + (Position::Bottom(Length::FingerTip), bottom_bar) + edge_padding + background
}

fn draw_bar(contents: Vec<Flood<AppMsg>>, palette: &Palette) -> Flood<AppMsg> {
    let enumeration = contents.into_iter().enumerate().collect::<Vec<_>>();
    enumeration.into_iter().fold(
        Flood::Color(palette.light_background),
        |bar, (i, button)| {
            let segment = button + Padding::Horizontal(Length::Spacing / 4);
            bar + (Position::Right(Length::Full / (i as u32 + 1)), segment)
        })
}
