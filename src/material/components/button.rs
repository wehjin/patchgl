use Color;
use flood::*;
use flood::Signal;
use flood::VersionCounter;
use TouchMsg;
use std::sync::Arc;
use material::palette::Palette;
use traits::Update;
pub use flood::Placement;

#[derive(Clone)]
pub struct Button<'a, MsgT, F> where
    F: Fn(ButtonMsg) -> MsgT + Send + Sync + 'static,
{
    pub msg_wrap: F,
    pub id: u64,
    pub palette: &'a Palette,
    pub mdl: &'a ButtonMdl,
    pub style: Vec<ButtonStyle>,
    pub click_msg: MsgT,
}

impl<'a, MsgT, F> From<Button<'a, MsgT, F>> for Flood<MsgT> where
    MsgT: Clone,
    F: Fn(ButtonMsg) -> MsgT + Send + Sync + 'static,
{
    fn from(button: Button<MsgT, F>) -> Self {
        let surface = draw(&button);
        let touch_sensor = {
            let button_id = button.id;
            let button_msg_wrap = button.msg_wrap;
            Sensor::Touch(button_id, Arc::new(move |touch_msg| {
                let msg = if touch_msg.tag() == button_id {
                    match touch_msg {
                        TouchMsg::Begin(_, _, _) => ButtonMsg::Press,
                        TouchMsg::End(tag, _, _) => ButtonMsg::Release(tag),
                        TouchMsg::Move(_, _, _) => ButtonMsg::None,
                        TouchMsg::Cancel(_) => ButtonMsg::Unpress,
                    }
                } else {
                    ButtonMsg::None
                };
                button_msg_wrap(msg)
            }))
        };
        let signal_sensor = {
            let click_msg_version: Version<MsgT> = (button.click_msg, button.mdl.click_msg_version_counter).into();
            let signal = Signal::from((button.id, click_msg_version));
            Sensor::Signal(signal)
        };
        surface + touch_sensor + signal_sensor
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ButtonStyle {
    Placement(Placement),
    Kind(ButtonKind),
}

impl From<Placement> for ButtonStyle {
    fn from(placement: Placement) -> Self {
        ButtonStyle::Placement(placement)
    }
}

impl From<ButtonKind> for ButtonStyle {
    fn from(kind: ButtonKind) -> Self {
        ButtonStyle::Kind(kind)
    }
}

impl ButtonStyle {
    fn is_kind(&self) -> bool {
        match self {
            &ButtonStyle::Kind(_) => true,
            _ => false,
        }
    }

    fn is_placement(&self) -> bool {
        match self {
            &ButtonStyle::Placement(_) => true,
            _ => false,
        }
    }
}

impl<'a> From<&'a Vec<ButtonStyle>> for ButtonKind {
    fn from(style: &'a Vec<ButtonStyle>) -> Self {
        if let Some(&ButtonStyle::Kind(ref value)) = style.iter().find(|it| it.is_kind()) {
            value.clone()
        } else {
            ButtonKind::LightPlainFlat("Button".into())
        }
    }
}

impl<'a> From<&'a Vec<ButtonStyle>> for Placement {
    fn from(style: &'a Vec<ButtonStyle>) -> Self {
        if let Some(&ButtonStyle::Placement(value)) = style.iter().find(|it| it.is_placement()) {
            value.clone()
        } else {
            Placement::Center
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ButtonMdl {
    pub press_state: PressState,
    pub click_msg_version_counter: VersionCounter,
}

impl Default for ButtonMdl {
    fn default() -> Self {
        ButtonMdl {
            press_state: PressState::Up,
            click_msg_version_counter: VersionCounter::default(),
        }
    }
}

impl Update<ButtonMsg> for ButtonMdl {
    fn update(&mut self, msg: ButtonMsg) {
        match msg {
            ButtonMsg::Press => {
                self.press_state = PressState::Down;
            }
            ButtonMsg::Unpress => {
                self.press_state = PressState::Up;
            }
            ButtonMsg::Release(_tag) => {
                if self.press_state == PressState::Down {
                    self.press_state = PressState::Up;
                    self.click_msg_version_counter.bump();
                }
            }
            ButtonMsg::None => {}
        }
    }
}

fn draw<MsgT, F>(button: &Button<MsgT, F>) -> Flood<MsgT> where
    MsgT: Clone,
    F: Fn(ButtonMsg) -> MsgT + Send + Sync + 'static,
{
    let palette = button.palette;
    let kind: ButtonKind = From::from(&button.style);
    let placement: Placement = From::from(&button.style);
    match &button.mdl.press_state {
        &PressState::Up => {
            let label = kind.label();
            let text_color = text_color(&kind, palette);
            flat_button_surface(label, text_color, placement)
        }
        &PressState::Down => {
            let label = kind.label();
            let text_color = text_color(&kind, palette);
            let backing_color = backing_color(&kind, palette);
            let surface = flat_button_surface(label, text_color, placement);
            let background = Flood::Color(backing_color);
            surface + background
        }
    }
}

fn flat_button_surface<MsgT>(label: &str, text_color: Color, placement: Placement) -> Flood<MsgT> where
    MsgT: Clone
{
    let text = Flood::Text(label.to_uppercase(), text_color, placement);
    let padding = Padding::Dual(Length::Spacing, Length::Full / 4);
    text + padding
}

fn text_color(kind: &ButtonKind, palette: &Palette) -> Color {
    let text_color = match kind {
        &ButtonKind::LightPlainFlat(_) => palette.light_background_text_primary,
        &ButtonKind::LightColoredFlat(_) => palette.secondary,
        &ButtonKind::DarkPlainFlat(_) => palette.dark_background_text_primary,
        &ButtonKind::DarkColoredFlat(_) => palette.secondary_light,
    };
    text_color
}

fn backing_color(kind: &ButtonKind, palette: &Palette) -> Color {
    let text_color = match kind {
        &ButtonKind::LightPlainFlat(_) => palette.light_background_divider,
        &ButtonKind::LightColoredFlat(_) => palette.light_background_divider,
        &ButtonKind::DarkPlainFlat(_) => palette.dark_background_divider,
        &ButtonKind::DarkColoredFlat(_) => palette.dark_background_divider,
    };
    text_color
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonMsg {
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
pub enum ButtonKind {
    LightPlainFlat(String),
    LightColoredFlat(String),
    DarkPlainFlat(String),
    DarkColoredFlat(String),
}

impl ButtonKind {
    fn label(&self) -> &str {
        match *self {
            ButtonKind::LightPlainFlat(ref label) => label.as_str(),
            ButtonKind::LightColoredFlat(ref label) => label.as_str(),
            ButtonKind::DarkPlainFlat(ref label) => label.as_str(),
            ButtonKind::DarkColoredFlat(ref label) => label.as_str(),
        }
    }
}
