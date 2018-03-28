#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

impl Rectangle {
    pub fn new_unit() -> Self {
        Rectangle { left: 0.0, right: 1.0, top: 0.0, bottom: 1.0 }
    }
}
