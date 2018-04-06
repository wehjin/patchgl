use ::button;
pub use ::button::Kind;
use ::flood::Flood;
pub use self::color::*;
pub use self::model::Model;
pub use self::palette::Palette;

mod color;
pub mod entry;
pub mod palette;

pub struct Button<'a, F, MsgT> where
    F: Fn(Msg) -> MsgT + Send + Sync + 'static,
{
    pub msg_wrap: F,
    pub id: u64,
    pub model: &'a Model,
    pub kind: Kind,
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
        move |msg: button::Msg| {
            (button_msg_wrap)(Msg::ButtonMsg(button_id, msg))
        }
    };
    button::flood(button_wrap, button::Button {
        id: button.id,
        kind: button.kind,
        model: button_model,
        click_msg: button.click_msg,
        palette: &button.model.palette,
    })
}

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::None => {}
        Msg::ButtonMsg(button_id, button_msg) => {
            let mut button_model = model.get_button_model(button_id);
            button::update(&mut button_model, button_msg);
            model.set_button_model(button_id, button_model);
        }
    }
}

mod model {
    use ::button;
    use std::collections::HashMap;
    use super::Palette;

    #[derive(Clone, PartialEq, Debug, Default)]
    pub struct Model {
        pub button_models: HashMap<u64, button::Model>,
        pub palette: Palette,
    }

    impl Model {
        pub fn get_button_model(&self, tag: u64) -> button::Model {
            match self.button_models.get(&tag) {
                Some(model) => *model,
                None => button::Model::default(),
            }
        }
        pub fn set_button_model(&mut self, tag: u64, button_model: button::Model) {
            let button_models = &mut self.button_models;
            button_models.insert(tag, button_model);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Msg {
    None,
    ButtonMsg(u64, button::Msg),
}
