use ::TouchMsg;
use std::fmt;
use std::sync::Arc;
use super::Length;

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
