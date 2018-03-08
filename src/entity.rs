use std::f32;
use std::option::Option;

use tcod::Color;
use tcod::map::Map;

use map_objects::game_map::GameMap;

use components::ai::BasicMonster;
use components::fighter::Fighter;

#[derive(Default, Debug, Clone)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub ch: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    // Fighter attributes
    pub fighter: Fighter
}

impl Entity {
    pub fn move_entity(&mut self, dr:(i32, i32)) {
        self.x += dr.0;
        self.y += dr.1;
    }

    pub fn move_towards(&mut self, target: (i32, i32),
                        game_map: &GameMap, entities: &Vec<Entity>) {
        let dx = target.0 - self.x;
        let dy = target.1 - self.y;

        let distance = ((dx*dx + dy*dy) as f32).sqrt();
        let dx = (dx as f32/distance).round() as i32;
        let dy = (dy as f32/distance).round() as i32;

        let is_blocked = match get_blocking_entities_at_location(entities, self.x + dx, self.y + dy) {
            Some(_) => true,
            None => false
        };

        if !(game_map.is_blocked(self.x + dx, self.y + dy) || is_blocked) {
            self.move_entity((dx, dy));
        }
    }

    pub fn distance_to(&self, other: &Entity) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        ((dx*dx + dy*dy) as f32).sqrt()
    }

    pub fn new(x: i32, y: i32, ch: char,
               color: Color, name: String,
               blocks: bool, fighter: Fighter) -> Entity {
        let mut ent = Entity::default();
        ent.x = x;
        ent.y = y;
        ent.ch = ch;
        ent.color = color;
        ent.name = name;
        ent.blocks = blocks;
        ent.fighter = fighter;

        ent
    }

}

pub fn get_blocking_entities_at_location(entities: &Vec<Entity>, dest_x: i32, dest_y: i32) -> Option<&Entity> {
    for entity in entities.iter() {
        if entity.blocks && entity.x == dest_x && entity.y == dest_y {
            return Some(entity)
        }
    }
    None
}

impl BasicMonster for Entity {
    fn take_turn(&mut self, target: &Entity, fov_map: &Map, game_map: &GameMap, entities: &Vec<Entity>) {
        if fov_map.is_in_fov(self.x, self.y) {
            if self.distance_to(target) >= 2.0 {
                self.move_towards((target.x, target.y), game_map, &entities);
            }
            else if target.fighter.hp > 0 {
                println!("The {} insults you! Your ego is damaged!", self.name)
            }
        }
    }
}
