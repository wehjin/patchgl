use ::{director, DirectorMsg};
use ::{screen, ScreenMsg};
use ::{Anchor, Block, Sigil};
use ::Color;
use ::flood::{Flood, Padding, Position, Sensor};
use ::TouchMsg;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;

pub enum WindowMsg<MsgT> {
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
struct Floodplain<MsgT> {
    pub width: u32,
    pub height: u32,
    pub screen: Option<Sender<ScreenMsg>>,
    pub flood: Flood<MsgT>,
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync + 'static>)>,
}

impl<MsgT> Floodplain<MsgT> {
    pub fn new(width: u32, height: u32) -> Self {
        Floodplain { width, height, screen: None, flood: Flood::Color(Color::default()), touch_adapters: Vec::new() }
    }

    pub fn cycle(&mut self) {
        if let Some(ref screen) = self.screen {
            let mut blocklist = build_blocks(0., 0., self.width as f32, self.height as f32, 0.0, &self.flood);
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
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync + 'static>)>,
}

impl<MsgT> Blocklist<MsgT> {
    pub fn push_block(&mut self, block: Block) {
        self.max_approach = self.max_approach.max(block.approach);
        self.blocks.push(block);
    }
    pub fn push_touch_adapter(&mut self, touch_adapter: (u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync + 'static>)) {
        self.touch_adapters.push(touch_adapter);
    }
    pub fn append(&mut self, rhs: &mut Blocklist<MsgT>) {
        self.max_approach = self.max_approach.max(rhs.max_approach);
        self.blocks.append(&mut rhs.blocks);
        self.touch_adapters.append(&mut rhs.touch_adapters);
    }
}

fn build_blocks<MsgT>(left: f32, top: f32,
                      width: f32, height: f32,
                      approach: f32, flood: &Flood<MsgT>) -> Blocklist<MsgT>
{
    match flood {
        &Flood::Ripple(Sensor::Touch(tag, ref msg_adapter), ref flood) => {
            let mut blocklist = build_blocks(left, top, width, height, approach, flood);
            let sigil = Sigil::Touch(tag);
            let anchor = Anchor { x: left, y: top };
            let block = Block { sigil, width, height, anchor, approach: blocklist.max_approach };
            let touch_adapter = (tag, msg_adapter.clone());
            blocklist.push_block(block);
            blocklist.push_touch_adapter(touch_adapter);
            blocklist
        }
        &Flood::Sediment(ref silt, ref far_flood, ref near_flood) => {
            let mut blocklist = build_blocks(left, top, width, height, approach, far_flood);
            let near_approach = silt.add_to(blocklist.max_approach);
            let mut near_blocklist = build_blocks(left, top, width, height, near_approach, near_flood);
            blocklist.append(&mut near_blocklist);
            blocklist
        }
        &Flood::Vessel(ref thickness, ref flood) => {
            let build_blocks_with_padding = |h_pad: f32, v_pad: f32| {
                let (core_left, core_top) = (left + h_pad, top + v_pad);
                let (core_width, core_height) = (width - 2.0 * h_pad, height - 2.0 * v_pad);
                build_blocks(core_left, core_top, core_width.max(0.0), core_height.max(0.0), approach, flood)
            };
            match thickness {
                &Padding::Dual(ref h_length, ref v_length) => {
                    build_blocks_with_padding(h_length.to_f32(width), v_length.to_f32(height))
                }
                &Padding::Uniform(ref length) => {
                    let pad = length.to_f32(width.max(height));
                    build_blocks_with_padding(pad, pad)
                }
                &Padding::Horizontal(ref length) => {
                    build_blocks_with_padding(length.to_f32(width), 0.0)
                }
            }
        }
        &Flood::Barrier(ref position, ref a_flood, ref b_flood) => {
            match position {
                &Position::Right(ref length) => {
                    let b_width = length.to_f32(width);
                    let a_width = width - b_width;
                    let (a_left, b_left) = (left, left + a_width);
                    let mut blocklist = build_blocks(a_left, top, a_width, height, approach, a_flood);
                    let mut b_blocklist = build_blocks(b_left, top, b_width, height, approach, b_flood);
                    blocklist.append(&mut b_blocklist);
                    blocklist
                }
                &Position::Bottom(ref length) => {
                    let b_height = length.to_f32(height);
                    let a_height = height - b_height;
                    let (a_top, b_top) = (top, top + a_height);
                    let mut blocklist = build_blocks(left, a_top, width, a_height, approach, a_flood);
                    let mut b_blocklist = build_blocks(left, b_top, width, b_height, approach, b_flood);
                    blocklist.append(&mut b_blocklist);
                    blocklist
                }
            }
        }
        &Flood::Text(ref string, color, placement) => {
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
            let sigil = Sigil::Color(color);
            let block = Block { sigil, width, height, anchor: Anchor { x: left, y: top }, approach };
            Blocklist { max_approach: approach, blocks: vec![block], touch_adapters: Vec::new() }
        }
    }
}
