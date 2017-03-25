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
        Self::new(255u8)
    }
    pub fn new_near() -> Self {
        Self::new(0u8)
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
