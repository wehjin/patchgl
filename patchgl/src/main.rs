#[macro_use]
extern crate glium;
extern crate xml;
extern crate cage;
extern crate patchgllib;
extern crate rusttype;
extern crate arrayvec;

use patchgllib::{run, RemoteScreen};
use std::thread;

fn main() {
    run(320, 480, |screen: &RemoteScreen| {
        use std::time::Duration;
        thread::sleep(Duration::from_secs(10));
        screen.close()
    });
}
