use ::{director, DirectorMsg};
use ::{screen, ScreenMsg};
use ::{Anchor, Block, Color, Sigil};
use ::flood::*;
pub use ::screen::MAX_APPROACH;
use ::TouchMsg;
pub use self::blocklist::Blocklist;
pub use self::blockrange::BlockRange;
pub use self::open_window::*;
pub use ::VirtualKeyCode;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::fmt;
use ::scribe::Scribe;

mod blockrange;
mod blocklist;
mod open_window;
mod keymap;

pub enum WindowMsg<MsgT> where
    MsgT: Clone
{
    Flood(Flood<MsgT>),
    Observe(Sender<MsgT>),
    WindowNote(WindowNote),
    Title(String),
}

pub enum WindowNote {
    Screen(Sender<ScreenMsg>),
    Range(f32, f32, f32, f32),
    Touch(TouchMsg),
    Key(VirtualKeyCode),
}

pub fn start<MsgT, F>(width: u32, height: u32, on_start: F) where
    MsgT: Clone + fmt::Debug + Send + Sync + 'static,
    F: Fn(Sender<WindowMsg<MsgT>>), F: Send + Sync + 'static,
{
    let range = BlockRange {
        left: 0.0,
        top: 0.0,
        width: width as f32,
        height: height as f32,
        approach: 0.0,
    };
    let window = spawn_window::<MsgT>(range, Some(0));
    {
        let window = window.clone();
        thread::spawn(move || {
            on_start(window);
        });
    }

    let send_window_note = move |window_note| {
        window.send(WindowMsg::WindowNote(window_note)).unwrap();
    };
    let (director, _) = director::spawn((), move |msg, _| {
        match msg {
            DirectorMsg::ScreenReady(next_screen) => {
                send_window_note(WindowNote::Screen(next_screen));
                ((), director::ControlFlow::Continue)
            }
            DirectorMsg::ScreenResized(new_width, new_height) => {
                send_window_note(WindowNote::Range(0.0, 0.0, new_width as f32, new_height as f32));
                ((), director::ControlFlow::Continue)
            }
            DirectorMsg::ScreenClosed => {
                ((), director::ControlFlow::Break)
            }
            DirectorMsg::TouchMsg(touch_msg) => {
                send_window_note(WindowNote::Touch(touch_msg));
                ((), director::ControlFlow::Continue)
            }
            DirectorMsg::KeyPressed(keycode) => {
                send_window_note(WindowNote::Key(keycode));
                ((), director::ControlFlow::Continue)
            }
        }
    });
    screen::start(width, height, director);
}

fn spawn_window<MsgT>(range: BlockRange, seed: Option<u64>) -> Sender<WindowMsg<MsgT>> where
    MsgT: Clone + fmt::Debug + Send + Sync + 'static,
{
    let (window, window_msgs) = channel::<WindowMsg<MsgT>>();
    thread::spawn(move || {
        let mut open_window = OpenWindow::new(range, seed);

        while let Ok(msg) = window_msgs.recv() {
            match msg {
                WindowMsg::Title(string) => {
                    open_window.set_title(&string);
                }
                WindowMsg::Flood(flood) => {
                    open_window.flood = flood;
                    open_window.cycle();
                }
                WindowMsg::Observe(observer) => {
                    open_window.observer = Some(observer);
                    open_window.cycle();
                }
                WindowMsg::WindowNote(window_msg) => {
                    match window_msg {
                        WindowNote::Screen(screen) => {
                            open_window.set_screen(screen);
                        }
                        WindowNote::Range(left, top, width, height) => {
                            open_window.range.left = left;
                            open_window.range.top = top;
                            open_window.range.width = width;
                            open_window.range.height = height;
                            open_window.cycle();
                        }
                        WindowNote::Touch(touch_msg) => {
                            open_window.touch(touch_msg);
                        }
                        WindowNote::Key(keycode) => {
                            open_window.press_key(keycode);
                        }
                    }
                }
            }
        }
    });
    window
}

pub fn build_blocklist<'a, MsgT>(range: &BlockRange, flood: &Flood<MsgT>, scribe: &Scribe<'a>) -> Blocklist<MsgT> where
    MsgT: Clone
{
    match flood {
        &Flood::Escape(ref raft) => {
            let mut blocklist = build_placeholder_blocklist::<MsgT>(range, scribe);
            let &Raft::RangeAdapter(tag, ref range_adapter) = raft;
            let raft_msg = range_adapter(tag, &range.with_approach(blocklist.max_approach + 1.0));
            blocklist.raft_msgs.push(raft_msg);
            blocklist
        }
        &Flood::Ripple(Sensor::Timeout(ref versioned_timeout), ref flood) => {
            let mut blocklist = build_blocklist(range, flood, scribe);
            blocklist.timeouts.push(versioned_timeout.clone());
            blocklist
        }
        &Flood::Ripple(Sensor::Signal(ref signal), ref flood) => {
            let mut blocklist = build_blocklist(range, flood, scribe);
            blocklist.signals.push(signal.clone());
            blocklist
        }
        &Flood::Ripple(Sensor::Input(ref adapter), ref flood) => {
            let mut blocklist = build_blocklist(range, flood, scribe);
            blocklist.input_adapters.push(adapter.clone());
            blocklist
        }
        &Flood::Ripple(Sensor::Touch(tag, ref adapter), ref flood) => {
            let mut blocklist = build_blocklist(range, flood, scribe);
            let block = Block {
                sigil: Sigil::Touch(tag),
                width: range.width,
                height: range.height,
                anchor: Anchor { x: range.left, y: range.top },
                approach: blocklist.max_approach,
            };
            let touch_adapter = (tag, adapter.clone());
            blocklist.push_block(block);
            blocklist.touch_adapters.push(touch_adapter);
            blocklist
        }
        &Flood::Sediment(ref silt, ref far_flood, ref near_flood) => {
            let mut far_blocklist = build_blocklist(range, far_flood, scribe);
            let near_approach = silt.add_to(far_blocklist.max_approach);
            let mut near_blocklist = build_blocklist(&range.with_approach(near_approach), near_flood, scribe);
            far_blocklist.append(&mut near_blocklist)
        }
        &Flood::Vessel(ref thickness, ref flood) => {
            match thickness {
                &Padding::Behind(ref length) => {
                    let a_pad = length.to_f32(MAX_APPROACH - 2.0, 0.0, scribe);
                    build_blocklist(&range.with_more_approach(a_pad), flood, scribe)
                }
                &Padding::Uniform(ref length) => {
                    let pad = length.to_f32(range.width.max(range.height), range.width.min(range.height), scribe);
                    build_blocklist(&range.with_padding(pad, pad), flood, scribe)
                }
                &Padding::Dual(ref h_length, ref v_length) => {
                    let h_pad = h_length.to_f32(range.width, range.height, scribe);
                    let v_pad = v_length.to_f32(range.height, range.width, scribe);
                    build_blocklist(&range.with_padding(h_pad, v_pad), flood, scribe)
                }
                &Padding::Horizontal(ref length) => {
                    let h_pad = length.to_f32(range.width, range.height, scribe);
                    build_blocklist(&range.with_padding(h_pad, 0.0), flood, scribe)
                }
                &Padding::Vertical(ref length) => {
                    let v_pad = length.to_f32(range.height, range.width, scribe);
                    build_blocklist(&range.with_padding(0.0, v_pad), flood, scribe)
                }
            }
        }
        &Flood::Barrier(ref position, ref a_flood, ref b_flood) => {
            match position {
                &Position::Left(ref length) => {
                    let left_width = length.to_f32(range.width, range.height, scribe);
                    let (left_range, right_range) = range.split_width(range.width - left_width);
                    build_blocklist(&right_range, a_flood, scribe).append(&mut build_blocklist(&left_range, b_flood, scribe))
                }
                &Position::Top(ref length) => {
                    let top_height = length.to_f32(range.height, range.width, scribe);
                    let (top_range, bottom_range) = range.split_height(range.height - top_height);
                    build_blocklist(&bottom_range, a_flood, scribe).append(&mut build_blocklist(&top_range, b_flood, scribe))
                }
                &Position::Right(ref length) => {
                    let right_width = length.to_f32(range.width, range.height, scribe);
                    let (left_range, right_range) = range.split_width(right_width);
                    build_blocklist(&left_range, a_flood, scribe).append(&mut build_blocklist(&right_range, b_flood, scribe))
                }
                &Position::Bottom(ref length) => {
                    let bottom_height = length.to_f32(range.height, range.width, scribe);
                    let (top_range, bottom_range) = range.split_height(bottom_height);
                    build_blocklist(&top_range, a_flood, scribe).append(&mut build_blocklist(&bottom_range, b_flood, scribe))
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
                ..Default::default()
            }
        }
        &Flood::Color(color) => {
            let &BlockRange { left, top, width, height, approach } = range;
            let sigil = Sigil::Color(color);
            Blocklist {
                max_approach: approach,
                blocks: vec![Block { sigil, width, height, anchor: Anchor { x: left, y: top }, approach }],
                ..Default::default()
            }
        }
    }
}

fn build_placeholder_blocklist<'a, MsgT>(range: &BlockRange, scribe: &Scribe<'a>) -> Blocklist<MsgT> where
    MsgT: Clone
{
    let placeholder_flood = Flood::Color(Color::grey());
    let mut blocklist = build_blocklist(range, &placeholder_flood, scribe);
    blocklist.update_max_approach(range.approach);
    blocklist
}
