use ::TouchMsg;
use Color;
pub use self::length::Length;
use std::fmt;
use std::ops::Add;
use std::sync::Arc;

mod length;

#[derive(Clone, Debug)]
pub enum Flood<MsgT = ()> {
    Color(Color),
    Text(String, Color, Placement),
    Barrier(Position, Box<Flood<MsgT>>, Box<Flood<MsgT>>),
    Vessel(Padding, Box<Flood<MsgT>>),
    Sediment(Silt, Box<Flood<MsgT>>, Box<Flood<MsgT>>),
    Ripple(Sensor<MsgT>, Box<Flood<MsgT>>),
}

impl<MsgT> Default for Flood<MsgT> {
    fn default() -> Self {
        Flood::Color(Color::default())
    }
}

impl<MsgT> Add<Sensor<MsgT>> for Flood<MsgT> {
    type Output = Flood<MsgT>;

    fn add(self, rhs: Sensor<MsgT>) -> <Self as Add<Sensor<MsgT>>>::Output {
        Flood::Ripple(rhs, Box::new(self))
    }
}

impl<MsgT> Add<Padding> for Flood<MsgT> {
    type Output = Flood<MsgT>;

    fn add(self, rhs: Padding) -> <Self as Add<Padding>>::Output {
        Flood::Vessel(rhs, Box::new(self))
    }
}

impl<MsgT> Add<(Silt, Flood<MsgT>)> for Flood<MsgT> {
    type Output = Flood<MsgT>;

    fn add(self, (silt, far): (Silt, Flood<MsgT>)) -> <Self as Add<(Silt, Flood<MsgT>)>>::Output {
        Flood::Sediment(silt, Box::new(far), Box::new(self))
    }
}

impl<MsgT> Add<Flood<MsgT>> for Flood<MsgT> {
    type Output = Flood<MsgT>;

    fn add(self, rhs: Flood<MsgT>) -> <Self as Add<Flood<MsgT>>>::Output {
        self + (Silt::Minimum, rhs)
    }
}

impl<MsgT> Add<(Position, Flood<MsgT>)> for Flood<MsgT> {
    type Output = Flood<MsgT>;

    fn add(self, (position, flood): (Position, Flood<MsgT>)) -> <Self as Add<(Position, Flood<MsgT>)>>::Output {
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

#[derive(Clone)]
pub enum Sensor<MsgT> where {
    Touch(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)
}

impl<MsgT> fmt::Debug for Sensor<MsgT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Sensor::Touch(tag, _) => write!(f, "Sensor::Touch({})", tag)
        }
    }
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

