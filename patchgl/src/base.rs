use std::fmt;

pub const SDF_SIZE: usize = 64;

#[derive(Copy)]
pub struct SignedDistanceField {
    distances: [u8; SDF_SIZE * SDF_SIZE]
}

impl fmt::Debug for SignedDistanceField {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.distances[..].fmt(formatter)
    }
}

impl Clone for SignedDistanceField {
    fn clone(&self) -> Self {
        SignedDistanceField { distances: self.distances }
    }
}

impl SignedDistanceField {
    pub fn new(fill: u8) -> Self {
        SignedDistanceField { distances: [fill; SDF_SIZE * SDF_SIZE] }
    }
    pub fn new_far() -> Self {
        SignedDistanceField::new(255u8)
    }
    pub fn new_near() -> Self {
        SignedDistanceField::new(0u8)
    }
    pub fn get_index(column: usize, row: usize) -> usize {
        row * SDF_SIZE + column
    }
    pub fn get_distance(&self, column: usize, row: usize) -> u8 {
        let index = Self::get_index(column, row);
        self.distances[index]
    }
    pub fn put_distance(&mut self, column: usize, row: usize, distance: u8) {
        let index = Self::get_index(column, row);
        self.distances[index] = distance
    }
}

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

#[derive(Debug, Copy, Clone)]
pub struct Glyff {
    sdf: SignedDistanceField,
    anchor: Rectangle
}

impl Glyff {
    pub fn new(sdf: SignedDistanceField, anchor: Rectangle) -> Self {
        Glyff { sdf: sdf, anchor: anchor }
    }
}

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

    #[test]
    fn glyffcenter_puts_a_glyff() {
        let mut glyff_center = GlyffCenter::new();
        glyff_center.put_glyff(0, Glyff::new(SignedDistanceField::new_near(), Rectangle::new_unit()));
        assert_eq!(1.0, glyff_center.get_glyff(0).anchor.right);
    }

    #[test]
    fn sdf_computes_column_index() {
        assert_eq!(1, SignedDistanceField::get_index(1, 0));
    }

    #[test]
    fn sdf_computes_row_index() {
        assert_eq!(SDF_SIZE, SignedDistanceField::get_index(0, 1))
    }

    #[test]
    fn sdf_gets_and_puts_distance() {
        let mut sdf = SignedDistanceField::new(0);
        sdf.put_distance(0, 1, 34);
        assert_eq!(34, sdf.get_distance(0, 1))
    }
}
