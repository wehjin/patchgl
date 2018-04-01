use ::TouchMsg;
use Color;
pub use self::length::Length;
use std::ops::Add;
use std::sync::mpsc::Sender;

mod length;

#[derive(Clone, Debug)]
pub enum Flood {
    Color(Color),
    Text(String, Color, Placement),
    Barrier(Position, Box<Flood>, Box<Flood>),
    Vessel(Padding, Box<Flood>),
    Sediment(Silt, Box<Flood>, Box<Flood>),
    Ripple(Touching, Box<Flood>),
}

impl Default for Flood {
    fn default() -> Self {
        Flood::Color(Color::default())
    }
}

impl Add<Touching> for Flood {
    type Output = Flood;

    fn add(self, rhs: Touching) -> <Self as Add<Touching>>::Output {
        Flood::Ripple(rhs, Box::new(self))
    }
}

impl Add<Padding> for Flood {
    type Output = Flood;

    fn add(self, rhs: Padding) -> <Self as Add<Padding>>::Output {
        Flood::Vessel(rhs, Box::new(self))
    }
}

impl Add<(Silt, Flood)> for Flood {
    type Output = Flood;

    fn add(self, (silt, far): (Silt, Flood)) -> <Self as Add<(Silt, Flood)>>::Output {
        Flood::Sediment(silt, Box::new(far), Box::new(self))
    }
}

impl Add<Flood> for Flood {
    type Output = Flood;

    fn add(self, rhs: Flood) -> <Self as Add<Flood>>::Output {
        self + (Silt::Minimum, rhs)
    }
}

impl Add<(Position, Flood)> for Flood {
    type Output = Flood;

    fn add(self, (position, flood): (Position, Flood)) -> <Self as Add<(Position, Flood)>>::Output {
        Flood::Barrier(position, Box::new(self), Box::new(flood))
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Placement {
    Start,
    Center,
    End,
    Custom(f32),
}

impl Into<f32> for Placement {
    fn into(self) -> f32 {
        match self {
            Placement::Start => 0.0,
            Placement::Center => 0.5,
            Placement::End => 1.0,
            Placement::Custom(placement) => placement,
        }
    }
}

impl Default for Placement {
    fn default() -> Self { Placement::Center }
}

#[derive(Clone, Debug)]
pub enum Touching {
    Channel(u64, Sender<TouchMsg>)
}

#[derive(Clone, PartialEq, Debug)]
pub enum Silt {
    Minimum,
}

impl Silt {
    pub fn add_to(&self, rear_approach: f32) -> f32 {
        match self {
            &Silt::Minimum => rear_approach + 1.0,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Padding {
    Uniform(Length),
    Dual(Length, Length),
    Horizontal(Length),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Position {
    Bottom(Length),
    Right(Length),
}

