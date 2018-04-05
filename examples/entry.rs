extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::app::Palette;
use patchgl::material::entry::Entry;
use patchgl::material::entry;
use patchgl::material;
use patchgl::flood::*;

fn main() {
    use patchgl::window;

    window::start(320, 400, |window| {
        use patchgl::app::App;

        let app = App::new(update, draw);
        app.run(Mdl::default(), window);
    });
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Msg {
    EntryMsg(entry::Msg)
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Mdl {
    pub entry_mdl: entry::Mdl,
}

impl Default for Mdl {
    fn default() -> Self {
        Mdl { entry_mdl: entry::Mdl::default() }
    }
}

fn draw(mdl: &Mdl, _palette: &Palette) -> Flood<Msg> {
    let entry = Entry {
        msg_wrap: Msg::EntryMsg,
        id: 26,
        mdl: &mdl.entry_mdl,
        label: "Label".into(),
        placeholder: Some("Placeholder".into()),
    };
    let entry_flood = entry::flood(&entry);
    entry_flood
        + Padding::Dual(Length::Spacing, Length::Full * 0.35)
        + (Stratum::JustBelow, Flood::Color(material::Color::LightBackground.into()))
}

fn update(mdl: &mut Mdl, msg: Msg) {
    let Msg::EntryMsg(entry_msg) = msg;
    entry::update(&mut mdl.entry_mdl, entry_msg);
}
