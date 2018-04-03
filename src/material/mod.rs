use ::app::Palette;
use ::button;
pub use ::button::Kind;
use ::flood::Flood;
pub use self::color::*;
pub use self::model::Model;

mod color;

pub struct Button<'a, MsgT> {
    pub id: u64,
    pub model: &'a Model,
    pub kind: Kind,
    pub _click_msg: MsgT,
}

pub fn button<F:, MsgT>(wrap: F, palette: &Palette, button: Button<MsgT>) -> Flood<MsgT> where
    F: Fn(Msg) -> MsgT + Send + Sync + 'static,
    MsgT: Send + Sync + 'static,
{
    let button_model = button.model.get_button_model(button.id);
    let button_wrap = {
        let button_id = button.id;
        move |msg: button::Msg| {
            wrap(Msg::ButtonMsg(button_id, msg))
        }
    };
    button::flood(button_wrap, palette, button::Button {
        id: button.id,
        kind: button.kind,
        model: button_model,
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

    #[derive(Clone, Eq, PartialEq, Debug, Default)]
    pub struct Model {
        pub button_models: HashMap<u64, button::Model>
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
