use std::sync::mpsc::{Receiver, Sender};
use super::{DirectorMessage, ScreenMessage};

pub struct RemoteScreen {
    sender: Sender<ScreenMessage>,
    _receiver: Receiver<DirectorMessage>,
}

impl RemoteScreen {
    pub fn new(sender: Sender<ScreenMessage>, receiver: Receiver<DirectorMessage>) -> Self {
        RemoteScreen { sender, _receiver: receiver }
    }

    pub fn update(&mut self, screen_message: ScreenMessage) {
        match screen_message {
            ScreenMessage::AddBlock(id, block) => {
                self.sender.send(ScreenMessage::AddBlock(id, block)).expect("send add-block");
            }
            ScreenMessage::Close => {
                self.sender.send(ScreenMessage::Close).expect("send close");
            }
        }
    }
}
