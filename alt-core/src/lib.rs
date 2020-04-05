pub mod frame_limiter;
pub mod game_objects;
pub mod tilemap;
pub mod timing;
pub mod world;

pub trait Describable {
    fn describe();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
