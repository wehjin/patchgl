use ::Color;
use ::color::argb;
use ::flood::*;
use ::material;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Entry<F, MsgT> where
    F: Fn(Msg) -> MsgT + Send + Sync + 'static,
{
    pub msg_wrap: F,
    pub id: u64,
    pub mdl: Mdl,
    pub label: String,
    pub placeholder: Option<String>,
}

impl<F, MsgT> Entry<F, MsgT> where
    F: Fn(Msg) -> MsgT, F: Send + Sync + 'static,
{
    fn placeholder_string(&self) -> String {
        if let Some(ref placeholder) = self.placeholder {
            placeholder.to_owned()
        } else {
            "".into()
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Mdl {
    pub pretext: Option<String>,
    pub cursor_visibility: CursorVisibility,
    pub blink_timeout_version_counter: VersionCounter,
}

impl Default for Mdl {
    fn default() -> Self {
        Mdl {
            pretext: None,
            cursor_visibility: CursorVisibility::Visible,
            blink_timeout_version_counter: VersionCounter::enabled(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CursorVisibility {
    Visible,
    Invisible,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Msg {
    ToggleBlink,
    Input(Input),
}

pub fn update(mdl: &mut Mdl, msg: Msg) {
    match msg {
        Msg::ToggleBlink => {
            mdl.cursor_visibility = match mdl.cursor_visibility {
                CursorVisibility::Visible => CursorVisibility::Invisible,
                CursorVisibility::Invisible => CursorVisibility::Visible,
            };
            mdl.blink_timeout_version_counter.bump();
        }
        Msg::Input(input) => {
            match input {
                Input::Insert(string) => {
                    mdl.pretext = match &mdl.pretext {
                        &Some(ref pretext) => {
                            let combined = pretext.to_owned() + &string;
                            trim_pretext(&combined)
                        }
                        &None => trim_pretext(&string),
                    }
                }
                Input::DeleteBack => {
                    mdl.pretext = match &mdl.pretext {
                        &Some(ref pretext) => {
                            if pretext.len() > 1 {
                                trim_pretext(&pretext[..pretext.len() - 1])
                            } else {
                                None
                            }
                        }
                        &None => None
                    }
                }
            }
        }
    }
}

fn trim_pretext(pretext: &str) -> Option<String> {
    let pretext = pretext.trim_left().to_owned();
    if pretext.is_empty() {
        None
    } else {
        Some(pretext.to_owned())
    }
}

pub fn flood<F, MsgT>(entry: Entry<F, MsgT>) -> Flood<MsgT> where
    MsgT: Clone + 'static,
    F: Fn(Msg) -> MsgT, F: Send + Sync + 'static,
{
    use std::sync::Arc;

    let surface = draw_focused_entry(&entry);
    let blink_timeout = Timeout {
        id: entry.id,
        msg: (entry.msg_wrap)(Msg::ToggleBlink),
        duration: Duration::Milliseconds(500),
    };
    let versioned_blink = Version::restore(blink_timeout, entry.mdl.blink_timeout_version_counter);
    let input_wrap = {
        Arc::new(move |input| (entry.msg_wrap)(Msg::Input(input)))
    };
    surface
        + Sensor::Timeout(versioned_blink)
        + Sensor::Input(input_wrap)
}

fn draw_focused_entry<F, MsgT>(entry: &Entry<F, MsgT>) -> Flood<MsgT> where
    MsgT: Clone,
    F: Fn(Msg) -> MsgT, F: Send + Sync + 'static,
{
    const LABEL_TOP_PADDING: f32 = 16.0;
    const LABEL_HEIGHT: f32 = 12.0;
    const LABEL_BOTTOM_PADDING: f32 = 8.0;
    const INPUT_HEIGHT: f32 = 16.0;
    const BOTTOM_LINE_HEIGHT: f32 = 2.0;
    const INPUT_BOTTOM_PADDING: f32 = 8.0 - BOTTOM_LINE_HEIGHT;
    const FULL_HEIGHT: f32 = LABEL_TOP_PADDING + LABEL_HEIGHT + LABEL_BOTTOM_PADDING
        + INPUT_HEIGHT + INPUT_BOTTOM_PADDING
        + BOTTOM_LINE_HEIGHT
    ;
    let accent_dark_color: Color = material::Color::PinkA700.into();

    let placeholder = {
        if entry.mdl.pretext.is_none() {
            let placeholder_color: Color = material::Color::LightBackgroundTextDisabled.into();
            Flood::Text(entry.placeholder_string(), placeholder_color, Placement::Start)
        } else {
            Flood::Color(argb::TRANSPARENT)
        }
    };
    let input = {
        let cursor = {
            let cursor_color: Color = match entry.mdl.cursor_visibility {
                CursorVisibility::Visible => accent_dark_color,
                CursorVisibility::Invisible => argb::TRANSPARENT,
            };
            Flood::Color(cursor_color)
        };
        let runway = Flood::Color(argb::TRANSPARENT);
        let cursor_width = Length::Pixels(1.0);
        let cursor_and_runway = runway + (Position::Left(cursor_width.clone()), cursor);
        match entry.mdl.pretext {
            Some(ref pretext_string) => {
                let color: Color = material::color::Color::LightBackgroundTextPrimary.into();
                let flood = Flood::Text(pretext_string.to_owned(), color, Placement::Start);
                let pretext_width = Length::Text(pretext_string.to_owned());
                let pretext_width_with_fuzz = pretext_width + 4;
                let length = pretext_width_with_fuzz.min(Length::Full - cursor_width.clone());
                cursor_and_runway + (Position::Left(length), flood)
            }
            None => cursor_and_runway
        }
    };
    let input_and_placeholder = input + (Stratum::JustBelow, placeholder);

    let bottom_line = Flood::Color(accent_dark_color);
    input_and_placeholder
        + (Position::Bottom(Length::Full * INPUT_BOTTOM_PADDING / FULL_HEIGHT), Flood::Color(argb::TRANSPARENT))
        + (Position::Bottom(Length::Full * BOTTOM_LINE_HEIGHT / FULL_HEIGHT), bottom_line)
        + (Position::Top(Length::Full * LABEL_BOTTOM_PADDING / FULL_HEIGHT), Flood::Color(argb::TRANSPARENT))
        + (Position::Top(Length::Full * LABEL_HEIGHT / FULL_HEIGHT), Flood::Text(entry.label.clone(), accent_dark_color, Placement::Start))
        + (Position::Top(Length::Full * LABEL_TOP_PADDING / FULL_HEIGHT), Flood::Color(argb::TRANSPARENT))
}


