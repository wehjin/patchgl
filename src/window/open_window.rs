use ::{Color, ScreenMsg, TouchMsg};
use ::flood::{Flood, Signal, Timeout, Version, Duration};
use ::window::BlockRange;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::fmt;
use super::build_blocklist;

pub struct OpenWindow<MsgT> where
    MsgT: Clone
{
    pub seed: Option<u64>,
    pub range: BlockRange,
    pub screen: Option<Sender<ScreenMsg>>,
    pub flood: Flood<MsgT>,
    pub touch_adapters: Vec<(u64, Arc<Fn(TouchMsg) -> MsgT + Send + Sync>)>,
    pub block_ids: Vec<u64>,
    pub observer: Option<Sender<MsgT>>,
    pub signals: HashMap<u64, Signal<MsgT>>,
    pub timeouts: HashMap<u64, Version<Timeout<MsgT>>>,
}

impl<MsgT> OpenWindow<MsgT> where
    MsgT: Clone + fmt::Debug + Send + 'static
{
    pub fn new(range: BlockRange, seed: Option<u64>) -> Self {
        OpenWindow {
            seed,
            range,
            screen: None,
            flood: Flood::Color(Color::default()),
            touch_adapters: Vec::new(),
            block_ids: Vec::new(),
            observer: None,
            signals: HashMap::new(),
            timeouts: HashMap::new(),
        }
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
        if let (Some(screen), Some(seed)) = (self.screen.clone(), self.seed.clone()) {
            self.block_ids.clear();
            let mut blocklist = build_blocklist(&self.range, &self.flood);

            self.touch_adapters.append(&mut blocklist.touch_adapters);
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
                                             block_id
                                         })
                                         .collect::<Vec<_>>();
            self.block_ids.append(&mut block_ids);

            // TODO Erase blocks that were not overwritten.

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
    let Duration::Seconds(secs) = timeout.duration;
    thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(secs));
        observer.send(msg).unwrap();
    });
}

