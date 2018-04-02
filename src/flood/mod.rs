use ::dervish::*;
use Color;
pub use self::extras::*;
pub use self::length::Length;
use std::ops::Add;

mod length;
mod extras;

#[derive(Clone, Debug)]
pub enum Flood<MsgT = ()> {
    Color(Color),
    Text(String, Color, Placement),
    Barrier(Position, Box<Flood<MsgT>>, Box<Flood<MsgT>>),
    Vessel(Padding, Box<Flood<MsgT>>),
    Sediment(Silt, Box<Flood<MsgT>>, Box<Flood<MsgT>>),
    Ripple(Sensor<MsgT>, Box<Flood<MsgT>>),
    Dervish(Dervish),
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

