use ::TouchMsg;
use ::window::BlockRange;
use std::fmt;
use std::sync::Arc;
use super::Length;
use super::Signal;
pub use self::timeout::*;
use ::flood::Version;

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

mod timeout {
    use std::fmt;

    pub struct Timeout<MsgT> {
        pub id: u64,
        pub msg: MsgT,
        pub duration: Duration,
    }

    impl<MsgT> Clone for Timeout<MsgT> where MsgT: Clone {
        fn clone(&self) -> Self {
            Timeout {
                id: self.id,
                msg: self.msg.clone(),
                duration: self.duration,
            }
        }
    }

    impl<MsgT> fmt::Debug for Timeout<MsgT> where MsgT: fmt::Debug {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "Timeout {{ id={:?}, msg={:?}, duration={:?} }}", self.id, self.msg, self.duration)
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Duration {
        Seconds(u64),
        Milliseconds(u64),
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Input {
    Insert(String),
    DeleteBack,
}

#[derive(Clone)]
pub enum Sensor<MsgT> where
    MsgT: Clone
{
    Touch(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>),
    Input(Arc<Fn(Input) -> MsgT + Send + Sync>),
    Signal(Signal<MsgT>),
    Timeout(Version<Timeout<MsgT>>),
}

impl<MsgT> fmt::Debug for Sensor<MsgT> where MsgT: Clone + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Sensor::Touch(tag, _) => write!(f, "Sensor::Touch({})", tag),
            &Sensor::Input(_) => write!(f, "Sensor::String()"),
            &Sensor::Signal(ref signal) => write!(f, "Sensor::Signal({:?})", signal),
            &Sensor::Timeout(ref versioned_timeout) => write!(f, "Sensor::Timeout({:?})", versioned_timeout),
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
