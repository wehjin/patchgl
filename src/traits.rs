use flood::Flood;

pub trait Mdl<MsgT>: Update<MsgT> + Draw<MsgT>
    where MsgT: Clone
{}

pub trait Update<MsgT> {
    fn update(&mut self, msg: MsgT);
}

pub trait Draw<MsgT> where MsgT: Clone
{
    fn draw(&self) -> Flood<MsgT>;
}
