use ::{director, DirectorMsg};
use ::{screen, ScreenMsg};
use ::{Anchor, Block, Sigil};
use ::Color;
use ::flood::*;
use ::TouchMsg;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;

pub enum WindowMsg<MsgT = ()> {
    None,
    Flood(Flood<MsgT>),
    Screen(Sender<ScreenMsg>),
    Size(u32, u32),
    TouchMsg(TouchMsg),
    Tx(Sender<MsgT>),
}

pub fn show<F, MsgT>(width: u32, height: u32, on_window: F) where
    F: Fn(Sender<WindowMsg<MsgT>>), F: Send + 'static,
    MsgT: Send + 'static
{
    let (window, _) = start_window(width, height);
    start_observer(on_window, window.clone());

    let director = start_director(window);
    screen::start(width, height, director);
}

fn start_director<MsgT>(window: Sender<WindowMsg<MsgT>>) -> Sender<DirectorMsg> where
    MsgT: Send + 'static
{
    let (director, _) = director::spawn(window, move |msg, window| {
        match msg {
            DirectorMsg::ScreenReady(next_screen) => {
                window.send(WindowMsg::Screen(next_screen)).unwrap();
                (window, director::ScanFlow::Continue)
            }
            DirectorMsg::ScreenResized(new_width, new_height) => {
                window.send(WindowMsg::Size(new_width, new_height)).unwrap();
                (window, director::ScanFlow::Continue)
            }
            DirectorMsg::ScreenClosed => {
                (window, director::ScanFlow::Break)
            }
            DirectorMsg::TouchMsg(touch_msg) => {
                window.send(WindowMsg::TouchMsg(touch_msg)).unwrap();
                (window, director::ScanFlow::Continue)
            }
        }
    });
    director
}

fn start_observer<F, MsgT>(on_window: F, window: Sender<WindowMsg<MsgT>>) -> JoinHandle<()> where
    F: Fn(Sender<WindowMsg<MsgT>>), F: Send + 'static,
    MsgT: Send + 'static
{
    thread::spawn(move || on_window(window))
}

fn start_window<MsgT>(width: u32, height: u32) -> (Sender<WindowMsg<MsgT>>, JoinHandle<()>)
    where MsgT: Send + 'static
{
    let (window, window_msgs) = channel::<WindowMsg<MsgT>>();
    let window_thread = thread::spawn(move || {
        let mut some_observer: Option<Sender<MsgT>> = None;
        let mut floodplain = Floodplain::new(width, height);
        while let Ok(msg) = window_msgs.recv() {
            match msg {
                WindowMsg::None => (),
                WindowMsg::Tx(observer) => {
                    some_observer = Some(observer);
                }
                WindowMsg::Screen(screen) => {
                    floodplain.screen = Some(screen);
                    floodplain.cycle();
                }
                WindowMsg::Size(width, height) => {
                    floodplain.width = width;
                    floodplain.height = height;
                    floodplain.cycle();
                }
                WindowMsg::Flood(flood) => {
                    floodplain.flood = flood;
                    floodplain.cycle();
                }
                WindowMsg::TouchMsg(touch_msg) => {
                    if let Some(ref observer) = some_observer {
                        if let Some(adapter) = floodplain.find_touch_adapter(touch_msg.tag()) {
                            let msg = adapter(touch_msg);
                            observer.send(msg).unwrap();
                        }
                    }
                }
            }
        }
    });
    (window, window_thread)
}

#[derive(Default)]
struct Floodplain<MsgT = ()> {
    pub width: u32,
    pub height: u32,
    pub screen: Option<Sender<ScreenMsg>>,
    pub flood: Flood<MsgT>,
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)>,
}

impl<MsgT> Floodplain<MsgT> {
    pub fn new(width: u32, height: u32) -> Self {
        Floodplain { width, height, screen: None, flood: Flood::Color(Color::default()), touch_adapters: Vec::new() }
    }

    pub fn cycle(&mut self) {
        if let Some(ref screen) = self.screen {
            let (left, top, width, height, approach) = (0.0, 0.0, self.width as f32, self.height as f32, 0.0);
            let mut blocklist = build_blocks(&BlockRange { left, top, width, height, approach }, &self.flood);
            self.touch_adapters.clear();
            self.touch_adapters.append(&mut blocklist.touch_adapters);
            blocklist.blocks.into_iter()
                     .enumerate()
                     .for_each(|(i, block)| {
                         let msg = ScreenMsg::AddBlock(i as u64, block);
                         screen.send(msg).unwrap();
                     });
        }
    }

    pub fn find_touch_adapter(&self, recipient_tag: u64) -> Option<Arc<Fn(TouchMsg) -> MsgT>> {
        let some_adapter = self.touch_adapters.iter()
                               .find(|&&(tag, _)| {
                                   tag == recipient_tag
                               });
        if let Some(&(_, ref adapter)) = some_adapter {
            Some(adapter.clone())
        } else {
            None
        }
    }
}

struct Blocklist<MsgT> {
    pub max_approach: f32,
    pub blocks: Vec<Block>,
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)>,
}

impl<MsgT> Blocklist<MsgT> {
    pub fn push_block(&mut self, block: Block) {
        self.max_approach = self.max_approach.max(block.approach);
        self.blocks.push(block);
    }
    pub fn push_touch_adapter(&mut self, touch_adapter: (u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)) {
        self.touch_adapters.push(touch_adapter);
    }
    pub fn append(mut self, rhs: &mut Blocklist<MsgT>) -> Self {
        self.max_approach = self.max_approach.max(rhs.max_approach);
        self.blocks.append(&mut rhs.blocks);
        self.touch_adapters.append(&mut rhs.touch_adapters);
        self
    }
}

#[derive(Copy, Clone, Debug)]
struct BlockRange {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub approach: f32,
}

impl BlockRange {
    pub fn with_approach(&self, approach: f32) -> Self {
        let mut range = self.clone();
        range.approach = approach;
        range
    }

    pub fn with_padding(&self, h_pad: f32, v_pad: f32) -> Self {
        BlockRange {
            left: self.left + h_pad,
            top: self.top + v_pad,
            width: (self.width - 2.0 * h_pad).max(0.0),
            height: (self.height - 2.0 * v_pad).max(0.0),
            approach: self.approach,
        }
    }
    pub fn split_width(&self, right_width: f32) -> (Self, Self) {
        let right_width = right_width.min(self.width);
        let left_width = self.width - right_width;
        let left_range = BlockRange {
            left: self.left,
            top: self.top,
            width: left_width,
            height: self.height,
            approach: self.approach,
        };
        let right_range = BlockRange {
            left: self.left + left_width,
            top: self.top,
            width: right_width,
            height: self.height,
            approach: self.approach,
        };
        (left_range, right_range)
    }
    pub fn split_height(&self, bottom_height: f32) -> (Self, Self) {
        let bottom_height = bottom_height.min(self.height);
        let top_height = self.height - bottom_height;
        let top_range = BlockRange {
            left: self.left,
            top: self.top,
            width: self.width,
            height: top_height,
            approach: self.approach,
        };
        let bottom_range = BlockRange {
            left: self.left,
            top: self.top + top_height,
            width: self.width,
            height: bottom_height,
            approach: self.approach,
        };
        (top_range, bottom_range)
    }
}

fn build_blocks<MsgT>(range: &BlockRange, flood: &Flood<MsgT>) -> Blocklist<MsgT>
{
    match flood {
        &Flood::Dervish(Dervish::Sender(ref _sender), ref flood) => {
            build_blocks(range, flood)
        }
        &Flood::Ripple(Sensor::Touch(tag, ref msg_adapter), ref flood) => {
            let mut blocklist = build_blocks(range, flood);
            let sigil = Sigil::Touch(tag);
            let anchor = Anchor { x: range.left, y: range.top };
            let block = Block { sigil, width: range.width, height: range.height, anchor, approach: blocklist.max_approach };
            let touch_adapter = (tag, msg_adapter.clone());
            blocklist.push_block(block);
            blocklist.push_touch_adapter(touch_adapter);
            blocklist
        }
        &Flood::Sediment(ref silt, ref far_flood, ref near_flood) => {
            let mut far_blocklist = build_blocks(range, far_flood);
            let near_approach = silt.add_to(far_blocklist.max_approach);
            let mut near_blocklist = build_blocks(&range.with_approach(near_approach), near_flood);
            far_blocklist.append(&mut near_blocklist)
        }
        &Flood::Vessel(ref thickness, ref flood) => {
            match thickness {
                &Padding::Dual(ref h_length, ref v_length) => {
                    let h_pad = h_length.to_f32(range.width);
                    let v_pad = v_length.to_f32(range.height);
                    build_blocks(&range.with_padding(h_pad, v_pad), flood)
                }
                &Padding::Uniform(ref length) => {
                    let pad = length.to_f32(range.width.max(range.height));
                    build_blocks(&range.with_padding(pad, pad), flood)
                }
                &Padding::Horizontal(ref length) => {
                    let h_pad = length.to_f32(range.width);
                    build_blocks(&range.with_padding(h_pad, 0.0), flood)
                }
            }
        }
        &Flood::Barrier(ref position, ref a_flood, ref b_flood) => {
            match position {
                &Position::Right(ref length) => {
                    let right_width = length.to_f32(range.width);
                    let (left_range, right_range) = range.split_width(right_width);
                    build_blocks(&left_range, a_flood).append(&mut build_blocks(&right_range, b_flood))
                }
                &Position::Bottom(ref length) => {
                    let bottom_height = length.to_f32(range.height);
                    let (top_range, bottom_range) = range.split_height(bottom_height);
                    build_blocks(&top_range, a_flood).append(&mut build_blocks(&bottom_range, b_flood))
                }
            }
        }
        &Flood::Text(ref string, color, placement) => {
            let &BlockRange { left, top, width, height, approach } = range;
            let sigil = Sigil::Paragraph {
                line_height: height,
                text: string.to_owned(),
                color,
                placement: placement.into(),
            };
            let block = Block { sigil, width, height, anchor: Anchor { x: left, y: top }, approach };
            Blocklist { max_approach: approach, blocks: vec![block], touch_adapters: Vec::new() }
        }
        &Flood::Color(color) => {
            let &BlockRange { left, top, width, height, approach } = range;
            let sigil = Sigil::Color(color);
            let block = Block { sigil, width, height, anchor: Anchor { x: left, y: top }, approach };
            Blocklist { max_approach: approach, blocks: vec![block], touch_adapters: Vec::new() }
        }
    }
}
