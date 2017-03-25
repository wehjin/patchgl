mod sdf;
mod rectangle;
mod glyff;
mod glyff_center;

pub use self::sdf::*;
pub use self::rectangle::*;
pub use self::glyff::*;
pub use self::glyff_center::*;

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
    fn sdf_gets_and_puts_distance() {
        let mut sdf = SignedDistanceField::new(0u8);
        sdf.put_distance(0, 1, 34);
        assert_eq!(34, sdf.get_distance(0, 1))
    }

    #[test]
    fn sdf_computes_column_index() {
        assert_eq!(1, SignedDistanceField::get_index(1, 0));
    }

    #[test]
    fn sdf_computes_row_index() {
        let index = SignedDistanceField::get_index(0, 1);
        assert_eq!(SDF_SIZE, index)
    }
}
