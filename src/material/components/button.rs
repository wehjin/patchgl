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
    pub kind: ButtonKind,
    pub placement: Placement,
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
    match (&button.kind, &button.mdl.press_state) {
        (&ButtonKind::ColoredFlat(ref label), &PressState::Up) => {
            flat_button_surface(label, palette.secondary, button.placement)
        }
        (&ButtonKind::ColoredFlat(ref label), &PressState::Down) => {
            let surface = flat_button_surface(label, palette.secondary, button.placement);
            let background = Flood::Color(palette.light_background_divider);
            surface + background
        }
        (&ButtonKind::PlainFlat(ref label), &PressState::Up) => {
            flat_button_surface(label, palette.light_background_text_primary, button.placement)
        }
        (&ButtonKind::PlainFlat(ref label), &PressState::Down) => {
            let surface = flat_button_surface(label, palette.light_background_text_primary, button.placement);
            let background = Flood::Color(palette.light_background_divider);
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
    PlainFlat(String),
    ColoredFlat(String),
}
