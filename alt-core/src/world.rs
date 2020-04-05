//! Game world
//!
//! Use to create as an instance of the game, where all data pertaining to the current state of
//! the world is saved.

use crate::game_objects::GameObject;
use crate::tilemap::Tilemap;

pub struct World {
    tilemap: Tilemap,
    game_objects: Vec<Box<dyn GameObject>>,
}

// TODO: Subject to change
impl World {
    /// Constructor
    pub fn new(width: u32, height: u32) -> Self {
        World {
            tilemap: Tilemap::new(width, height),
            game_objects: vec![],
        }
    }

    /// Should be called by the game server
    pub fn add_player(&mut self, x: usize, y: usize) -> Option<Box<dyn GameObject>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::world::World;

    #[test]
    fn test_world() {
        let mut world = World::new(32, 32);
        world.add_player(0, 0);

        world.add_player(1, 0);

        world.add_player(999, 999);
    }
}
