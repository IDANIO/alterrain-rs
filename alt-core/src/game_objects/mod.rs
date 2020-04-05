pub trait GameObject {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
}

pub struct Player;

impl GameObject for Player {
    fn x(&self) -> usize {
        unimplemented!()
    }

    fn y(&self) -> usize {
        unimplemented!()
    }
}
