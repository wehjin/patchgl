use ::app::Palette;
use ::Color;
use ::flood::*;
use ::flood::Signal;
use ::flood::VersionCounter;
use ::TouchMsg;
use std::sync::Arc;

#[derive(Clone)]
pub struct Button<MsgT> {
    pub id: u64,
    pub kind: Kind,
    pub model: Model,
    pub click_msg: MsgT,
}

pub fn flood<F, MsgT>(wrap: F, palette: &Palette, button: Button<MsgT>) -> Flood<MsgT> where
    MsgT: Clone + Send + Sync + 'static,
    F: Fn(Msg) -> MsgT + Send + Sync + 'static
{
    let surface = draw(&button, palette);
    let touch_sensor = {
        let button_id = button.id;
        Sensor::Touch(button_id, Arc::new(move |touch_msg| {
            let msg = if touch_msg.tag() == button_id {
                match touch_msg {
                    TouchMsg::Begin(_, _, _) => Msg::Press,
                    TouchMsg::End(tag, _, _) => Msg::Release(tag),
                    TouchMsg::Move(_, _, _) => Msg::None,
                    TouchMsg::Cancel(_) => Msg::Unpress,
                }
            } else {
                Msg::None
            };
            wrap(msg)
        }))
    };
    let signal_sensor = {
        let click_msg_version: Version<MsgT> = (button.click_msg, button.model.click_msg_version_counter).into();
        let signal = Signal::from((button.id, click_msg_version));
        Sensor::Signal(signal)
    };
    surface + touch_sensor + signal_sensor
}

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Press => {
            model.press_state = PressState::Down;
        }
        Msg::Unpress => {
            model.press_state = PressState::Up;
        }
        Msg::Release(_tag) => {
            if model.press_state == PressState::Down {
                model.press_state = PressState::Up;
                model.click_msg_version_counter.bump();
            }
        }
        Msg::None => {}
    }
}

fn draw<MsgT>(button: &Button<MsgT>, palette: &Palette) -> Flood<MsgT> where
    MsgT: Clone
{
    match (&button.kind, &button.model.press_state) {
        (&Kind::ColoredFlat(ref label), &PressState::Up) => {
            flat_button_surface(label, palette.secondary)
        }
        (&Kind::ColoredFlat(ref label), &PressState::Down) => {
            let surface = flat_button_surface(label, palette.secondary);
            let background = Flood::Color(palette.light_background_divider);
            surface + background
        }
        (&Kind::PlainFlat(ref label), &PressState::Up) => {
            flat_button_surface(label, palette.light_background_text_primary)
        }
        (&Kind::PlainFlat(ref label), &PressState::Down) => {
            let surface = flat_button_surface(label, palette.light_background_text_primary);
            let background = Flood::Color(palette.light_background_divider);
            surface + background
        }
    }
}

fn flat_button_surface<MsgT>(label: &str, text_color: Color) -> Flood<MsgT> where
    MsgT: Clone
{
    let text = Flood::Text(label.to_uppercase(), text_color, Placement::Center);
    let padding = Padding::Dual(Length::Spacing, Length::Full / 4);
    text + padding
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Model {
    pub press_state: PressState,
    pub click_msg_version_counter: VersionCounter,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            press_state: PressState::Up,
            click_msg_version_counter: VersionCounter::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Msg {
    Press,
    Unpress,
    Release(u64),
    None,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PressState {
    Up,
    Down,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Kind {
    PlainFlat(String),
    ColoredFlat(String),
}
