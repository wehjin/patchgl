extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

struct MainDirector {
    width: u32,
    height: u32,
}

impl MainDirector {
    fn new(width: u32, height: u32) -> Self {
        MainDirector { width, height }
    }

    fn start(self) {
        patchgl::create_screen(self.width, self.height, self);
    }
}

impl patchgl::ScreenRunner for MainDirector {
    fn on_screen_ready(&mut self, screen: patchgl::RemoteScreen) {
        let mut screen = screen;
        use patchgl::{Block, Color, Sigil, WebColor, ScreenMessage};
        let block = Block {
            sigil: Sigil::Color(Color::from_web(WebColor::Blue)),
            width: self.width as f32,
            height: self.height as f32,
            ..Default::default()
        };
        screen.update(ScreenMessage::AddBlock(1, block));
    }
}

fn main() {
    let director = MainDirector::new(320, 480);
    director.start();
}
