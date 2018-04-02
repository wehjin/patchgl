use ::{Block, TouchMsg};
use std::sync::Arc;

#[derive(Default)]
pub struct BlockList<MsgT> {
    pub max_approach: f32,
    pub blocks: Vec<Block>,
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)>,
}

impl<MsgT> BlockList<MsgT> {
    pub fn new(max_approach: f32) -> Self {
        BlockList { max_approach, blocks: Vec::new(), touch_adapters: Vec::new() }
    }
    pub fn push_block(&mut self, block: Block) {
        self.max_approach = self.max_approach.max(block.approach);
        self.blocks.push(block);
    }
    pub fn push_touch_adapter(&mut self, touch_adapter: (u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)) {
        self.touch_adapters.push(touch_adapter);
    }
    pub fn append(mut self, rhs: &mut BlockList<MsgT>) -> Self {
        self.max_approach = self.max_approach.max(rhs.max_approach);
        self.blocks.append(&mut rhs.blocks);
        self.touch_adapters.append(&mut rhs.touch_adapters);
        self
    }
}
