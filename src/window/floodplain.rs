use ::{Color, ScreenMsg, TouchMsg};
use ::flood::Flood;
use ::window::BlockRange;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use super::build_blocklist;

#[derive(Default)]
pub struct Floodplain<MsgT = ()> {
    pub width: u32,
    pub height: u32,
    pub screen: Option<Sender<ScreenMsg>>,
    pub flood: Flood<MsgT>,
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)>,
}

impl<MsgT> Floodplain<MsgT> {
    pub fn new(width: u32, height: u32) -> Self {
        Floodplain { width, height, screen: None, flood: Flood::Color(Color::default()), touch_adapters: Vec::new() }
    }

    pub fn cycle(&mut self) {
        if let Some(ref screen) = self.screen {
            let (left, top, width, height, approach) = (0.0, 0.0, self.width as f32, self.height as f32, 0.0);
            let mut blocklist = build_blocklist(&BlockRange { left, top, width, height, approach }, &self.flood);
            self.touch_adapters.clear();
            self.touch_adapters.append(&mut blocklist.touch_adapters);
            blocklist.blocks.into_iter()
                     .enumerate()
                     .for_each(|(i, block)| {
                         let msg = ScreenMsg::AddBlock(i as u64, block);
                         screen.send(msg).unwrap();
                     });
        }
    }

    pub fn find_touch_adapter(&self, recipient_tag: u64) -> Option<Arc<Fn(TouchMsg) -> MsgT>> {
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
