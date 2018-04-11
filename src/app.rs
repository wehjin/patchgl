use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::fmt;
use traits::{Update, Draw};
use flood::Flood;
use window::WindowMsg;


pub fn run<MdlT, MsgT>(width: u32, height: u32, title: &str, model: MdlT)
    where
        MdlT: Update<MsgT> + Draw<MsgT> + Send + Sync + 'static + Clone + PartialEq + fmt::Debug,
        MsgT: Send + Sync + 'static + Clone + PartialEq + fmt::Debug,
{
    use window;
    let title = title.to_owned();
    window::start(width, height, move |window| {
        let app = App::new(MdlT::update, MdlT::draw);
        app.run(&title, model.clone(), window);
    });
}

pub struct App<MsgT, MdlT> where
    MsgT: Clone
{
    update_f: Box<Fn(&mut MdlT, MsgT) -> ()>,
    draw_f: Box<Fn(&MdlT) -> Flood<MsgT>>,
    msg: PhantomData<MsgT>,
    mdl: PhantomData<MdlT>,
}

impl<MsgT, MdlT> App<MsgT, MdlT> where
    MsgT: Clone + Send + 'static,
    MdlT: Clone + PartialEq
{
    pub fn new<UpdF, DrwF>(update: UpdF, draw: DrwF) -> Self where
        UpdF: Fn(&mut MdlT, MsgT) -> () + 'static,
        DrwF: Fn(&MdlT) -> Flood<MsgT> + 'static,
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

    pub fn draw(&self, model: &MdlT) -> Flood<MsgT> {
        (self.draw_f)(model)
    }

    pub fn run(self, title: &str, model: MdlT, window: Sender<WindowMsg<MsgT>>) {
        window.send(WindowMsg::Title(title.to_owned())).unwrap();

        let mut running_app = RunningApp::new(self, window, model);
        running_app.run();
    }
}

struct RunningApp<MsgT, MdlT> where
    MsgT: Clone
{
    app_msgs: Receiver<MsgT>,
    app_tx: Sender<MsgT>,
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
        RunningApp { app_msgs, app_tx: app_sender, window, model, app }
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
        let flood = self.app.draw(&self.model);
        let flood_msg = WindowMsg::Flood(flood);
        self.window.send(flood_msg).unwrap();
    }
}
