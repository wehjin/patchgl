use ::TouchMsg;
use Color;
pub use self::length::Length;
use std::ops::{Add, BitAnd, Sub};
use std::sync::mpsc::Sender;

mod length;

#[derive(Clone, Debug)]
pub enum Flood {
    Color(Color),
    Text(String, Color),
    Barrier(Position, Box<Flood>, Box<Flood>),
    Vessel(Thickness, Box<Flood>),
    Sediment(Silt, Box<Flood>, Box<Flood>),
    Sensor(u64, Box<Flood>, Sender<TouchMsg>),
}

impl Flood {
    pub fn track(self, tag: u64, tracker: Sender<TouchMsg>) -> Self {
        Flood::Sensor(tag, Box::new(self), tracker)
    }
}

impl Default for Flood {
    fn default() -> Self {
        Flood::Color(Color::default())
    }
}

impl Sub<Thickness> for Flood {
    type Output = Flood;

    fn sub(self, rhs: Thickness) -> <Self as Sub<Thickness>>::Output {
        Flood::Vessel(rhs, Box::new(self))
    }
}

impl BitAnd<(Silt, Flood)> for Flood {
    type Output = Flood;

    fn bitand(self, (silt, far): (Silt, Flood)) -> <Self as BitAnd<(Silt, Flood)>>::Output {
        Flood::Sediment(silt, Box::new(far), Box::new(self))
    }
}

impl BitAnd<Flood> for Flood {
    type Output = Flood;

    fn bitand(self, rhs: Flood) -> <Self as BitAnd<Flood>>::Output {
        self & (Silt::Minimum, rhs)
    }
}

impl Add<(Position, Flood)> for Flood {
    type Output = Flood;

    fn add(self, (position, flood): (Position, Flood)) -> <Self as Add<(Position, Flood)>>::Output {
        Flood::Barrier(position, Box::new(self), Box::new(flood))
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
pub enum Thickness {
    Uniform(Length),
    Dual(Length, Length),
    Horizontal(Length),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Position {
    Bottom(Length),
}

