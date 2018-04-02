use super::{Color, WebColor};

#[derive(Clone, Debug)]
pub enum Sigil {
    Color(Color),
    Paragraph { line_height: f32, text: String, color: Color, placement: f32 },
    Touch(u64),
}

impl Default for Sigil {
    fn default() -> Self {
        Sigil::Color(Color::from(WebColor::DeepPink))
    }
}
