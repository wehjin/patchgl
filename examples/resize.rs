extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{DirectorMsg, Screen, ScreenMsg};
use patchgl::{Block, Color, Sigil, WebColor};
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (director_msg_sender, director_msg_receiver) = channel::<DirectorMsg>();

    let width = 320;
    let height = 400;

    thread::spawn(move || {
        let mut opt_screen: Option<Screen> = None;
        while let None = opt_screen {
            if let Ok(DirectorMsg::ScreenReady(new_screen)) = director_msg_receiver.recv() {
                opt_screen = Some(new_screen);
            }
        }

        let mut screen = opt_screen.unwrap();
        send_scene_to_screen(&screen);

        let mut done = false;
        while !done {
            if let Ok(director_msg) = director_msg_receiver.recv() {
                match director_msg {
                    DirectorMsg::ScreenReady(_) => panic!("Duplicate ScreenReady"),
                    DirectorMsg::ScreenResized(new_width, new_height) => {
                        screen.width = new_width;
                        screen.height = new_height;
                        send_scene_to_screen(&screen);
                    }
                    DirectorMsg::ScreenClosed => {
                        done = true;
                    }
                }
            }
        }
    });
    patchgl::create_screen(width, height, director_msg_sender);
}

fn send_scene_to_screen(screen: &Screen) {
    let block = Block {
        sigil: Sigil::Color(Color::from_web(WebColor::Blue)),
        width: screen.width as f32,
        height: screen.height as f32,
        ..Default::default()
    };
    screen.msg_sender.send(ScreenMsg::AddBlock(1, block)).unwrap();
}

