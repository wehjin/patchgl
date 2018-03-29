use super::{Color, WebColor};

pub enum Sigil {
    Color(Color),
    Paragraph { line_height: f32, text: String }
}

impl Default for Sigil {
    fn default() -> Self {
        Sigil::Color(Color::from_web(WebColor::DeepPink))
    }
}

