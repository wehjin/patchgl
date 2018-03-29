use std::sync::mpsc::{Receiver, Sender};
use super::{DirectorMessage, ScreenMessage};

pub struct RemoteDirector {
    pub _director_message_sender: Sender<DirectorMessage>,
    pub screen_message_receiver: Receiver<ScreenMessage>,
}
