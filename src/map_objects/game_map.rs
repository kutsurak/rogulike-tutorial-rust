use std::cmp::min;
use std::cmp::max;
use std::vec::Vec;

use rand::{thread_rng, Rng};

use tcod::colors;

use entity::Entity;
use map_objects::tile::Tile;

#[derive(Debug, Default)]
pub struct Rect {
    pub x_topleft: i32,
    pub y_topleft: i32,
    pub x_bottomright: i32,
    pub y_bottomright: i32
}

impl Rect {
    pub fn new(x_topleft: i32, y_topleft: i32, width: i32, height: i32) -> Rect {
        let mut rect = Rect::default();

        rect.x_topleft = x_topleft;
        rect.y_topleft = y_topleft;
        rect.x_bottomright = x_topleft + width;
        rect.y_bottomright = y_topleft + height;

        rect
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x_topleft + self.x_bottomright)/2;
        let center_y = (self.y_topleft + self.y_bottomright)/2;

        (center_x, center_y)
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        (self.x_topleft <= other.x_bottomright && self.x_bottomright >= other.x_topleft &&
         self.y_topleft <= other.y_bottomright && self.x_bottomright >= other.y_topleft)
    }
}

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
                let mut t = Tile::new(true);
                t.x = x;
                t.y = y;
                v.push(t);
            }
        }

        v
    }

    pub fn make_map(&mut self, max_rooms: i32, room_min_size: i32, room_max_size: i32,
                    map_width: i32, map_height: i32, player: &mut Entity,
                    entities: &mut Vec<Entity>, max_monsters_per_room: i32) {

        let mut rooms = Vec::new();
        let mut num_rooms = 0;
        let mut rng = thread_rng();

        for _r in 1..max_rooms {
            let w = rng.gen_range(room_min_size, room_max_size);
            let h = rng.gen_range(room_min_size, room_max_size);
            let x = rng.gen_range(0, map_width - w - 1);
            let y = rng.gen_range(0, map_height - h - 1);

            let new_room = Rect::new(x, y, w, h);

            let mut room_is_good = true;
            for other_room in &rooms {
                if new_room.intersect(other_room) {
                    room_is_good = false;
                    break
                }
            }

            if room_is_good {
                self.create_room(&new_room);
                let new_center = new_room.center();

                if num_rooms == 0 {
                    player.x = new_center.0;
                    player.y = new_center.1;
                }
                else {
                    let prev_center = rooms[(num_rooms - 1) as usize].center();
                    if rng.gen_range(0, 2) == 1 {
                        self.create_h_tunnel(prev_center.0, new_center.0,prev_center.1);
                        self.create_v_tunnel(prev_center.1, new_center.1,prev_center.0);
                    }
                    else {
                        self.create_h_tunnel(prev_center.0, new_center.0,prev_center.1);
                        self.create_v_tunnel(prev_center.1, new_center.1,prev_center.0);
                    }
                }

                self.place_entities(&new_room, entities, max_monsters_per_room);

                rooms.push(new_room);
                num_rooms += 1;
            }
        }
    }

    pub fn index(&self, x: i32, y: i32) -> usize {
        (x*self.height + y) as usize
    }

    fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..(max(x1, x2) + 1) {
            let idx = self.index(x, y);
            self.tiles[idx].blocked = false;
            self.tiles[idx].block_sight = false;
        }
    }

    fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..(max(y1, y2) + 1) {
            let idx = self.index(x, y);
            self.tiles[idx].blocked = false;
            self.tiles[idx].block_sight = false;
        }
    }

    fn place_entities(&mut self, room: &Rect, entities: &mut Vec<Entity>, max_monsters_per_room: i32) {
        let num_of_monsters = thread_rng().gen_range(0, max_monsters_per_room);

        for _i in 0..num_of_monsters {
            let x = thread_rng().gen_range(room.x_topleft + 1, room.x_bottomright - 1);
            let y = thread_rng().gen_range(room.y_topleft + 1, room.y_bottomright - 1);

            let mut keep_position = true;
            for entity in entities.iter() {
                //println!("entity list iteration {:?}", entity);
                if entity.x == x && entity.y == y {
                    keep_position = false;
                    break;
                }
            }

            if keep_position {
                let orc_or_troll = thread_rng().gen_range(0, 100);
                let mut monster = Entity::default();
                monster.x = x;
                monster.y = y;
                monster.blocks = true;
                if orc_or_troll < 80 {
                    monster.ch = 'o';
                    monster.color = colors::DESATURATED_GREEN;
                    monster.name = "Orc".to_string();
                }
                else {
                    monster.ch = 'T';
                    monster.color = colors::DARKER_GREEN;
                    monster.name = "Troll".to_string();
                }
                entities.push(monster);
            }
        }

    }

    fn create_room(&mut self, room: &Rect) {
        for x in (room.x_topleft + 1)..(room.x_bottomright) {
            for y in (room.y_topleft + 1)..(room.y_bottomright) {
                let idx = (x*self.height + y) as usize;
                self.tiles[idx].blocked = false;
                self.tiles[idx].block_sight = false
            }
        }
    }

    /*
    fn static_small_wall_filter(&self, x: i32, y: i32) -> bool {
        (x == 30 || x == 31 || x == 32) && y == 22
    }
    */

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        // NOTE: without this check the c library crashes.
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return true;
        }
        let idx: usize = (x*self.height + y) as usize;
        self.tiles[idx].blocked
    }
}