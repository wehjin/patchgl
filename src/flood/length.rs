use std::ops::{Add, Sub, Mul, Div};
use scribe::Scribe;

#[derive(Clone, PartialEq, Debug)]
pub enum Length {
    Zero,
    FingerTip,
    Pixels(f32),
    Spacing,
    Sum(Box<Length>, Box<Length>),
    Scale(f32, Box<Length>),
    Half,
    Third,
    Full,
    Min(Box<Length>, Box<Length>),
    Negative(Box<Length>),
    Cross,
    Inverse(Box<Length>),
    Product(Box<Length>, Box<Length>),
    Text(String),
    TextUnit(String),
    CardApproach,
}

impl Length {
    pub fn to_f32<'a>(&self, context: f32, alt_context: f32, scribe: &Scribe<'a>) -> f32 {
        match self {
            &Length::Zero => 0.0,
            &Length::Full => context,
            &Length::Half => context / 2.0,
            &Length::Third => context / 3.0,
            &Length::FingerTip => 44.0,
            &Length::Pixels(pixels) => pixels,
            &Length::Spacing => 16.0,
            &Length::Sum(ref a, ref b) => a.to_f32(context, alt_context, scribe) + b.to_f32(context, alt_context, scribe),
            &Length::Scale(factor, ref a) => a.to_f32(context, alt_context, scribe) * factor,
            &Length::Min(ref a, ref b) => a.to_f32(context, alt_context, scribe).min(b.to_f32(context, alt_context, scribe)),
            &Length::Negative(ref a) => -a.to_f32(context, alt_context, scribe),
            &Length::Inverse(ref a) => 1.0 / a.to_f32(context, alt_context, scribe),
            &Length::Cross => alt_context,
            &Length::Product(ref a, ref b) => a.to_f32(context, alt_context, scribe) * b.to_f32(context, alt_context, scribe),
            &Length::Text(ref text) => alt_context * scribe.size_text(text),
            &Length::TextUnit(ref text) => scribe.size_text(text),
            &Length::CardApproach => 2.0,
        }
    }

    pub fn min(self, rhs: Length) -> Self {
        Length::Min(Box::new(self), Box::new(rhs))
    }
}

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> <Self as Add<Length>>::Output {
        Length::Sum(Box::new(self), Box::new(rhs))
    }
}

impl Add<i32> for Length {
    type Output = Length;

    fn add(self, rhs: i32) -> <Self as Add<i32>>::Output {
        Length::Sum(Box::new(self), Box::new(Length::Pixels(rhs as f32)))
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Length) -> <Self as Sub<Length>>::Output {
        Length::Sum(Box::new(self), Box::new(Length::Negative(Box::new(rhs))))
    }
}

impl Mul for Length {
    type Output = Length;

    fn mul(self, rhs: Length) -> <Self as Mul<Length>>::Output {
        Length::Product(Box::new(self), Box::new(rhs))
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

impl Div for Length {
    type Output = Length;

    fn div(self, rhs: Length) -> <Self as Div<Length>>::Output {
        Length::Product(Box::new(self), Box::new(Length::Inverse(Box::new(rhs))))
    }
}

impl Div<u32> for Length {
    type Output = Length;

    fn div(self, rhs: u32) -> <Self as Div<u32>>::Output {
        self / (rhs as f32)
    }
}

impl Div<i32> for Length {
    type Output = Length;

    fn div(self, rhs: i32) -> <Self as Div<i32>>::Output {
        self / (rhs as f32)
    }
}

impl Div<usize> for Length {
    type Output = Length;

    fn div(self, rhs: usize) -> <Self as Div<usize>>::Output {
        self / (rhs as f32)
    }
}

impl Div<f32> for Length {
    type Output = Length;

    fn div(self, rhs: f32) -> <Self as Div<f32>>::Output {
        self * rhs.recip()
    }
}
