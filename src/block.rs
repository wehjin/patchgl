use super::{Anchor, Color, Sigil, WebColor};

#[derive(Debug)]
pub struct Block {
    pub sigil: Sigil,
    pub width: f32,
    pub height: f32,
    pub approach: f32,
    pub anchor: Anchor,
}

impl Default for Block {
    fn default() -> Self {
        let block = Block {
            sigil: Sigil::Color(Color::from(WebColor::Grey)),
            width: 0.0,
            height: 0.0,
            approach: 0.0,
            anchor: Default::default(),
        };
        block
    }
}

impl Block {
    pub fn is_hit(&self, x: f32, y: f32) -> bool {
        self.anchor.x >= x && x < (self.anchor.x + self.width) && self.anchor.y >= y && y < (self.anchor.y + self.height)
    }
}
