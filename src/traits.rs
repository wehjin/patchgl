use flood::Flood;

pub trait Update<MsgT> {
    fn update(&mut self, msg: MsgT);
}

pub trait Draw<MsgT> where MsgT: Clone
{
    fn draw(&self) -> Flood<MsgT>;
}
