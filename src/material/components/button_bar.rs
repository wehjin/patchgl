pub struct Button<MsgT> {
    pub id: u64,
    pub label: String,
    pub intent: ButtonIntent,
    pub click_msg: MsgT,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonIntent {
    Call,
    Provide,
    Inform,
}

pub struct ButtonBar<'a, MsgT, F> where
    F: Fn(ButtonBarMsg) -> MsgT + Send + Sync + 'static,
{
    pub msg_wrap: F,
    pub palette: &'a Palette,
    pub button_bar_mdl: &'a button_bar::ButtonBarMdl,
    pub buttons: Vec<Button<MsgT>>,
}

pub use self::button_bar::ButtonBarMsg;
use ::material::Palette;
use ::flood::*;
use std::sync::Arc;

impl<'a, MsgT, F> Into<Flood<MsgT>> for ButtonBar<'a, MsgT, F> where
    F: Fn(ButtonBarMsg) -> MsgT + Send + Sync + 'static,
    MsgT: Clone + Send + Sync + 'static,
{
    fn into(self) -> Flood<MsgT> {
        let msg_wrap = Arc::new(self.msg_wrap);
        let palette = self.palette;
        let button_mdls = &self.button_bar_mdl.button_mdls;
        let default_button_mdl = ButtonMdl::default();
        use self::button::ButtonMdl;

        self.buttons.into_iter().fold(Flood::Color(self.palette.transparent), |bar, button| {
            let button_mdl = match button_mdls.get(&button.id) {
                Some(button_mdl) => button_mdl,
                None => &default_button_mdl,
            };

            let string = button.label.to_uppercase();
            let surface_padding_length: Length = Length::Spacing / 2;

            let full_button = {
                let surface = {
                    let text_color = match button.intent {
                        ButtonIntent::Call => palette.secondary,
                        ButtonIntent::Provide => palette.light_background,
                        ButtonIntent::Inform => palette.light_background_text_disabled,
                    };
                    let text = Flood::Text(string.clone(), text_color, Placement::Center);
                    text + Padding::Uniform(surface_padding_length.clone())
                };

                let feedback = match button_mdl.activation {
                    ButtonActivation::Pressed => Flood::Color(palette.light_background_divider),
                    ButtonActivation::Released => Flood::Color(palette.transparent),
                };
                use self::button::ButtonActivation;

                let touch_sensor = {
                    let button_id = button.id;
                    let msg_wrap = msg_wrap.clone();
                    Sensor::Touch(button_id, Arc::new(move |touch_msg| {
                        let button_bar_msg = ButtonBarMsg::Touch(button_id, touch_msg);
                        let owner_msg = msg_wrap(button_bar_msg);
                        owner_msg
                    }))
                };
                use std::sync::Arc;

                let signal_sensor = {
                    let button_id = button.id;
                    let versioned_click_msg: Version<MsgT> = (button.click_msg, button_mdl.click_msg_version_counter).into();
                    Sensor::Signal(Signal::from((button_id, versioned_click_msg)))
                };
                surface + feedback + touch_sensor + signal_sensor
            };
            let full_button_length = Length::Text(string.clone()) + (surface_padding_length.clone() * 2);
            let spacer = Flood::Color(palette.transparent);
            bar + (Position::Right(full_button_length), full_button) + (Position::Right(Length::Spacing / 2), spacer)
        })
    }
}

pub mod button_bar {
    pub struct ButtonBarMdl {
        pub button_mdls: HashMap<u64, ButtonMdl>,
    }

    use std::collections::HashMap;
    use super::button::ButtonMdl;

    pub fn update_button_bar_mdl(mdl: &mut ButtonBarMdl, msg: ButtonBarMsg) {
        match msg {
            ButtonBarMsg::Touch(button_id, touch_msg) => {
                let button_msg = ButtonMsg::Touch(touch_msg);
                let button_mdls = &mut mdl.button_mdls;
                if button_mdls.contains_key(&button_id) {
                    let button_mdl = button_mdls.get_mut(&button_id).unwrap();
                    update_button_mdl(button_mdl, button_msg);
                } else {
                    let mut new_button_mdl = ButtonMdl::default();
                    update_button_mdl(&mut new_button_mdl, button_msg);
                    button_mdls.insert(button_id, new_button_mdl);
                }
            }
        }
    }

    use super::button::{update_button_mdl, ButtonMsg};

    pub enum ButtonBarMsg {
        Touch(u64, TouchMsg)
    }

    use material::TouchMsg;
}

pub mod button {
    pub struct ButtonMdl {
        pub activation: ButtonActivation,
        pub click_msg_version_counter: VersionCounter,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum ButtonActivation {
        Pressed,
        Released,
    }

    use flood::VersionCounter;

    impl Default for ButtonMdl {
        fn default() -> Self {
            ButtonMdl {
                activation: ButtonActivation::Released,
                click_msg_version_counter: VersionCounter::enabled_after_bump(),
            }
        }
    }

    pub fn update_button_mdl(mdl: &mut ButtonMdl, msg: ButtonMsg) {
        match msg {
            ButtonMsg::Touch(TouchMsg::Begin(_, _, _)) => {
                mdl.activation = ButtonActivation::Pressed;
            }
            ButtonMsg::Touch(TouchMsg::End(_, _, _)) => {
                if mdl.activation == ButtonActivation::Pressed {
                    mdl.click_msg_version_counter.bump()
                }
                mdl.activation = ButtonActivation::Released;
            }
            ButtonMsg::Touch(TouchMsg::Cancel(_)) => {
                mdl.activation = ButtonActivation::Released;
            }
            ButtonMsg::Touch(TouchMsg::Move(_, _, _)) => {
                // TODO Released if Pressed and moved beyond threshold
                // TODO Hover
            }
        }
    }

    use material::TouchMsg;

    pub enum ButtonMsg {
        Touch(TouchMsg)
    }
}
