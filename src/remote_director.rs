use std::sync::mpsc::Sender;
use super::{Director, DirectorMsg};

pub struct RemoteDirector {
    pub director_message_sender: Sender<DirectorMsg>,
}

impl Director for RemoteDirector {
    fn send_director_msg(&self, msg: DirectorMsg) {
        self.director_message_sender.send(msg).expect("send director-message");
    }
}
