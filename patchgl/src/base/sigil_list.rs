use super::Sigil;

pub trait SigilList {
    fn as_vec(&self) -> &Vec<Sigil>;
}

pub struct BasicSigilList {
    sigils: Vec<Sigil>
}

impl SigilList for BasicSigilList {
    fn as_vec(&self) -> &Vec<Sigil> {
        &self.sigils
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{Sigil, Shape};

    #[test]
    fn basic_sigil_list_produces_vec() {
        let basic_sigil_list = BasicSigilList {
            sigils: vec!(Sigil::new_from_width_height(30f32, 20f32, Shape::Rectangle))
        };
        let sigils = basic_sigil_list.sigils;
        assert_eq!(1, sigils.len());
        assert_eq!(30f32, sigils[0].width())
    }
}
