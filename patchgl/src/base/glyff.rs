use super::SignedDistanceField;
use super::Rectangle;

#[derive(Debug, Copy, Clone)]
pub struct Glyff {
    pub sdf: SignedDistanceField,
    pub anchor: Rectangle
}

impl Glyff {
    pub fn new(sdf: SignedDistanceField, anchor: Rectangle) -> Self {
        Glyff { sdf: sdf, anchor: anchor }
    }
}
