use patchgl::Color;
use patchgl::dervish::*;
use patchgl::flood::*;
use super::app::Palette;

pub fn update(mdl: &mut Mdl, msg: Msg) -> Option<Note> {
    match msg {
        Msg::Press => {
            mdl.press_state = PressState::Down;
            None
        }
        Msg::Unpress => {
            mdl.press_state = PressState::Up;
            None
        }
        Msg::Release => {
            if mdl.press_state == PressState::Down {
                mdl.press_state = PressState::Up;
                Some(Note::Clicked)
            } else {
                None
            }
        }
    }
}

pub fn draw<MsgT>(mdl: &Mdl, palette: &Palette) -> Flood<MsgT> {
    match (&mdl.kind, &mdl.press_state) {
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

pub fn special<MsgT>(palette: &Palette) -> Flood<MsgT> {
    use std::sync::Arc;
    let builder = Arc::new(|| {
        use std::thread;
        use std::sync::mpsc::channel;
        let (dervish, dervish_msgs) = channel::<DervishMsg>();
        thread::spawn(move || {
            while let Ok(msg) = dervish_msgs.recv() {
                match msg {}
            }
        });
        dervish
    });
    Flood::Dervish(Dervish::Builder(builder))
}

fn flat_button_surface<MsgT>(label: &str, text_color: Color) -> Flood<MsgT> {
    let text = Flood::Text(label.to_uppercase(), text_color, Placement::Center);
    let padding = Padding::Dual(Length::Spacing, Length::Full / 4);
    text + padding
}


#[derive(Clone, Debug)]
pub struct Mdl {
    pub kind: Kind,
    pub press_state: PressState,
}

impl Mdl {
    pub fn colored_flat(label: &str) -> Self {
        Mdl::new(Kind::ColoredFlat(String::from(label)))
    }

    pub fn plain_flat(label: &str) -> Self {
        Mdl::new(Kind::PlainFlat(String::from(label)))
    }

    pub fn new(kind: Kind) -> Self {
        Mdl { kind, press_state: PressState::Up }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Msg {
    Press,
    Unpress,
    Release,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Note {
    Clicked,
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
