//! Game world
//!
//! Use to create as an instance of the game, where all data pertaining to the current state of
//! the world is saved.

use crate::tilemap::Tilemap;

pub struct World {
    tilemap: Tilemap,
}

// TODO: Subject to change
impl World {
    /// Constructor
    pub fn new(width: u32, height: u32) -> Self {
        World {
            tilemap: Tilemap::new(width, height),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::world::World;

    #[test]
    fn test_world() {
        let mut world = World::new(32, 32);
    }
}
