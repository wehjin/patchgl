use super::{Color, WebColor};

#[derive(Clone, PartialEq, Debug)]
pub enum Sigil {
    Color(Color),
    Paragraph { line_height: f32, text: String, color: Color },
    Ghost(u64),
}

impl Default for Sigil {
    fn default() -> Self {
        Sigil::Color(Color::from(WebColor::DeepPink))
    }
}
