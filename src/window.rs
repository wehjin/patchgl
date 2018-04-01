use ::{director, DirectorMsg};
use ::{screen, ScreenMsg};
use ::{Anchor, Block, Sigil};
use ::flood::{Flood, Padding, Position, Touching};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;

pub enum WindowMsg {
    None,
    Flood(Flood),
    Screen(Sender<ScreenMsg>),
    Size(u32, u32),
}

pub fn create<F>(width: u32, height: u32, on_window: F) where
    F: Fn(Sender<WindowMsg>), F: Send + 'static
{
    let (window, _) = start_window(width, height);
    start_observer(on_window, window.clone());

    let director = start_director(window);
    screen::start(width, height, director);
}

fn start_director(window: Sender<WindowMsg>) -> Sender<DirectorMsg> {
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
        }
    });
    director
}

fn start_observer<F>(on_window: F, window: Sender<WindowMsg>) -> JoinHandle<()> where
    F: Fn(Sender<WindowMsg>), F: Send + 'static
{
    thread::spawn(move || on_window(window))
}

fn start_window(width: u32, height: u32) -> (Sender<WindowMsg>, JoinHandle<()>)
{
    let (window, window_msgs) = channel::<WindowMsg>();
    let window_thread = thread::spawn(move || {
        let mut floodplain = Floodplain::new(width, height);
        while let Ok(msg) = window_msgs.recv() {
            match msg {
                WindowMsg::None => (),
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
            }
        }
    });
    (window, window_thread)
}

#[derive(Default, Debug)]
struct Floodplain {
    pub width: u32,
    pub height: u32,
    pub screen: Option<Sender<ScreenMsg>>,
    pub flood: Flood,
}

impl Floodplain {
    pub fn new(width: u32, height: u32) -> Self {
        Floodplain { width, height, ..Default::default() }
    }

    pub fn cycle(&self) {
        if let Some(ref screen) = self.screen {
            let (_max_approach, blocks) = build_blocks(0., 0., self.width as f32, self.height as f32, 0.0, &self.flood);
            blocks.into_iter().enumerate().for_each(|(i, block)| {
                let msg = ScreenMsg::AddBlock(i as u64, block);
                screen.send(msg).unwrap();
            });
        }
    }
}

fn build_blocks(left: f32, top: f32, width: f32, height: f32, approach: f32, flood: &Flood) -> (f32, Vec<Block>) {
    match flood {
        &Flood::Ripple(Touching::Channel(tag, ref tracker), ref flood) => {
            let (max_approach, mut blocks) = build_blocks(left, top, width, height, approach, flood);
            let sigil = Sigil::Channel(tag, tracker.clone());
            let anchor = Anchor { x: left, y: top };
            blocks.push(Block { sigil, width, height, anchor, approach: max_approach });
            (max_approach, blocks)
        }
        &Flood::Sediment(ref silt, ref far_flood, ref near_flood) => {
            let (far_max_approach, mut blocks) = build_blocks(left, top, width, height, approach, far_flood);
            let near_approach = silt.add_to(far_max_approach);
            let (near_max_approach, near_blocks) = build_blocks(left, top, width, height, near_approach, near_flood);

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
                    let (a_max_approach, mut blocks) = build_blocks(a_left, top, a_width, height, approach, a_flood);
                    let (b_max_approach, b_blocks) = build_blocks(b_left, top, b_width, height, approach, b_flood);
                    blocks.extend(b_blocks);
                    (a_max_approach.max(b_max_approach), blocks)
                }
                &Position::Bottom(ref length) => {
                    let b_height = length.to_f32(height);
                    let a_height = height - b_height;
                    let (a_top, b_top) = (top, top + a_height);
                    let (a_max_approach, mut blocks) = build_blocks(left, a_top, width, a_height, approach, a_flood);
                    let (b_max_approach, b_blocks) = build_blocks(left, b_top, width, b_height, approach, b_flood);
                    blocks.extend(b_blocks);
                    (a_max_approach.max(b_max_approach), blocks)
                }
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
