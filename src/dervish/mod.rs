use ::window::BlockRange;
use std::fmt;
use std::sync::Arc;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub enum DervishMsg {}

#[derive(Clone)]
pub enum Dervish {
    Builder(Arc<Fn() -> Sender<DervishMsg> + Send + Sync>)
}

impl fmt::Debug for Dervish {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "DervishBuilder")
    }
}

pub struct DervishSettings {
    pub range: BlockRange,
    pub dervish_builder: Dervish,
}
