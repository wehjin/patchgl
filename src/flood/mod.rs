use ::{director, DirectorMsg, screen, ScreenMsg};
use ::{Anchor, Block, Color, Sigil};
pub use self::length::Length;
use std::ops::{Add, BitAnd, Sub};
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
    Sediment(Silt, Box<Flood>, Box<Flood>),
}

impl Sub<Thickness> for Flood {
    type Output = Flood;

    fn sub(self, rhs: Thickness) -> <Self as Sub<Thickness>>::Output {
        Flood::Vessel(rhs, Box::new(self))
    }
}

impl BitAnd<(Silt, Flood)> for Flood {
    type Output = Flood;

    fn bitand(self, (silt, far): (Silt, Flood)) -> <Self as BitAnd<(Silt, Flood)>>::Output {
        Flood::Sediment(silt, Box::new(far), Box::new(self))
    }
}

impl BitAnd<Flood> for Flood {
    type Output = Flood;

    fn bitand(self, rhs: Flood) -> <Self as BitAnd<Flood>>::Output {
        self & (Silt::Minimum, rhs)
    }
}

impl Add<(Position, Flood)> for Flood {
    type Output = Flood;

    fn add(self, (position, flood): (Position, Flood)) -> <Self as Add<(Position, Flood)>>::Output {
        Flood::Barrier(position, Box::new(self), Box::new(flood))
    }
}


#[derive(Clone, PartialEq, Debug)]
pub enum Silt {
    Minimum,
}

impl Silt {
    pub fn add_to(&self, rear_approach: f32) -> f32 {
        match self {
            &Silt::Minimum => rear_approach + 1.0,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Thickness {
    Uniform(Length),
    Dual(Length, Length),
    Horizontal(Length),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Position {
    Bottom(Length),
    BottomBar,
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
            let (_, blocks) = build_blocks(0., 0., self.width as f32, self.height as f32, 0.0, flood);
            blocks.into_iter().enumerate().for_each(|(i, block)| {
                let msg = ScreenMsg::AddBlock(i as u64, block);
                screen.send(msg).unwrap();
            });
        }
    }
}

fn build_blocks(left: f32, top: f32, width: f32, height: f32, approach: f32, flood: &Flood) -> (f32, Vec<Block>) {
    match flood {
        &Flood::Sediment(ref silt, ref far_flood, ref near_flood) => {
            let (far_max_approach, far_blocks) = build_blocks(left, top, width, height, approach, far_flood);
            let near_approach = silt.add_to(far_max_approach);
            let (near_max_approach, near_blocks) = build_blocks(left, top, width, height, near_approach, near_flood);
            let mut blocks = Vec::from(far_blocks);
            blocks.extend(near_blocks);
            (near_max_approach.max(far_max_approach), blocks)
        }
        &Flood::Vessel(ref thickness, ref flood) => {
            let build_blocks_with_padding = |h_pad: f32, v_pad: f32| {
                let (core_left, core_top) = (left + h_pad, top + v_pad);
                let (core_width, core_height) = (width - 2.0 * h_pad, height - 2.0 * v_pad);
                build_blocks(core_left, core_top, core_width.max(0.0), core_height.max(0.0), approach, flood)
            };
            match thickness {
                &Thickness::Dual(ref h_length, ref v_length) => {
                    build_blocks_with_padding(h_length.to_f32(), v_length.to_f32())
                }
                &Thickness::Uniform(ref length) => {
                    let pad = length.to_f32();
                    build_blocks_with_padding(pad, pad)
                }
                &Thickness::Horizontal(ref length) => {
                    build_blocks_with_padding(length.to_f32(), 0.0)
                }
            }
        }
        &Flood::Barrier(ref position, ref a_flood, ref b_flood) => {
            let build_blocks_with_bottom_length = |length: &Length| {
                let bottom_height = length.to_f32();
                let top_height = height - bottom_height;
                let barrier_y = top + top_height;
                let (a_max_approach, a_blocks) = build_blocks(left, top, width, top_height, approach, a_flood);
                let (b_max_approach, b_blocks) = build_blocks(left, barrier_y, width, bottom_height, approach, b_flood);
                let mut blocks = Vec::from(a_blocks);
                blocks.extend(b_blocks);
                (a_max_approach.max(b_max_approach), blocks)
            };
            match position {
                &Position::Bottom(ref length) => build_blocks_with_bottom_length(length),
                &Position::BottomBar => build_blocks_with_bottom_length(&Length::BottomBarHeight),
            }
        }
        &Flood::Text(ref string, color) => {
            let sigil = Sigil::Paragraph { line_height: height, text: string.to_owned(), color };
            (approach, vec![Block { sigil, width, height, anchor: Anchor { x: left, y: top }, approach }])
        }
        &Flood::Color(color) => {
            let sigil = Sigil::Color(color);
            let block = Block { sigil, width, height, anchor: Anchor { x: left, y: top }, approach };
            (approach, vec![block])
        }
    }
}

impl Default for Plains {
    fn default() -> Self {
        Plains { width: 0, height: 0, screen: None }
    }
}
