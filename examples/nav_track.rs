extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::flood::*;
use patchgl::traits::*;
use patchgl::app;
use patchgl::material::Palette;
use patchgl::material::components::nav_track::*;

fn main() {
    app::run(768, 480, "Navigation Track", AppMdl::default());
}

#[derive(Clone, PartialEq, Debug)]
struct AppMdl {
    selection: Option<usize>,
    nav_track_mdl: NavTrackMdl,
}

#[derive(Clone, PartialEq, Debug)]
enum AppMsg {
    ItemSelected(Option<usize>),
    NavTrackMsg(NavTrackMsg),
}

impl Default for AppMdl {
    fn default() -> Self {
        AppMdl {
            selection: None,
            nav_track_mdl: NavTrackMdl::default(),
        }
    }
}

impl Update<AppMsg> for AppMdl {
    fn update(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::ItemSelected(selection) => self.selection = selection,
            AppMsg::NavTrackMsg(msg) => self.nav_track_mdl.update(msg),
        }
    }
}

impl Draw<AppMsg> for AppMdl {
    fn draw(&self) -> Flood<AppMsg> {
        let palette = &Palette::default();
        let items = vec![
            NavTrackItem { label: "A".into() },
            NavTrackItem { label: "B".into() },
        ];

        let detail = {
            let (text, color) = match self.selection {
                Some(index) if index < items.len() => (items[index].label.to_owned(), palette.primary),
                _ => ("Empty".to_owned(), palette.light_background_text_disabled)
            };
            Flood::Text(text, color, Placement::Center)
                + Padding::Dual(Length::Spacing * 2, Length::Full * 0.45)
                + Flood::Color(palette.light_background)
        };

        let nav_track: Flood<AppMsg> = NavTrack {
            nav_track_msg_wrap: AppMsg::NavTrackMsg,
            id: 11,
            palette,
            mdl: &self.nav_track_mdl,
            item_selected_msg_wrap: |nav_track_event| {
                match nav_track_event {
                    NavTrackEvt::ItemSelected(selection) => AppMsg::ItemSelected(selection),
                }
            },
            items,
        }.into();

        detail + (Position::Left(Length::Full * 0.2), nav_track)
    }
}
