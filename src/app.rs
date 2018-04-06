use ::Color;
use ::flood::Flood;
use ::material;
use ::window::WindowMsg;
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct App<MsgT, MdlT> where
    MsgT: Clone
{
    update_f: Box<Fn(&mut MdlT, MsgT) -> ()>,
    draw_f: Box<Fn(&MdlT, &Palette) -> Flood<MsgT>>,
    msg: PhantomData<MsgT>,
    mdl: PhantomData<MdlT>,
}

impl<MsgT, MdlT> App<MsgT, MdlT> where
    MsgT: Clone + Send + 'static,
    MdlT: Clone + PartialEq
{
    pub fn new<UpdF, DrwF>(update: UpdF, draw: DrwF) -> Self where
        UpdF: Fn(&mut MdlT, MsgT) -> () + 'static,
        DrwF: Fn(&MdlT, &Palette) -> Flood<MsgT> + 'static,
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

    pub fn draw(&self, model: &MdlT, palette: &Palette) -> Flood<MsgT> {
        (self.draw_f)(model, palette)
    }

    pub fn run(self, title: &str, model: MdlT, window: Sender<WindowMsg<MsgT>>) {
        window.send(WindowMsg::Title(title.to_owned())).unwrap();

        let mut running_app = RunningApp::new(self, window, model);
        running_app.run();
    }
}

#[derive(Copy, Clone, Debug)]
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
            primary: material::Color::Pink500.into(),
            secondary: material::Color::PurpleA400.into(),
            light_background: material::Color::LightBackground.into(),
            light_background_raised: material::Color::LightBackgroundCard.into(),
            light_background_text_primary: material::Color::LightBackgroundTextPrimary.into(),
            light_background_divider: material::Color::LightBackgroundDivider.into(),
        }
    }
}

struct RunningApp<MsgT, MdlT> where
    MsgT: Clone
{
    app_msgs: Receiver<MsgT>,
    app_tx: Sender<MsgT>,
    palette: Palette,
    window: Sender<WindowMsg<MsgT>>,
    model: MdlT,
    app: App<MsgT, MdlT>,
}

impl<MsgT, MdlT> RunningApp<MsgT, MdlT> where
    MsgT: Clone + Send + 'static,
    MdlT: Clone + PartialEq,
{
    pub fn new(app: App<MsgT, MdlT>, window: Sender<WindowMsg<MsgT>>, model: MdlT) -> Self
    {
        let (app_sender, app_msgs) = channel::<MsgT>();
        RunningApp { app_msgs, app_tx: app_sender, palette: Palette::default(), window, model, app }
    }

    pub fn run(&mut self) {
        self.connect_window();
        self.flood_window();
        while let Ok(app_msg) = self.app_msgs.recv() {
            let old_mdl = self.model.clone();
            self.app.update(&mut self.model, app_msg);
            if self.model != old_mdl {
                self.flood_window();
            }
        }
    }

    fn connect_window(&self) {
        self.window.send(WindowMsg::Observe(self.app_tx.clone())).unwrap();
    }

    fn flood_window(&self) {
        let flood = self.app.draw(&self.model, &self.palette);
        let flood_msg = WindowMsg::Flood(flood);
        self.window.send(flood_msg).unwrap();
    }
}
