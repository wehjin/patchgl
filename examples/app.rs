use patchgl::{Color, X11Color};
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
    pub text: Color,
    pub background: Color,
    pub button_idle_background: Color,
    pub button_activated_background: Color,
    pub button_border: Color,
}

impl Palette {
    pub fn new() -> Self {
        Palette {
            text: Color::from(X11Color::Indigo),
            background: Color::from(X11Color::Lavender),
            button_idle_background: Color::from(X11Color::Lavender),
            button_activated_background: Color::from(X11Color::Thistle),
            button_border: Color::from(X11Color::MediumPurple),
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
        RunningApp { app_msgs, app_sender, palette: Palette::new(), window, model, app }
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
