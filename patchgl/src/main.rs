#[macro_use]
extern crate glium;
extern crate xml;
extern crate cage;
extern crate patchgllib;
extern crate rusttype;
extern crate arrayvec;

use patchgllib::{run, RemoteScreen, Patch};
use std::thread;
use std::time::Duration;

fn main() {
    run(320, 480, |screen: &RemoteScreen| {
        let patch = Patch::from_dimensions(320.0, 320.0, 0f32);
        screen.draw(patch);
        thread::sleep(Duration::from_secs(10));
        screen.close()
    });
}
