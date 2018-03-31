use std::ops::{Add, Mul};

#[derive(Clone, PartialEq, Debug)]
pub enum Length {
    FingerTip,
    Pixels(f32),
    Padding,
    Sum(Box<Length>, Box<Length>),
    Scale(f32, Box<Length>),
}

impl Length {
    pub fn to_f32(&self) -> f32 {
        match self {
            &Length::FingerTip => 44.0,
            &Length::Pixels(pixels) => pixels,
            &Length::Padding => 16.0,
            &Length::Sum(ref a, ref b) => a.to_f32() + b.to_f32(),
            &Length::Scale(factor, ref a) => a.to_f32() * factor,
        }
    }
}

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> <Self as Add<Length>>::Output {
        Length::Sum(Box::new(self), Box::new(rhs))
    }
}

impl Mul<f32> for Length {
    type Output = Length;

    fn mul(self, rhs: f32) -> <Self as Mul<f32>>::Output {
        Length::Scale(rhs, Box::new(self))
    }
}

impl Mul<usize> for Length {
    type Output = Length;

    fn mul(self, rhs: usize) -> <Self as Mul<usize>>::Output {
        self * rhs as f32
    }
}
