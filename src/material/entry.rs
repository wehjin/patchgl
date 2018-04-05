use ::Color;
use ::color::argb;
use ::flood::*;
use ::material;

#[derive(Clone)]
pub struct Entry {
    pub id: u64,
    pub label: String,
    pub placeholder: Option<String>,
}

impl Entry {
    fn placeholder_string(&self) -> String {
        if let Some(ref placeholder) = self.placeholder {
            placeholder.to_owned()
        } else {
            "".into()
        }
    }
}

pub fn draw_focused_entry<MsgT>(entry: &Entry) -> Flood<MsgT> where
    MsgT: Clone
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
    let placeholder_color: Color = material::Color::LightBackgroundTextDisabled.into();
    let transparent_flood = Flood::Color(argb::TRANSPARENT);

    let text = Flood::Text(entry.placeholder_string(), placeholder_color, Placement::Start);
    let bottom_line = Flood::Color(accent_dark_color);
    text
        + (Position::Bottom(Length::Full * INPUT_BOTTOM_PADDING / FULL_HEIGHT), transparent_flood.clone())
        + (Position::Bottom(Length::Full * BOTTOM_LINE_HEIGHT / FULL_HEIGHT), bottom_line)
        + (Position::Top(Length::Full * LABEL_BOTTOM_PADDING / FULL_HEIGHT), transparent_flood.clone())
        + (Position::Top(Length::Full * LABEL_HEIGHT / FULL_HEIGHT), Flood::Text(entry.label.clone(), accent_dark_color, Placement::Start))
        + (Position::Top(Length::Full * LABEL_TOP_PADDING / FULL_HEIGHT), transparent_flood.clone())
}


