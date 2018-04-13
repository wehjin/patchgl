extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::app::App;
use patchgl::flood::*;
use patchgl::material;
use patchgl::window;

fn main() {
    window::start(640, 400, |window| {
        let app = App::new(update, draw);
        app.run("Counter", Model::default(), window);
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
        AppMsg::MaterialMsg(material_msg) => {
            material::update(&mut model.material, material_msg);
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Model {
    pub count: i32,
    pub material: material::Model,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            count: 0,
            material: material::Model::default(),
        }
    }
}

#[derive(Clone, Debug)]
enum AppMsg {
    Up,
    Down,
    Reset,
    MaterialMsg(material::Msg),
}

fn draw(model: &Model) -> Flood<AppMsg> {
    let edge_padding = Padding::Uniform(Length::Spacing);
    let palette = &model.material.palette;
    let background = Flood::Color(palette.light_background);

    let text = format!("{:+}", model.count);
    let body = Flood::Text(text, palette.primary, Placement::Center);
    let bottom_bar = {
        let mut buttons = Vec::new();
        buttons.push(material::button(material::Button {
            msg_wrap: AppMsg::MaterialMsg,
            id: 32,
            model: &model.material,
            kind: material::ButtonKind::ColoredFlat("Down".into()),
            click_msg: AppMsg::Down,
        }));
        buttons.push(material::button(material::Button {
            msg_wrap: AppMsg::MaterialMsg,
            id: 33,
            model: &model.material,
            kind: material::ButtonKind::PlainFlat("Reset".into()),
            click_msg: AppMsg::Reset,
        }));
        buttons.push(material::button(material::Button {
            msg_wrap: AppMsg::MaterialMsg,
            id: 34,
            model: &model.material,
            kind: material::ButtonKind::ColoredFlat("Up".into()),
            click_msg: AppMsg::Up,
        }));
        draw_bar(buttons)
    };
    body + (Position::Bottom(Length::FingerTip), bottom_bar) + edge_padding + background
}

fn draw_bar(contents: Vec<Flood<AppMsg>>) -> Flood<AppMsg> {
    use ::patchgl::color::argb::TRANSPARENT;
    let enumeration = contents.into_iter().enumerate().collect::<Vec<_>>();
    enumeration.into_iter().fold(
        Flood::Color(TRANSPARENT),
        |bar, (i, button)| {
            let segment = button + Padding::Horizontal(Length::Spacing / 4);
            bar + (Position::Right(Length::Full / (i as u32 + 1)), segment)
        })
}
