//! Game world
//!
//! Use to create as an instance of the game, where all data pertaining to the current state of
//! the world is saved.

use crate::game_objects::GameObject;

pub struct World {
    width: usize,
    height: usize,
    // TODO: use tilemap struct, for right now, assume tilemap is a fixed 2d array.
    tilemap: [[i32; 32]; 32],
    game_objects: Vec<Box<dyn GameObject>>,
}

// TODO: Subject to change
impl World {
    /// Constructor
    pub fn new(width: usize, height: usize) -> Self {
        World {
            width,
            height,
            tilemap: [[0; 32]; 32],
            game_objects: vec![],
        }
    }

    pub fn add_player(x: i32, y: i32) -> Option<Box<dyn GameObject>> {
        None
    }
}
