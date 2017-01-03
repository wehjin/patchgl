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
    fn clone(&self) -> SignedDistanceField {
        SignedDistanceField {
            distances: self.distances
        }
    }
}

impl SignedDistanceField {
    pub fn get_index(column: usize, row: usize) -> usize {
        row * SDF_SIZE + column
    }
    pub fn new() -> Self {
        SignedDistanceField { distances: [0u8; SDF_SIZE * SDF_SIZE] }
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
    left: f32,
    right: f32,
    top: f32,
    bottom: f32
}

#[derive(Debug, Copy, Clone)]
pub struct Glyff {
    pub sdf: SignedDistanceField,
    pub anchor: Rectangle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_computes_sdf_column_index() {
        assert_eq!(1, SignedDistanceField::get_index(1, 0));
    }

    #[test]
    fn it_computes_sdf_row_index() {
        assert_eq!(SDF_SIZE, SignedDistanceField::get_index(0, 1))
    }

    #[test]
    fn it_gets_and_puts_distance_in_sdf() {
        let mut sdf = SignedDistanceField::new();
        sdf.put_distance(0, 1, 34);
        assert_eq!(34, sdf.get_distance(0, 1))
    }
}
