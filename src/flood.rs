use ::{director, DirectorMsg, screen, ScreenMsg};
use ::{Block, Color, Sigil};
use std::sync::mpsc::Sender;

#[derive(Copy, Clone, Debug)]
pub enum Flood {
    Color(Color)
}

struct Plains {
    pub width: u32,
    pub height: u32,
    pub screen: Option<Sender<ScreenMsg>>,
}

impl Plains {
    pub fn new(width: u32, height: u32) -> Self {
        Plains { width, height, screen: None }
    }

    pub fn flood(&self, flood: &Flood) {
        if let Some(ref screen) = self.screen {
            let block = self.build_block(flood);
            screen.send(ScreenMsg::AddBlock(1, block)).unwrap();
        }
    }

    fn build_block(&self, flood: &Flood) -> Block {
        match flood {
            &Flood::Color(ref color) => {
                Block {
                    sigil: Sigil::Color(*color),
                    width: self.width as f32,
                    height: self.height as f32,
                    ..Default::default()
                }
            }
        }
    }
}

impl Default for Plains {
    fn default() -> Self {
        Plains { width: 0, height: 0, screen: None }
    }
}

pub fn render(width: u32, height: u32, flood: Flood) {
    let director = director::spawn(Plains::new(width, height), move |msg, carry| {
        match msg {
            DirectorMsg::ScreenReady(next_screen) => {
                let mut plains = carry;
                plains.screen = Some(next_screen);
                plains.flood(&flood);
                (plains, director::ScanFlow::Continue)
            }
            DirectorMsg::ScreenResized(new_width, new_height) => {
                let mut plains = carry;
                plains.width = new_width;
                plains.height = new_height;
                plains.flood(&flood);
                (plains, director::ScanFlow::Continue)
            }
            DirectorMsg::ScreenClosed => {
                (Plains::default(), director::ScanFlow::Break)
            }
        }
    });
    screen::start(width, height, director);
}