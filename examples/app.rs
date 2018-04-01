use patchgl::Color;
use patchgl::flood::Flood;
use patchgl::WindowMsg;
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct App<MsgT, MdlT> {
    update_f: Box<Fn(&mut MdlT, MsgT) -> ()>,
    draw_f: Box<Fn(&MdlT, &Palette, &Sender<MsgT>) -> Flood>,
    msg: PhantomData<MsgT>,
    mdl: PhantomData<MdlT>,
}

impl<MsgT, MdlT> App<MsgT, MdlT> where
    MsgT: Send + 'static
{
    pub fn new<UpdF, DrwF>(update: UpdF, draw: DrwF) -> Self where
        UpdF: Fn(&mut MdlT, MsgT) -> () + 'static,
        DrwF: Fn(&MdlT, &Palette, &Sender<MsgT>) -> Flood + 'static,
    {
        App {
            update_f: Box::new(update),
            draw_f: Box::new(draw),
            msg: PhantomData,
            mdl: PhantomData,
        }
    }

    pub fn update(&self, model: &mut MdlT, msg: MsgT) {
        (self.update_f)(model, msg);
    }

    pub fn draw(&self, model: &MdlT, palette: &Palette, app: &Sender<MsgT>) -> Flood {
        (self.draw_f)(model, palette, app)
    }

    pub fn run(self, model: MdlT, window: Sender<WindowMsg>) {
        let mut running_app = RunningApp::new(self, window, model);
        running_app.run();
    }
}

pub struct Palette {
    pub primary: Color,
    pub secondary: Color,
    pub light_background: Color,
    pub light_background_raised: Color,
    pub light_background_text_primary: Color,
    pub light_background_divider: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Palette {
            primary: MaterialColor::Pink500.into(),
            secondary: MaterialColor::PurpleA400.into(),
            light_background: MaterialColor::LightBackground.into(),
            light_background_raised: MaterialColor::LightBackgroundCard.into(),
            light_background_text_primary: MaterialColor::LightBackgroundTextPrimary.into(),
            light_background_divider: MaterialColor::LightBackgroundDivider.into(),
        }
    }
}

#[allow(dead_code)]
pub enum MaterialColor {
    LightBackground,
    LightBackgroundCard,
    LightBackgroundTextPrimary,
    LightBackgroundTextSecondary,
    LightBackgroundTextDisabled,
    LightBackgroundDivider,
    PurpleA100,
    PurpleA200,
    PurpleA400,
    PurpleA700,
    Pink500,
}

impl Into<Color> for MaterialColor {
    fn into(self) -> Color {
        match self {
            MaterialColor::LightBackground => Color::from_hexrgb(0xfa, 0xfa, 0xfa),
            MaterialColor::LightBackgroundCard => Color::white(),
            MaterialColor::LightBackgroundTextPrimary => Color::custom_black(0.87),
            MaterialColor::LightBackgroundTextSecondary => Color::custom_black(0.54),
            MaterialColor::LightBackgroundTextDisabled => Color::custom_black(0.38),
            MaterialColor::LightBackgroundDivider => Color::custom_black(0.12),
            MaterialColor::PurpleA100 => Color::from_hexrgb(0xea, 0x80, 0xfc),
            MaterialColor::PurpleA200 => Color::from_hexrgb(0xe0, 0x40, 0xfb),
            MaterialColor::PurpleA400 => Color::from_hexrgb(0xd5, 0x00, 0xf9),
            MaterialColor::PurpleA700 => Color::from_hexrgb(0xaa, 0x00, 0xff),
            MaterialColor::Pink500 => Color::from_hexrgb(0xe9, 0x1e, 0x64),
        }
    }
}

struct RunningApp<MsgT, MdlT>
{
    app_msgs: Receiver<MsgT>,
    app_sender: Sender<MsgT>,
    palette: Palette,
    window: Sender<WindowMsg>,
    model: MdlT,
    app: App<MsgT, MdlT>,
}

impl<MsgT, MdlT> RunningApp<MsgT, MdlT> where
    MsgT: Send + 'static,
{
    pub fn new(app: App<MsgT, MdlT>, window: Sender<WindowMsg>, model: MdlT) -> Self
    {
        let (app_sender, app_msgs) = channel::<MsgT>();
        RunningApp { app_msgs, app_sender, palette: Palette::default(), window, model, app }
    }

    pub fn run(&mut self) {
        self.flood_window();
        while let Ok(app_msg) = self.app_msgs.recv() {
            self.step(app_msg);
        }
    }

    fn step(&mut self, msg: MsgT) {
        self.app.update(&mut self.model, msg);
        self.flood_window();
    }

    fn flood_window(&self) {
        let flood = self.app.draw(&self.model, &self.palette, &self.app_sender);
        let flood_msg = WindowMsg::Flood(flood);
        self.window.send(flood_msg).unwrap();
    }
}