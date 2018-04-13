use std::sync::Arc;
use std::collections::HashMap;
use flood::*;
use material::Palette;
use material;
use material::components::button::*;
use material::components::button::Placement;
use id::*;
use traits::Update;
use flood::{Version, VersionCounter, Sensor, Signal};

#[derive(Clone, PartialEq, Debug)]
pub struct NavTrack<'a, MsgT, F, G> where
    F: Fn(NavTrackMsg) -> MsgT + Send + Sync + 'static,
    G: Fn(NavTrackEvt) -> MsgT + Send + Sync + 'static,
{
    pub nav_track_msg_wrap: F,
    pub id: u64,
    pub palette: &'a Palette,
    pub mdl: &'a NavTrackMdl,
    pub item_selected_msg_wrap: G,
    pub items: Vec<NavTrackItem>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum NavTrackEvt {
    ItemSelected(Option<usize>)
}

#[derive(Clone, PartialEq, Debug)]
pub struct NavTrackMdl {
    selected_index: Option<usize>,
    button_mdls: HashMap<u64, ButtonMdl>,
    item_selected_version_counter: VersionCounter,
}

impl Default for NavTrackMdl {
    fn default() -> Self {
        NavTrackMdl {
            selected_index: None,
            button_mdls: HashMap::new(),
            item_selected_version_counter: VersionCounter::enabled_after_bump(),
        }
    }
}

impl Update<NavTrackMsg> for NavTrackMdl {
    fn update(&mut self, msg: NavTrackMsg) {
        match msg {
            NavTrackMsg::SelectItem(index) => {
                self.selected_index = Some(index);
                self.item_selected_version_counter.bump();
            }
            NavTrackMsg::ButtonMsgWrap(id, msg) => {
                if self.button_mdls.contains_key(&id) {
                    let button_mdl = self.button_mdls.get_mut(&id).unwrap();
                    button_mdl.update(msg);
                } else {
                    let mut button_mdl = ButtonMdl::default();
                    button_mdl.update(msg);
                    let button_mdls = &mut self.button_mdls;
                    button_mdls.insert(id, button_mdl);
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum NavTrackMsg {
    SelectItem(usize),
    ButtonMsgWrap(u64, ButtonMsg),
}

#[derive(Clone, PartialEq, Debug)]
pub struct NavTrackItem {
    pub label: String,
}

impl<'a, MsgT, F, G> From<NavTrack<'a, MsgT, F, G>> for Flood<MsgT> where
    MsgT: Clone,
    F: Fn(NavTrackMsg) -> MsgT + Send + Sync + 'static,
    G: Fn(NavTrackEvt) -> MsgT + Send + Sync + 'static,
{
    fn from(nav_track: NavTrack<'a, MsgT, F, G>) -> Self {
        let palette = nav_track.palette;
        let track_msg_wrap = Arc::new(nav_track.nav_track_msg_wrap);
        let items = nav_track.items;
        let button_ids = nav_track.id.sub_ids(Angle::A, items.len());
        let nav_track_mdl = nav_track.mdl;
        let default_button_mdl = &ButtonMdl::default();
        let buttons = items.iter().enumerate()
            .map(|(i, item)| {
                let id = button_ids[i];
                let msg_wrap = {
                    let msg_wrap = track_msg_wrap.clone();
                    move |button_msg: ButtonMsg| {
                        let nav_track_msg = NavTrackMsg::ButtonMsgWrap(id.clone(), button_msg);
                        let msg: MsgT = (msg_wrap)(nav_track_msg);
                        msg
                    }
                };
                let mdl = nav_track_mdl.button_mdls.get(&id)
                    .unwrap_or(&default_button_mdl);
                let kind = {
                    let label = item.label.to_owned();
                    match nav_track_mdl.selected_index {
                        Some(selected_index) if selected_index == i => ButtonKind::ColoredFlat(label),
                        _ => ButtonKind::PlainFlat(label),
                    }
                };
                let placement = Placement::Start;
                let click_msg = (track_msg_wrap)(NavTrackMsg::SelectItem(i));
                Button { msg_wrap, id, palette, mdl, kind, placement, click_msg }
            })
            .rev()
            .collect::<Vec<_>>();

        let buttons = buttons.into_iter()
            .fold(Flood::Color(palette.transparent), |panel, button| {
                let item = Flood::<MsgT>::from(button);
                panel + (Position::Top(material::Length::ListItemHeight.into()), item)
            });

        let item_selected_msg = (nav_track.item_selected_msg_wrap)(NavTrackEvt::ItemSelected(nav_track_mdl.selected_index.clone()));
        let versioned_item_selected_msg = Version { value: item_selected_msg, counter: nav_track_mdl.item_selected_version_counter.clone() };
        buttons
            + Padding::Vertical(material::Length::ListGroupPadding.into())
            + Flood::Color(palette.light_background_raised)
            + Padding::Behind(material::Length::NavApproach.into())
            + Sensor::Signal(Signal { id: nav_track.id, version: versioned_item_selected_msg })
    }
}
