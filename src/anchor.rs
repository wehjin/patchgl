#[derive(Copy, Clone, Debug)]
pub struct Anchor {
    pub x: f32,
    pub y: f32,
}

impl Anchor {
    pub fn top_left() -> Self { Anchor { x: 0.0, y: 0.0 } }
}

impl Default for Anchor {
    fn default() -> Self {
        Self::top_left()
    }
}
