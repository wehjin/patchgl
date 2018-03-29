use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use super::DirectorMsg;

#[derive(Eq, PartialEq)]
pub enum ScanFlow {
    Break,
    Continue,
}

pub fn spawn<T, F>(carry: T, f: F) -> Sender<DirectorMsg>
    where F: Fn(DirectorMsg, T) -> (T, ScanFlow), F: Send + 'static, T: Send + 'static
{
    let (sender, receiver) = channel::<DirectorMsg>();
    thread::spawn(move || {
        scan_messages(&receiver, carry, f);
    });
    sender
}

pub fn scan_messages<T, F>(director_msg_receiver: &Receiver<DirectorMsg>, carry: T, f: F)
    where F: Fn(DirectorMsg, T) -> (T, ScanFlow)
{
    let (mut data, mut flow) = (carry, ScanFlow::Continue);
    while flow == ScanFlow::Continue {
        let (new_carry, new_flow) = match director_msg_receiver.recv() {
            Ok(msg) => f(msg, data),
            Err(_) => (data, ScanFlow::Break)
        };
        data = new_carry;
        flow = new_flow;
    }
}
