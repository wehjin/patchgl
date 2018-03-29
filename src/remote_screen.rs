use glium::glutin::EventsLoopProxy;
use std::sync::mpsc::{Receiver, Sender};
use super::{Block, DirectorMessage, ScreenMessage};

pub struct RemoteScreen {
    pub sender: Sender<ScreenMessage>,
    pub _receiver: Receiver<DirectorMessage>,
    pub events_loop_proxy: EventsLoopProxy,
}

impl RemoteScreen {
    pub fn add_block(&self, id: u64, block: Block) {
        self.sender.send(ScreenMessage::AddBlock(id, block)).expect("send add-block");
        self.events_loop_proxy.wakeup().expect("wakeup after add-block");
    }
    pub fn close(&self) {
        self.sender.send(ScreenMessage::Close).expect("send close");
        self.events_loop_proxy.wakeup().expect("wakeup after close");
    }
}
