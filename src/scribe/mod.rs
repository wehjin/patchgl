use rusttype::{Font, FontCollection};
use rusttype::PositionedGlyph;
pub use rusttype::Scale;

mod glyph_writer;
mod layout;

pub struct Scribe<'a> {
    font: Font<'a>,
}

impl<'a> Scribe<'a> {
    pub fn fit_text(&'a self, text: &str, scale: Scale, width: u32, placement: f32) -> Vec<PositionedGlyph<'a>> {
        layout::fit_text(&self.font, text, scale, width, placement)
    }
}

impl<'a> Default for Scribe<'a> {
    fn default() -> Self {
        let font = FontCollection::from_bytes(include_bytes!("Arial Unicode.ttf") as &[u8]).into_font().unwrap();
        Scribe { font }
    }
}
