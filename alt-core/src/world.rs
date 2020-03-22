//! Game world
//!
//! Use to create as an instance of the game, where all data pertaining to the current state of
//! the world is saved.

use crate::game_objects::GameObject;
use crate::tilemap::Tilemap;

pub struct World {
    width: usize,
    height: usize,
    // tilemap: Tilemap,
    game_objects: Vec<Box<dyn GameObject>>,
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        World {
            width,
            height,
            // tilemap: Tilemap {},
            game_objects: vec![],
        }
    }
}
