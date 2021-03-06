use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use super::DirectorMsg;

#[derive(Eq, PartialEq)]
pub enum ControlFlow {
    Break,
    Continue,
}

pub fn spawn<T, F>(carry: T, f: F) -> (Sender<DirectorMsg>, JoinHandle<()>) where
    F: Fn(DirectorMsg, T) -> (T, ControlFlow), F: Send + 'static,
    T: Send + 'static
{
    let (director, director_msgs) = channel::<DirectorMsg>();
    let director_thread = thread::spawn(move || {
        scan_messages(&director_msgs, carry, f);
    });
    (director, director_thread)
}

pub fn scan_messages<T, F>(director_msgs: &Receiver<DirectorMsg>, carry: T, f: F) -> T where
    F: Fn(DirectorMsg, T) -> (T, ControlFlow)
{
    let (mut active_carry, mut flow) = (carry, ControlFlow::Continue);
    while flow == ControlFlow::Continue {
        let (new_carry, new_flow) = match director_msgs.recv() {
            Ok(msg) => f(msg, active_carry),
            Err(_) => (active_carry, ControlFlow::Break)
        };
        active_carry = new_carry;
        flow = new_flow;
    }
    active_carry
}
