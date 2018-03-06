use entity::Entity;
use tcod::map::Map;
use map_objects::game_map::GameMap;

pub trait BasicMonster {
    fn take_turn(&mut self, target: &Entity, fov_map: &Map, game_map: &GameMap, entities: &Vec<Entity>);
}

/*
impl BasicMonster {
    pub fn take_turn(&self) {
        println!("The {} wonders when it will get to move.", self.owner);
    }
}
*/
