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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{Glyff, SignedDistanceField, Rectangle};

    #[test]
    fn glyffcenter_puts_a_glyff() {
        let mut glyff_center = GlyffCenter::new();
        glyff_center.put_glyff(0, Glyff::new(SignedDistanceField::new_near(), Rectangle::new_unit()));
        assert_eq!(1.0, glyff_center.get_glyff(0).anchor.right);
    }
}
