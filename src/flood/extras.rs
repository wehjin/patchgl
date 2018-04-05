use ::TouchMsg;
use ::window::BlockRange;
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
pub enum Sensor<MsgT> where
    MsgT: Clone
{
    Touch(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>),
    Signal(Signal<MsgT>),
}

impl<MsgT> fmt::Debug for Sensor<MsgT> where
    MsgT: Clone
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Sensor::Touch(tag, _) => write!(f, "Sensor::Touch({})", tag),
            &Sensor::Signal(ref signal) => write!(f, "Sensor::{:?}", signal),
        }
    }
}

#[derive(Clone)]
pub enum Signal<MsgT> where
    MsgT: Clone
{
    Set(u64, u64),
    GoIfGreater(u64, u64, MsgT),
}

impl<MsgT> Signal<MsgT> where
    MsgT: Clone
{
    pub fn id(&self) -> u64 {
        match self {
            &Signal::Set(id, _count) => id,
            &Signal::GoIfGreater(id, _, _) => id,
        }
    }
    pub fn count(&self) -> u64 {
        match self {
            &Signal::Set(_id, count) => count,
            &Signal::GoIfGreater(_, count, _) => count,
        }
    }
}

impl<MsgT> fmt::Debug for Signal<MsgT> where
    MsgT: Clone
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Signal::Set(id, count) => write!(f, "Signal::Set(id={}, count={})", id, count),
            &Signal::GoIfGreater(id, count, _) => write!(f, "Signal::SetAndGo(id={}, count={}, msg=MsgT)", id, count),
        }
    }
}

#[derive(Clone)]
pub enum Raft<MsgT> {
    RangeAdapter(u64, Arc<Fn(u64, &BlockRange) -> MsgT + Send + Sync>)
}

impl<MsgT> fmt::Debug for Raft<MsgT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let &Raft::RangeAdapter(tag, _) = self;
        write!(f, "Raft::RangeAdapter({}, Arc<Fn(u64, &BlockRange) -> MsgT + Send + Sync>", tag)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Stratum {
    JustBelow,
}

impl Stratum {
    pub fn add_to(&self, rear_approach: f32) -> f32 {
        match self {
            &Stratum::JustBelow => rear_approach + 0.001,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Padding {
    Uniform(Length),
    Dual(Length, Length),
    Horizontal(Length),
    Vertical(Length),
    Behind(Length),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Position {
    Left(Length),
    Top(Length),
    Right(Length),
    Bottom(Length),
}
