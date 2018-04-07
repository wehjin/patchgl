extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

fn main() {
    use patchgl::window;
    window::start(768, 768, |window| {
        use patchgl::app::App;

        let app = App::new(update, draw);
        app.run("Stepper", Mdl::default(), window);
    });
}


#[derive(Clone, PartialEq, Debug, Default)]
struct Mdl {
    active_index: i32,
    button_bar_mdl: ButtonBarMdl,
}

use patchgl::material::components::button_bar::*;

fn update(mdl: &mut Mdl, msg: Msg) {
    match msg {
        Msg::ButtonBarMsg(button_bar_msg) => {
            update_button_bar(&mut mdl.button_bar_mdl, button_bar_msg);
        }
        Msg::Continue => {
            mdl.active_index = (mdl.active_index + 1).min(3);
        }
        Msg::Back => {
            mdl.active_index = (mdl.active_index - 1).max(0);
        }
        Msg::Done => {}
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Msg {
    Continue,
    Back,
    Done,
    ButtonBarMsg(ButtonBarMsg),
}


fn draw(mdl: &Mdl) -> Flood<Msg> {
    use patchgl::material::Palette;
    use patchgl::material::components::stepper::*;

    let palette = Palette::default();
    let buttons = if mdl.active_index == 0 {
        vec![
            Button {
                id: 99,
                label: "Start".into(),
                intent: ButtonIntent::Call,
                click_msg: Msg::Continue,
            },
        ]
    } else if mdl.active_index == 3 {
        vec![
            Button {
                id: 99,
                label: "Done".into(),
                intent: ButtonIntent::Call,
                click_msg: Msg::Done,
            },
            Button {
                id: 100,
                label: "Back".into(),
                intent: ButtonIntent::Provide,
                click_msg: Msg::Back,
            },
        ]
    } else {
        vec![
            Button {
                id: 99,
                label: "Continue".into(),
                intent: ButtonIntent::Call,
                click_msg: Msg::Continue,
            },
            Button {
                id: 100,
                label: "Back".into(),
                intent: ButtonIntent::Provide,
                click_msg: Msg::Back,
            },
        ]
    };
    let button_bar: Flood<Msg> = ButtonBar {
        msg_wrap: Msg::ButtonBarMsg,
        palette: &palette,
        button_bar_mdl: &mdl.button_bar_mdl,
        buttons,
    }.into();
    let content = Flood::Color(palette.primary) + (Position::Bottom(Length::Spacing * 3), button_bar);
    let stepper: Flood<Msg> = Stepper {
        palette: &palette,
        id: vec![1],
        active_index: mdl.active_index as usize,
        active_content: content,
        steps: vec![
            Step { label: "Fee" },
            Step { label: "Fi" },
            Step { label: "Fo" },
            Step { label: "Fum" },
        ],
    }.into();
    stepper
        + Padding::Uniform(Length::Spacing * 1.5)
        + (Stratum::JustBelow, Flood::Color(palette.light_background))
}

use patchgl::flood::*;

