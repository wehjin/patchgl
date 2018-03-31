use ::TouchMsg;
use std::sync::mpsc::Sender;
use super::{Color, WebColor};

#[derive(Clone, Debug)]
pub enum Sigil {
    Color(Color),
    Paragraph { line_height: f32, text: String, color: Color },
    Ghost(Sender<TouchMsg>),
}

impl Default for Sigil {
    fn default() -> Self {
        Sigil::Color(Color::from(WebColor::DeepPink))
    }
}
