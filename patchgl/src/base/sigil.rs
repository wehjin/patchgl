use super::Shape;

pub struct Sigil {
    pub shape: Shape,
    pub left_x: f32,
    pub right_x: f32,
    pub top_y: f32,
    pub bottom_y: f32,
    pub near_z: f32,
}

impl Sigil {
    pub fn new_from_width_height(width: f32, height: f32, shape: Shape) -> Self {
        Sigil {
            shape: shape,
            left_x: 0f32,
            right_x: width,
            bottom_y: 0f32,
            top_y: height,
            near_z: 0f32,
        }
    }

    pub fn width(&self) -> f32 {
        self.right_x - self.left_x
    }
}