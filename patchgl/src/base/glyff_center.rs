use super::Glyff;
use super::SignedDistanceField;
use super::Rectangle;

#[derive(Copy, Clone)]
pub struct GlyffCenter {
    glyffs: [Glyff; 32]
}

impl GlyffCenter {
    pub fn new() -> Self {
        let glyff = Glyff::new(SignedDistanceField::new_far(), Rectangle::new_unit());
        GlyffCenter {
            glyffs: [glyff; 32]
        }
    }
    pub fn put_glyff(&mut self, key: usize, glyff: Glyff) {
        self.glyffs[key] = glyff;
    }
    pub fn get_glyff(&self, key: usize) -> &Glyff {
        &self.glyffs[key]
    }
}
