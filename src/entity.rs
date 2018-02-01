use std::option::Option;

use tcod::Color;

#[derive(Default, Debug)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub ch: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool
}

impl Entity {
    pub fn move_entity(&mut self, dr:(i32, i32)) {
        self.x += dr.0;
        self.y += dr.1;
    }

    pub fn new(x: i32, y: i32, ch: char, color: Color, name: String, blocks: bool) -> Entity {
        let mut ent = Entity::default();
        ent.x = x;
        ent.y = y;
        ent.ch = ch;
        ent.color = color;
        ent.name = name;
        ent.blocks = blocks;

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
