use ::{director, DirectorMsg, screen, ScreenMsg};
use ::{Anchor, Block, Color, Sigil};
use std::sync::mpsc::Sender;

pub fn render_forever(width: u32, height: u32, flood: Flood) {
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

#[derive(Clone, Debug)]
pub enum Flood {
    Color(Color),
    Text(String, Color),
    Barrier(Position, Box<Flood>, Box<Flood>),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Position {
    BottomMinusLength(Length)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Length {
    FingerTip,
    Pixels(f32),
}

impl Length {
    pub fn to_f32(&self) -> f32 {
        match self {
            &Length::FingerTip => 44.0,
            &Length::Pixels(pixels) => pixels,
        }
    }
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
            let blocks = build_blocks(0., 0., self.width as f32, self.height as f32, flood);
            blocks.into_iter().enumerate().for_each(|(i, block)| {
                let msg = ScreenMsg::AddBlock(i as u64, block);
                screen.send(msg).unwrap();
            });
        }
    }
}

fn build_blocks(left: f32, top: f32, width: f32, height: f32, flood: &Flood) -> Vec<Block> {
    match flood {
        &Flood::Color(color) => {
            let sigil = Sigil::Color(color);
            let block = Block { sigil, width, height, anchor: Anchor { x: left, y: top }, ..Default::default() };
            println!("Colorblock: {:?}", block);
            vec![block]
        }
        &Flood::Text(ref string, color) => {
            let sigil = Sigil::Paragraph { line_height: height, text: string.to_owned(), color };
            vec![Block { sigil, width, height, anchor: Anchor { x: left, y: top }, ..Default::default() }]
        }
        &Flood::Barrier(position, ref a_flood, ref b_flood) => {
            let Position::BottomMinusLength(length) = position;
            let bottom_height = length.to_f32();
            let top_height = height - bottom_height;
            let barrier_y = top + top_height;
            let mut blocks = build_blocks(left, top, width, top_height, a_flood);
            blocks.append(&mut build_blocks(left, barrier_y, width, bottom_height, b_flood));
            blocks
        }
    }
}

impl Default for Plains {
    fn default() -> Self {
        Plains { width: 0, height: 0, screen: None }
    }
}
