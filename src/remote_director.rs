use glium::glutin::EventsLoopProxy;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use super::{DirectorMessage, RemoteScreen, ScreenMessage};

pub struct RemoteDirector {
    _sender: Sender<DirectorMessage>,
    receiver: Receiver<ScreenMessage>,
}

impl RemoteDirector {
    pub fn connect<F>(events_loop_proxy: EventsLoopProxy, on_screen_ready: F) -> Self
        where F: Fn(&RemoteScreen) -> () + Send + 'static
    {
        let (send_to_screen, receive_from_director) = channel::<ScreenMessage>();
        let (send_to_director, receive_from_screen) = channel::<DirectorMessage>();
        let director = RemoteDirector {
            _sender: send_to_director,
            receiver: receive_from_director,
        };
        thread::spawn(move || {
            let remote_screen = RemoteScreen {
                sender: send_to_screen,
                _receiver: receive_from_screen,
                events_loop_proxy,
            };
            on_screen_ready(&remote_screen)
        });
        director
    }

    pub fn receive_screen_message(&self) -> Option<ScreenMessage> {
        let result = self.receiver.try_recv();
        if result.is_ok() {
            Option::Some(result.unwrap())
        } else {
            Option::None
        }
    }
}
