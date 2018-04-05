use ::Color;
use ::color::argb;
use ::flood::*;
use ::material;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Entry<'a, F, MsgT> where
    F: Fn(Msg) -> MsgT + Send + Sync + 'static,
{
    pub msg_wrap: F,
    pub id: u64,
    pub mdl: &'a Mdl,
    pub label: String,
    pub placeholder: Option<String>,
}

impl<'a, F, MsgT> Entry<'a, F, MsgT> where
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
    pub cursor_visibility: CursorVisibility,
    pub blink_timeout_version_counter: VersionCounter,
}

impl Default for Mdl {
    fn default() -> Self {
        Mdl {
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Msg {
    ToggleBlink,
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
    }
}

pub fn flood<'a, F, MsgT>(entry: &Entry<'a, F, MsgT>) -> Flood<MsgT> where
    MsgT: Clone,
    F: Fn(Msg) -> MsgT, F: Send + Sync + 'static,
{
    let blink_timeout = Timeout {
        id: entry.id,
        msg: (entry.msg_wrap)(Msg::ToggleBlink),
        duration: Duration::Milliseconds(500),
    };
    let versioned_blink = Version::restore(blink_timeout, entry.mdl.blink_timeout_version_counter);
    draw_focused_entry(entry) + Sensor::Timeout(versioned_blink)
}

fn draw_focused_entry<'a, F, MsgT>(entry: &Entry<'a, F, MsgT>) -> Flood<MsgT> where
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

    let cursor_color: Color = match entry.mdl.cursor_visibility {
        CursorVisibility::Visible => accent_dark_color,
        CursorVisibility::Invisible => argb::TRANSPARENT,
    };

    let placeholder_color: Color = material::Color::LightBackgroundTextDisabled.into();
    let placeholder = Flood::Text(entry.placeholder_string(), placeholder_color, Placement::Start);

    let cursor = Flood::Color(cursor_color);
    let input = Flood::Color(argb::TRANSPARENT) + (Position::Left(Length::Pixels(1.0)), cursor);

    let input_and_placeholder = input + (Stratum::JustBelow, placeholder);

    let bottom_line = Flood::Color(accent_dark_color);
    input_and_placeholder
        + (Position::Bottom(Length::Full * INPUT_BOTTOM_PADDING / FULL_HEIGHT), Flood::Color(argb::TRANSPARENT))
        + (Position::Bottom(Length::Full * BOTTOM_LINE_HEIGHT / FULL_HEIGHT), bottom_line)
        + (Position::Top(Length::Full * LABEL_BOTTOM_PADDING / FULL_HEIGHT), Flood::Color(argb::TRANSPARENT))
        + (Position::Top(Length::Full * LABEL_HEIGHT / FULL_HEIGHT), Flood::Text(entry.label.clone(), accent_dark_color, Placement::Start))
        + (Position::Top(Length::Full * LABEL_TOP_PADDING / FULL_HEIGHT), Flood::Color(argb::TRANSPARENT))
}


