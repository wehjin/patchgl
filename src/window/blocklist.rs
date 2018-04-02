use ::{Block, TouchMsg};
use ::dervish::*;
use std::sync::Arc;

#[derive(Default)]
pub struct Blocklist<MsgT> {
    pub max_approach: f32,
    pub blocks: Vec<Block>,
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)>,
    pub whirlings: Vec<WhirlingDervish<MsgT>>,
}

impl<MsgT> Blocklist<MsgT> {
    pub fn push_block(&mut self, block: Block) {
        self.update_max_approach(block.approach);
        self.blocks.push(block);
    }
    pub fn push_touch_adapter(&mut self, touch_adapter: (u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)) {
        self.touch_adapters.push(touch_adapter);
    }
    pub fn push_whirling(&mut self, whirling: WhirlingDervish<MsgT>) {
        self.update_max_approach(whirling.blocklist.max_approach);
        self.whirlings.push(whirling);
    }
    pub fn update_max_approach(&mut self, max_approach: f32) {
        self.max_approach = self.max_approach.max(max_approach)
    }

    pub fn append(mut self, rhs: &mut Blocklist<MsgT>) -> Self {
        self.max_approach = self.max_approach.max(rhs.max_approach);
        self.blocks.append(&mut rhs.blocks);
        self.touch_adapters.append(&mut rhs.touch_adapters);
        self.whirlings.append(&mut rhs.whirlings);
        self
    }
}
