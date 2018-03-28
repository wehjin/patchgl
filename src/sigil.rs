use super::{Color, WebColor};

pub enum Sigil {
    FilledRectangle(Color),
    Paragraph { line_height: f32, text: String }
}

impl Default for Sigil {
    fn default() -> Self {
        Sigil::FilledRectangle(Color::from_web(WebColor::DeepPink))
    }
}

