use ::{Color, ScreenMsg, TouchMsg};
use ::flood::Flood;
use ::window::BlockRange;
use ::window::WindowNote;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use super::build_blocklist;

pub enum FloodplainMsg<MsgT> {
    Flood(Flood<MsgT>),
    Observe(Sender<MsgT>),
    WindowNote(WindowNote),
}

pub struct Floodplain<MsgT> {
    pub seed: Option<u64>,
    pub range: BlockRange,
    pub screen: Option<Sender<ScreenMsg>>,
    pub flood: Flood<MsgT>,
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)>,
    pub block_ids: Vec<u64>,
    pub observer: Option<Sender<MsgT>>,
}

impl<MsgT> Floodplain<MsgT> {
    pub fn new(range: BlockRange, seed: Option<u64>) -> Self {
        Floodplain {
            seed,
            range,
            screen: None,
            flood: Flood::Color(Color::default()),
            touch_adapters: Vec::new(),
            block_ids: Vec::new(),
            observer: None,
        }
    }

    pub fn touch(&mut self, touch_msg: TouchMsg) {
        if let Some(ref observer) = self.observer {
            if let Some(touch_msg_adapter) = self.find_touch_adapter(touch_msg.tag()) {
                let msg = touch_msg_adapter(touch_msg);
                observer.send(msg).unwrap();
            }
        }
    }

    pub fn cycle(&mut self) {
        self.touch_adapters.clear();
        if let (&Some(ref screen), &Some(seed)) = (&self.screen, &self.seed) {
            self.block_ids.clear();
            let mut blocklist = build_blocklist(&self.range, &self.flood);

            self.touch_adapters.append(&mut blocklist.touch_adapters);
            if let Some(ref observer) = self.observer {
                blocklist.raft_msgs.into_iter()
                         .for_each(|msg| {
                             observer.send(msg).unwrap();
                         });
            }
            let mut block_ids = blocklist.blocks.into_iter()
                                         .enumerate()
                                         .map(|(i, block)| {
                                             let block_id = seed + (i as u64);
                                             let msg = ScreenMsg::AddBlock(block_id, block);
                                             screen.send(msg).unwrap();
                                             block_id
                                         })
                                         .collect::<Vec<_>>();
            self.block_ids.append(&mut block_ids);

            // TODO Erase block that were not overwritten.
        }
    }

    fn find_touch_adapter(&self, recipient_tag: u64) -> Option<Arc<Fn(TouchMsg) -> MsgT>> {
        let some_adapter = self.touch_adapters.iter()
                               .find(|&&(tag, _)| {
                                   tag == recipient_tag
                               });
        if let Some(&(_, ref adapter)) = some_adapter {
            Some(adapter.clone())
        } else {
            None
        }
    }
}
