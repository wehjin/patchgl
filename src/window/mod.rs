use ::{director, DirectorMsg};
use ::{screen, ScreenMsg};
use ::{Anchor, Block, Color, Sigil};
use ::dervish::*;
use ::flood::*;
use ::TouchMsg;
pub use self::blocklist::Blocklist;
pub use self::blockrange::BlockRange;
use self::floodplain::Floodplain;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;

mod blockrange;
mod blocklist;
mod floodplain;

pub enum WindowMsg<MsgT = ()> {
    None,
    Flood(Flood<MsgT>),
    Screen(Sender<ScreenMsg>),
    Size(u32, u32),
    TouchMsg(TouchMsg),
    Watcher(Sender<MsgT>),
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
        let mut some_watcher: Option<Sender<MsgT>> = None;
        let mut floodplain = Floodplain::new(width, height);
        while let Ok(msg) = window_msgs.recv() {
            match msg {
                WindowMsg::None => (),
                WindowMsg::Watcher(watcher) => {
                    some_watcher = Some(watcher);
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
                    if let Some(ref watcher) = some_watcher {
                        if let Some(adapter) = floodplain.find_touch_adapter(touch_msg.tag()) {
                            let msg = adapter(touch_msg);
                            watcher.send(msg).unwrap();
                        }
                    }
                }
            }
        }
    });
    (window, window_thread)
}

pub fn build_blocklist<MsgT>(range: &BlockRange, flood: &Flood<MsgT>) -> Blocklist<MsgT>
{
    match flood {
        &Flood::Dervish(ref builder) => {
            let mut blocklist = build_placeholder_blocklist(range);
            let range = range.with_more_approach(1.0);
            blocklist.push_dervish_settings(DervishSettings { range, dervish_builder: builder.clone() });
            blocklist
        }
        &Flood::Ripple(Sensor::Touch(tag, ref msg_adapter), ref flood) => {
            let mut blocklist = build_blocklist(range, flood);
            let block = Block {
                sigil: Sigil::Touch(tag),
                width: range.width,
                height: range.height,
                anchor: Anchor { x: range.left, y: range.top },
                approach: blocklist.max_approach,
            };
            let touch_adapter = (tag, msg_adapter.clone());
            blocklist.push_block(block);
            blocklist.push_touch_adapter(touch_adapter);
            blocklist
        }
        &Flood::Sediment(ref silt, ref far_flood, ref near_flood) => {
            let mut far_blocklist = build_blocklist(range, far_flood);
            let near_approach = silt.add_to(far_blocklist.max_approach);
            let mut near_blocklist = build_blocklist(&range.with_approach(near_approach), near_flood);
            far_blocklist.append(&mut near_blocklist)
        }
        &Flood::Vessel(ref thickness, ref flood) => {
            match thickness {
                &Padding::Dual(ref h_length, ref v_length) => {
                    let h_pad = h_length.to_f32(range.width);
                    let v_pad = v_length.to_f32(range.height);
                    build_blocklist(&range.with_padding(h_pad, v_pad), flood)
                }
                &Padding::Uniform(ref length) => {
                    let pad = length.to_f32(range.width.max(range.height));
                    build_blocklist(&range.with_padding(pad, pad), flood)
                }
                &Padding::Horizontal(ref length) => {
                    let h_pad = length.to_f32(range.width);
                    build_blocklist(&range.with_padding(h_pad, 0.0), flood)
                }
            }
        }
        &Flood::Barrier(ref position, ref a_flood, ref b_flood) => {
            match position {
                &Position::Right(ref length) => {
                    let right_width = length.to_f32(range.width);
                    let (left_range, right_range) = range.split_width(right_width);
                    build_blocklist(&left_range, a_flood).append(&mut build_blocklist(&right_range, b_flood))
                }
                &Position::Bottom(ref length) => {
                    let bottom_height = length.to_f32(range.height);
                    let (top_range, bottom_range) = range.split_height(bottom_height);
                    build_blocklist(&top_range, a_flood).append(&mut build_blocklist(&bottom_range, b_flood))
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
            Blocklist {
                max_approach: approach,
                blocks: vec![Block { sigil, width, height, anchor: Anchor { x: left, y: top }, approach }],
                touch_adapters: Vec::new(),
                whirlings: Vec::new(),
            }
        }
        &Flood::Color(color) => {
            let &BlockRange { left, top, width, height, approach } = range;
            let sigil = Sigil::Color(color);
            Blocklist {
                max_approach: approach,
                blocks: vec![Block { sigil, width, height, anchor: Anchor { x: left, y: top }, approach }],
                touch_adapters: Vec::new(),
                whirlings: Vec::new(),
            }
        }
    }
}

fn build_placeholder_blocklist<MsgT>(range: &BlockRange) -> Blocklist<MsgT> {
    let placeholder_flood = Flood::Color(Color::grey());
    let mut blocklist = build_blocklist(range, &placeholder_flood);
    blocklist.update_max_approach(range.approach);
    blocklist
}
