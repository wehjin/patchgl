use std::fmt;

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
