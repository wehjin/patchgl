use std::ops::{Add, Div, Mul};

#[derive(Clone, PartialEq, Debug)]
pub enum Length {
    Zero,
    FingerTip,
    Pixels(f32),
    Spacing,
    Sum(Box<Length>, Box<Length>),
    Scale(f32, Box<Length>),
    Half,
}

impl Length {
    pub fn to_f32(&self, context: f32) -> f32 {
        match self {
            &Length::Half => context * 0.5,
            &Length::Zero => 0.0,
            &Length::FingerTip => 44.0,
            &Length::Pixels(pixels) => pixels,
            &Length::Spacing => 16.0,
            &Length::Sum(ref a, ref b) => a.to_f32(context) + b.to_f32(context),
            &Length::Scale(factor, ref a) => a.to_f32(context) * factor,
        }
    }
}

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> <Self as Add<Length>>::Output {
        Length::Sum(Box::new(self), Box::new(rhs))
    }
}

impl Mul<usize> for Length {
    type Output = Length;

    fn mul(self, rhs: usize) -> <Self as Mul<usize>>::Output {
        self * rhs as f32
    }
}

impl Mul<f32> for Length {
    type Output = Length;

    fn mul(self, rhs: f32) -> <Self as Mul<f32>>::Output {
        Length::Scale(rhs, Box::new(self))
    }
}

impl Div<u32> for Length {
    type Output = Length;

    fn div(self, rhs: u32) -> <Self as Div<u32>>::Output {
        self / (rhs as f32)
    }
}

impl Div<f32> for Length {
    type Output = Length;

    fn div(self, rhs: f32) -> <Self as Div<f32>>::Output {
        self * rhs.recip()
    }
}
