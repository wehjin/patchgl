use std::sync::mpsc::{channel, Sender};
use std::thread;

pub fn connect<StartT, DestT>(dest: &Sender<DestT>) -> Sender<StartT> where
    DestT: Send + 'static + From<StartT>,
    StartT: Send + 'static
{
    start_with_transformer(dest, DestT::from)
}

pub fn start_with_transformer<StartT, DestT, F>(dest: &Sender<DestT>, f: F) -> Sender<StartT> where
    DestT: Send + 'static,
    StartT: Send + 'static,
    F: Fn(StartT) -> DestT, F: Send + 'static
{
    let (input, input_msgs) = channel::<StartT>();
    let output = dest.clone();
    thread::spawn(move || {
        while let Ok(input_msg) = input_msgs.recv() {
            let output_msg = f(input_msg);
            output.send(output_msg).unwrap()
        }
    });
    input
}
