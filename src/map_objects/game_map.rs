use std::vec::Vec;
use map_objects::tile::Tile;

#[derive(Debug, Default)]
pub struct GameMap {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>
}

impl GameMap {
    pub fn new(width: i32, height: i32) -> GameMap {
        let mut map = GameMap::default();
        map.width = width;
        map.height = height;

        map.tiles = map.init_tiles();

        map
    }

    fn init_tiles(&self) -> Vec<Tile> {
        let mut v = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let mut t = Tile::new(false);

                t.x = x;
                t.y = y;
                if (x == 30 || x == 31 || x == 32) && y == 22 {
                    t.blocked = true;
                    t.block_sight = true;
                }
                v.push(t);
            }
        }

        v
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        let idx: usize = (x*self.height + y) as usize;
        // NOTE: without this check it panics. Why?
        if idx > self.tiles.len() {
            return true
        }
        self.tiles[idx].blocked
    }
}