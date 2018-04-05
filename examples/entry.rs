extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

fn main() {
    use patchgl::window;
    window::start(320, 400, |window| {
        use patchgl::window::WindowMsg;
        use patchgl::flood::*;
        use patchgl::material;
        use patchgl::material::entry::*;

        let entry = Entry {
            id: 26,
            label: "Label".into(),
            placeholder: Some("Placeholder".into()),
        };
        let flood = draw_focused_entry(&entry)
            + Padding::Dual(Length::Spacing, Length::Full * 0.35)
            + (Stratum::JustBelow, Flood::Color(material::Color::LightBackground.into()));

        window.send(WindowMsg::Flood::<()>(flood)).unwrap();
    });
}
