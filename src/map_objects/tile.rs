/// A tile on a map. It may or may not be blocked, and may or may not block sight.
#[derive(Default, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub x: i32,
    pub y: i32
}

impl Tile {
    pub fn new(blocked: bool) -> Tile {
        let mut ret = Tile::default();
        ret.blocked = blocked;
        ret.block_sight = blocked;

        ret
    }


    pub fn new_with_sight(blocked: bool, block_sight: bool) -> Tile {
        let mut ret = Tile::default();
        ret.blocked = blocked;
        ret.block_sight = block_sight;

        ret
    }
}