use std::sync::mpsc::Sender;
use super::{DirectorMsg, local_screen};

pub fn start(width: u32, height: u32, director: Sender<DirectorMsg>) {
    local_screen::start(width, height, director);
}
