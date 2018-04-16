extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::material::components::button::*;
use patchgl::material;
use patchgl::flood::*;
use patchgl::flood::Placement;
use patchgl::traits::*;
use patchgl::window;

fn main() {
    window::start(640, 400, |window| {
        use patchgl::app::App;
        let app = App::new(AppMdl::update, AppMdl::draw);
        app.run("Buttons", AppMdl::default(), window);
    });
}

#[derive(Clone, PartialEq, Debug)]
struct AppMdl {
    pub button_mdl: ButtonMdl,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum AppMsg {
    ButtonMsg(ButtonMsg),
    None,
}

impl Default for AppMdl {
    fn default() -> Self {
        AppMdl {
            button_mdl: ButtonMdl::default(),
        }
    }
}

impl Update<AppMsg> for AppMdl {
    fn update(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::ButtonMsg(msg) => {
                self.button_mdl.update(msg);
            }
            AppMsg::None => {}
        }
    }
}

impl Draw<AppMsg> for AppMdl {
    fn draw(&self) -> Flood<AppMsg> {
        let palette = patchgl::material::Palette::default();
        let button_label = "Plain Flat";
        let light_plain_flat = Flood::<AppMsg>::from(Button {
            msg_wrap: AppMsg::ButtonMsg,
            id: 11,
            palette: &palette,
            mdl: &self.button_mdl,
            kind: ButtonKind::PlainFlat(button_label.into()),
            placement: Placement::Center,
            click_msg: AppMsg::None,
        });
        let light_panel = light_plain_flat
            + Padding::Dual((Length::Full - material::Length::ButtonWidth(button_label.into()).into()) * 0.5,
                            (Length::Full - material::Length::ButtonHeight.into()) * 0.5)
            + Flood::Color(palette.light_background_raised);

        let dark_plain_flat = Flood::<AppMsg>::from(Button {
            msg_wrap: AppMsg::ButtonMsg,
            id: 11,
            palette: &palette,
            mdl: &self.button_mdl,
            kind: ButtonKind::PlainFlat(button_label.into()),
            placement: Placement::Center,
            click_msg: AppMsg::None,
        });
        let dark_panel = dark_plain_flat
            + Padding::Dual((Length::Full - material::Length::ButtonWidth(button_label.into()).into()) * 0.5,
                            (Length::Full - material::Length::ButtonHeight.into()) * 0.5)
            + Flood::Color(palette.dark_background_raised);

        light_panel + (Position::Right(Length::Full * 0.5), dark_panel)
    }
}
