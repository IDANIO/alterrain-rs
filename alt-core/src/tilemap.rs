#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Water = 0,
    Grass = 1,
}

pub struct Tilemap {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

impl Tilemap {
    pub fn new(width: u32, height: u32) -> Self {
        // Well, these are temp stuff, meant to be removed.
        let h = (width / 2) as f32;
        let k = (height / 2) as f32;
        let a = h / 2f32;
        let b = k / 2f32;
        let tiles = (0..width * height)
            .map(|i| {
                let x = (i % width) as f32;
                let y = (i / width) as f32;

                if (x - h).powf(2f32) / a.powf(2f32) + (y - k).powf(2f32) / b.powf(2f32) <= 1f32 {
                    Tile::Grass
                } else {
                    Tile::Water
                }
            })
            .collect();

        Tilemap {
            width,
            height,
            tiles,
        }
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn get_tile(&self, row: u32, col: u32) -> Option<&Tile> {
        let index = self.get_index(row, col);
        self.tiles.get(index)
    }

    pub fn set_tile(&mut self, row: u32, col: u32, new_tile: Tile) -> bool {
        let index = self.get_index(row, col);
        if let Some(tile) = self.tiles.get_mut(index) {
            *tile = new_tile;
            true
        } else {
            false
        }
    }

    pub fn get_tile_unchecked(&self, row: u32, col: u32) -> &Tile {
        unsafe { self.tiles.get_unchecked(self.get_index(row, col)) }
    }
}

impl std::fmt::Display for Tilemap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.tiles.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Tile::Water { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tilemap::Tilemap;

    #[test]
    fn test_creation() {
        let tm = Tilemap::new(32, 32);

        assert_eq!(
            tm.to_string(),
            "◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n\
            ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻\n"
        );
    }
}
