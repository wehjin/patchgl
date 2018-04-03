use patchgl::Color;
use patchgl::flood::*;
use patchgl::TouchMsg;
use std::sync::Arc;
use super::app::Palette;

pub struct Button {
    pub id: u64,
    pub kind: Kind,
    pub model: Model,
}

pub fn flood<F, UpMsgT>(wrap: F, palette: &Palette, button: Button) -> Flood<UpMsgT> where
    F: Fn(Msg) -> UpMsgT + Send + Sync + 'static
{
    let surface = draw(&button, palette);
    surface + Sensor::Touch(button.id, Arc::new(move |touch_msg| {
        wrap(if touch_msg.tag() == button.id {
            match touch_msg {
                TouchMsg::Begin(_, _, _) => Msg::Press,
                TouchMsg::End(tag, _, _) => Msg::Release(tag),
                TouchMsg::Move(_, _, _) => Msg::None,
                TouchMsg::Cancel(_) => Msg::Unpress,
            }
        } else {
            Msg::None
        })
    }))
}

pub fn update(mdl: &mut Model, msg: Msg) -> Option<Note> {
    match msg {
        Msg::Press => {
            mdl.press_state = PressState::Down;
            None
        }
        Msg::Unpress => {
            mdl.press_state = PressState::Up;
            None
        }
        Msg::Release(tag) => {
            if mdl.press_state == PressState::Down {
                mdl.press_state = PressState::Up;
                Some(Note::Clicked(tag))
            } else {
                None
            }
        }
        Msg::None => {
            None
        }
    }
}

pub fn draw<MsgT>(button: &Button, palette: &Palette) -> Flood<MsgT> {
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

fn flat_button_surface<MsgT>(label: &str, text_color: Color) -> Flood<MsgT> {
    let text = Flood::Text(label.to_uppercase(), text_color, Placement::Center);
    let padding = Padding::Dual(Length::Spacing, Length::Full / 4);
    text + padding
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Model {
    pub press_state: PressState,
}

impl Default for Model {
    fn default() -> Self {
        Model { press_state: PressState::Up }
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
pub enum Note {
    Clicked(u64),
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
