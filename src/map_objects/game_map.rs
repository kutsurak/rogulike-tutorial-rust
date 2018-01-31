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
                if self.static_small_wall_filter(x, y) {
                    t.blocked = true;
                    t.block_sight = true;
                }
                v.push(t);
            }
        }

        v
    }

    fn static_small_wall_filter(&self, x: i32, y: i32) -> bool {
        (x == 30 || x == 31 || x == 32) && y == 22
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        // NOTE: without this check the c library crashes.
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return true;
        }
        let idx: usize = (x*self.height + y) as usize;
        println!("{}, {} => {} ?? {}", x, y, idx, self.tiles.len());
        self.tiles[idx].blocked
    }
}