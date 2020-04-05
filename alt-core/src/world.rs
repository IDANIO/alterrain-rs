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

    pub fn add_player(&mut self, x: usize, y: usize) -> Option<Box<dyn GameObject>> {
        self.tilemap[x][y];
        None
    }
}

#[cfg(test)]
mod test {
    use crate::world::World;

    #[test]
    fn test_world() {
        let mut world = World::new(32, 32);
        world.add_player(0, 0);

        world.add_player(1, 0);

        world.add_player(999, 999);
    }
}
