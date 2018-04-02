use ::window::{Blocklist, BlockRange};
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub enum DervishMsg {}

#[derive(Clone, Debug)]
pub enum Dervish {
    Sender(Sender<DervishMsg>)
}

pub struct WhirlingDervish<MsgT> {
    pub blocklist: Blocklist<MsgT>,
    pub range: BlockRange,
    pub sender: Sender<DervishMsg>,
}
