use ::{director, DirectorMsg, screen, ScreenMsg};
use ::{Anchor, Block, Color, Sigil};
pub use self::length::Length;
use std::ops::Sub;
use std::sync::mpsc::Sender;

mod length;

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
    Vessel(Thickness, Box<Flood>),
}

impl Sub<Thickness> for Flood {
    type Output = Flood;

    fn sub(self, rhs: Thickness) -> <Self as Sub<Thickness>>::Output {
        Flood::Vessel(rhs, Box::new(self))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Thickness {
    Uniform(Length),
    Dual(Length, Length),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Position {
    BottomMinusLength(Length)
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
        &Flood::Barrier(ref position, ref a_flood, ref b_flood) => {
            let &Position::BottomMinusLength(ref length) = position;
            let bottom_height = length.to_f32();
            let top_height = height - bottom_height;
            let barrier_y = top + top_height;
            let mut blocks = build_blocks(left, top, width, top_height, a_flood);
            blocks.append(&mut build_blocks(left, barrier_y, width, bottom_height, b_flood));
            blocks
        }
        &Flood::Vessel(ref thickness, ref flood) => {
            let build_blocks_with_padding = |h_pad: f32, v_pad: f32| {
                let (core_left, core_top) = (left + h_pad, top + v_pad);
                let (core_width, core_height) = (width - 2.0 * h_pad, height - 2.0 * v_pad);
                build_blocks(core_left, core_top, core_width.max(0.0), core_height.max(0.0), flood)
            };
            match thickness {
                &Thickness::Dual(ref h_length, ref v_length) => {
                    let (h_pad, v_pad) = (h_length.to_f32(), v_length.to_f32());
                    build_blocks_with_padding(h_pad, v_pad)
                }
                &Thickness::Uniform(ref length) => {
                    let pad = length.to_f32();
                    build_blocks_with_padding(pad, pad)
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
