use std::sync::mpsc::{channel, Sender};
use std::thread;

pub fn spawn<InT, OutT, F>(output_sender: &Sender<OutT>, convert: F) -> Sender<InT> where
    InT: Send + 'static,
    OutT: Send + 'static,
    F: Fn(InT) -> OutT, F: Send + 'static
{
    let (input_sender, input_msgs) = channel::<InT>();
    let output_sender = output_sender.clone();
    thread::spawn(move || {
        while let Ok(input_msg) = input_msgs.recv() {
            let output_msg = convert(input_msg);
            output_sender.send(output_msg).unwrap()
        }
    });
    input_sender
}
