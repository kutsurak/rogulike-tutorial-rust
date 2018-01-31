use tcod::Color;

#[derive(Default)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub ch: char,
    pub color: Color
}

impl Entity {
    pub fn move_entity(&mut self, dr:(i32, i32)) {
        self.x += dr.0;
        self.y += dr.1;
    }
}
