use tcod::map::{Map, FovAlgorithm};

use map_objects::game_map::GameMap;

pub fn initialize_fov(gm: &GameMap) -> Map {
    let mut fov_map = Map::new(gm.width, gm.height);

    for y in 0..gm.height {
        for x in 0..gm.width {
            fov_map.set(x, y,
                        !gm.tiles[gm.index(x, y)].block_sight,
                        !gm.tiles[gm.index(x, y)].blocked);
        }
    }

    fov_map
}

pub fn recompute_fov(fov_map: &mut Map, x: i32, y: i32,
                     max_radius: i32, light_walls: bool,
                     algorithm: FovAlgorithm) {
    fov_map.compute_fov(x, y, max_radius, light_walls, algorithm);
}
