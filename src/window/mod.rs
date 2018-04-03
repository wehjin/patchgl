use ::{director, DirectorMsg};
use ::{screen, ScreenMsg};
use ::{Anchor, Block, Color, Sigil};
use ::flood::*;
use ::TouchMsg;
pub use self::blocklist::Blocklist;
pub use self::blockrange::BlockRange;
pub use self::floodplain::*;
use std::sync::mpsc::{channel, Sender};
use std::thread;

mod blockrange;
mod blocklist;
mod floodplain;

pub enum WindowNote {
    Screen(Sender<ScreenMsg>),
    Range(f32, f32, f32, f32),
    Touch(TouchMsg),
}

pub fn start<MsgT, F>(width: u32, height: u32, notify_floodplain: F) where
    MsgT: Send + Sync + 'static,
    F: Fn(Sender<FloodplainMsg<MsgT>>), F: Send + Sync + 'static,
{
    let range = BlockRange {
        left: 0.0,
        top: 0.0,
        width: width as f32,
        height: height as f32,
        approach: 0.0,
    };
    let floodplain = spawn_floodplain::<MsgT>(range, Some(0));
    {
        let floodplain = floodplain.clone();
        thread::spawn(move || {
            notify_floodplain(floodplain);
        });
    }

    let send_window_note = move |window_note| {
        floodplain.send(FloodplainMsg::WindowNote(window_note)).unwrap();
    };
    let (director, _) = director::spawn((), move |msg, _| {
        match msg {
            DirectorMsg::ScreenReady(next_screen) => {
                send_window_note(WindowNote::Screen(next_screen));
                ((), director::ScanFlow::Continue)
            }
            DirectorMsg::ScreenResized(new_width, new_height) => {
                send_window_note(WindowNote::Range(0.0, 0.0, new_width as f32, new_height as f32));
                ((), director::ScanFlow::Continue)
            }
            DirectorMsg::ScreenClosed => {
                ((), director::ScanFlow::Break)
            }
            DirectorMsg::TouchMsg(touch_msg) => {
                send_window_note(WindowNote::Touch(touch_msg));
                ((), director::ScanFlow::Continue)
            }
        }
    });
    screen::start(width, height, director);
}

fn spawn_floodplain<MsgT>(range: BlockRange, seed: Option<u64>) -> Sender<FloodplainMsg<MsgT>> where
    MsgT: Send + Sync + 'static,
{
    let (floodplain, floodplain_msgs) = channel::<FloodplainMsg<MsgT>>();
    thread::spawn(move || {
        let mut floodplain = Floodplain::new(range, seed);
        while let Ok(msg) = floodplain_msgs.recv() {
            match msg {
                FloodplainMsg::Flood(flood) => {
                    floodplain.flood = flood;
                    floodplain.cycle();
                }
                FloodplainMsg::Observe(observer) => {
                    floodplain.observer = Some(observer);
                    floodplain.cycle();
                }
                FloodplainMsg::WindowNote(window_msg) => {
                    match window_msg {
                        WindowNote::Screen(screen) => {
                            floodplain.screen = Some(screen);
                            floodplain.cycle();
                        }
                        WindowNote::Range(left, top, width, height) => {
                            floodplain.range.left = left;
                            floodplain.range.top = top;
                            floodplain.range.width = width;
                            floodplain.range.height = height;
                            floodplain.cycle();
                        }
                        WindowNote::Touch(touch_msg) => {
                            floodplain.touch(touch_msg);
                        }
                    }
                }
            }
        }
    });
    floodplain
}

pub fn build_blocklist<MsgT>(range: &BlockRange, flood: &Flood<MsgT>) -> Blocklist<MsgT>
{
    match flood {
        &Flood::Escape(ref raft) => {
            let mut blocklist = build_placeholder_blocklist::<MsgT>(range);
            let &Raft::RangeAdapter(tag, ref range_adapter) = raft;
            let raft_msg = range_adapter(tag, &range.with_approach(blocklist.max_approach + 1.0));
            blocklist.push_raft_msg(raft_msg);
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
                raft_msgs: Vec::new(),
            }
        }
        &Flood::Color(color) => {
            let &BlockRange { left, top, width, height, approach } = range;
            let sigil = Sigil::Color(color);
            Blocklist {
                max_approach: approach,
                blocks: vec![Block { sigil, width, height, anchor: Anchor { x: left, y: top }, approach }],
                touch_adapters: Vec::new(),
                raft_msgs: Vec::new(),
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
