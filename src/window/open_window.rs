use ::{Color, ScreenMsg, TouchMsg};
use ::flood::{Duration, Flood, Input, Signal, Timeout, Version};
use ::scribe::Scribe;
use ::window::{BlockRange, VirtualKeyCode};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use super::build_blocklist;
use super::keymap;


pub struct OpenWindow<'a, MsgT> where
    MsgT: Clone
{
    pub seed: Option<u64>,
    pub range: BlockRange,
    pub screen: Option<Sender<ScreenMsg>>,
    pub flood: Flood<MsgT>,
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)>,
    pub input_adapters: Vec<Arc<Fn(Input) -> MsgT + Send + Sync>>,
    pub block_ids: Vec<u64>,
    pub observer: Option<Sender<MsgT>>,
    pub signals: HashMap<u64, Signal<MsgT>>,
    pub timeouts: HashMap<u64, Version<Timeout<MsgT>>>,
    pub title: Option<String>,
    pub scribe: Scribe<'a>,
}

impl<'a, MsgT> OpenWindow<'a, MsgT> where
    MsgT: Clone + fmt::Debug + Send + 'static
{
    pub fn new(range: BlockRange, seed: Option<u64>) -> Self {
        OpenWindow {
            seed,
            range,
            screen: None,
            flood: Flood::Color(Color::default()),
            touch_adapters: Vec::new(),
            input_adapters: Vec::new(),
            block_ids: Vec::new(),
            observer: None,
            signals: HashMap::new(),
            timeouts: HashMap::new(),
            title: None,
            scribe: Scribe::default(),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = Some(title.into());
    }

    fn send_title_to_screen(&self) {
        if let (&Some(ref title), &Some(ref screen)) = (&self.title, &self.screen) {
            screen.send(ScreenMsg::Title(title.to_owned())).unwrap();
        }
    }

    pub fn set_screen(&mut self, screen: Sender<ScreenMsg>) {
        self.screen = Some(screen);
        self.send_title_to_screen();
        self.cycle();
    }

    pub fn press_key(&mut self, keycode: VirtualKeyCode) {
        if let Some(ref observer) = self.observer {
            if let Some(ref string) = keymap::code_to_string(keycode) {
                self.send_input_msg(Input::Insert(string.to_owned()), observer);
            } else if keycode == VirtualKeyCode::Back {
                self.send_input_msg(Input::DeleteBack, observer);
            } else {
                println!("Ignored key {:?}", keycode);
            }
        }
    }

    fn send_input_msg(&self, input_msg: Input, observer: &Sender<MsgT>) {
        self.input_adapters.iter().for_each(|adapter| {
            let msg = (adapter)(input_msg.clone());
            observer.send(msg).unwrap();
        });
    }

    pub fn touch(&mut self, touch_msg: TouchMsg) {
        if let Some(ref observer) = self.observer {
            if let Some(touch_msg_adapter) = self.find_touch_adapter(touch_msg.tag()) {
                let msg = touch_msg_adapter(touch_msg);
                observer.send(msg).unwrap();
            }
        }
    }

    pub fn cycle(&mut self) {
        self.touch_adapters.clear();
        self.input_adapters.clear();
        if let (Some(screen), Some(seed)) = (self.screen.clone(), self.seed.clone()) {
            let mut old_blocks: HashSet<u64> = self.block_ids.drain(..).collect();

            let mut blocklist = build_blocklist(&self.range, &self.flood, &self.scribe);

            self.touch_adapters.append(&mut blocklist.touch_adapters);
            self.input_adapters.append(&mut blocklist.input_adapters);

            if let Some(ref observer) = self.observer {
                blocklist.raft_msgs.into_iter()
                    .for_each(|msg| {
                        observer.send(msg).unwrap();
                    });
            }

            let mut block_ids = blocklist.blocks.into_iter()
                .enumerate()
                .map(|(i, block)| {
                    let block_id = seed + (i as u64);
                    let msg = ScreenMsg::AddBlock(block_id, block);
                    screen.send(msg).unwrap();
                    old_blocks.remove(&block_id);
                    block_id
                })
                .collect::<Vec<_>>();
            self.block_ids.append(&mut block_ids);

            // TODO Optimize with start- and end-cycle ScreenMsgs.
            old_blocks.into_iter().for_each(|it| {
                use Block;
                let msg = ScreenMsg::AddBlock(it, Block::default());
                screen.send(msg).unwrap();
            });

            self.cycle_signals(blocklist.signals);
            self.cycle_timeouts(blocklist.timeouts);
        }
    }

    fn cycle_timeouts(&mut self, timeout_versions: Vec<Version<Timeout<MsgT>>>) {
        timeout_versions.into_iter().for_each(|timeout_version| {
            let id = timeout_version.value.id;
            if let Some(ref observer) = self.observer {
                let old_timeout_version = self.timeouts.get(&id);
                if timeout_version.upgrades_option(&old_timeout_version) {
                    start_timeout_timer(&timeout_version.value, observer.clone());
                }
            }
            let timeouts = &mut self.timeouts;
            timeouts.insert(id, timeout_version);
        });
    }

    fn cycle_signals(&mut self, signals: Vec<Signal<MsgT>>) {
        let mut go_msgs = Vec::new();
        signals.into_iter().for_each(|signal| {
            let id = signal.id;
            {
                let old_signal = self.signals.get(&id);
                if signal.upgrades_option(&old_signal) {
                    go_msgs.push(signal.clone_value())
                }
            }
            self.set_signal(id, signal);
        });
        if let Some(ref observer) = self.observer {
            go_msgs.into_iter()
                .for_each(|msg| {
                    observer.send(msg).unwrap();
                });
        }
    }

    fn set_signal(&mut self, id: u64, signal: Signal<MsgT>) {
        let signals = &mut self.signals;
        signals.insert(id, signal);
    }

    fn find_touch_adapter(&self, recipient_tag: u64) -> Option<Arc<Fn(TouchMsg) -> MsgT>> {
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

fn start_timeout_timer<MsgT>(timeout: &Timeout<MsgT>, observer: Sender<MsgT>) where
    MsgT: Clone + fmt::Debug + Send + 'static
{
    use std::{thread, time};

// TODO Use one or a pool of threads for all timeouts.
    let msg = timeout.msg.clone();
    let duration = timeout.duration;
    thread::spawn(move || {
        const MILLIS_LOWER_LIMIT: u64 = 300;
        match duration {
            Duration::Seconds(secs) => thread::sleep(time::Duration::from_secs(secs)),
            Duration::Milliseconds(millis) => thread::sleep(time::Duration::from_millis(millis.max(MILLIS_LOWER_LIMIT)))
        };
        observer.send(msg).unwrap();
    });
}

