use ::{Block, TouchMsg};
use ::flood::{Signal, Timeout, Version, Input};
use std::sync::Arc;

pub struct Blocklist<MsgT> where
    MsgT: Clone
{
    pub max_approach: f32,
    pub blocks: Vec<Block>,
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)>,
    pub input_adapters: Vec<Arc<Fn(Input) -> MsgT + Send + Sync>>,
    pub raft_msgs: Vec<MsgT>,
    pub signals: Vec<Signal<MsgT>>,
    pub timeouts: Vec<Version<Timeout<MsgT>>>,
}

impl<MsgT> Default for Blocklist<MsgT> where
    MsgT: Clone
{
    fn default() -> Self {
        Blocklist {
            max_approach: 0.0,
            blocks: Vec::new(),
            touch_adapters: Vec::new(),
            input_adapters: Vec::new(),
            raft_msgs: Vec::new(),
            signals: Vec::new(),
            timeouts: Vec::new(),
        }
    }
}

impl<MsgT> Blocklist<MsgT> where
    MsgT: Clone
{
    pub fn push_block(&mut self, block: Block) {
        self.update_max_approach(block.approach);
        self.blocks.push(block);
    }
    pub fn update_max_approach(&mut self, max_approach: f32) {
        self.max_approach = self.max_approach.max(max_approach)
    }

    pub fn append(mut self, rhs: &mut Blocklist<MsgT>) -> Self {
        self.max_approach = self.max_approach.max(rhs.max_approach);
        self.blocks.append(&mut rhs.blocks);
        self.touch_adapters.append(&mut rhs.touch_adapters);
        self.input_adapters.append(&mut rhs.input_adapters);
        self.raft_msgs.append(&mut rhs.raft_msgs);
        self.signals.append(&mut rhs.signals);
        self.timeouts.append(&mut rhs.timeouts);
        self
    }
}
