use flood::Flood;
use traits::Update;
pub use window::TouchMsg;
pub use flood::Placement;
use self::components::button;
pub use self::components::button::{ButtonStyle, ButtonKind};
pub use self::color::*;
pub use self::model::Model;
pub use self::palette::Palette;
pub use self::length::*;

mod color;
mod length;
pub mod entry;
pub mod palette;
pub mod components;

pub struct Button<'a, F, MsgT> where
    F: Fn(Msg) -> MsgT + Send + Sync + 'static,
{
    pub msg_wrap: F,
    pub id: u64,
    pub model: &'a Model,
    pub style: Vec<ButtonStyle>,
    pub click_msg: MsgT,
}

pub fn button<'a, F, MsgT>(button: Button<'a, F, MsgT>) -> Flood<MsgT> where
    F: Fn(Msg) -> MsgT + Send + Sync + 'static,
    MsgT: Clone + Send + Sync + 'static,
{
    let button_model = button.model.get_button_model(button.id);
    let button_wrap = {
        let button_id = button.id;
        let button_msg_wrap = button.msg_wrap;
        move |msg: button::ButtonMsg| {
            (button_msg_wrap)(Msg::ButtonMsg(button_id, msg))
        }
    };
    button::Button {
        msg_wrap: button_wrap,
        id: button.id,
        palette: &button.model.palette,
        mdl: &button_model,
        style: button.style,
        click_msg: button.click_msg,
    }.into()
}

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::None => {}
        Msg::ButtonMsg(button_id, button_msg) => {
            let mut button_model = model.get_button_model(button_id);
            button_model.update(button_msg);
            model.set_button_model(button_id, button_model);
        }
    }
}

mod model {
    use super::components::button;
    use std::collections::HashMap;
    use super::Palette;

    #[derive(Clone, PartialEq, Debug, Default)]
    pub struct Model {
        pub button_models: HashMap<u64, button::ButtonMdl>,
        pub palette: Palette,
    }

    impl Model {
        pub fn get_button_model(&self, tag: u64) -> button::ButtonMdl {
            match self.button_models.get(&tag) {
                Some(model) => *model,
                None => button::ButtonMdl::default(),
            }
        }
        pub fn set_button_model(&mut self, tag: u64, button_model: button::ButtonMdl) {
            let button_models = &mut self.button_models;
            button_models.insert(tag, button_model);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Msg {
    None,
    ButtonMsg(u64, button::ButtonMsg),
}
