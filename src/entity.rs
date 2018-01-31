use tcod::Color;

#[derive(Default)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub ch: char,
    pub color: Color
}

impl Entity {

    // Immutable (FP) entity


    pub fn move_entity(&self, dr: (i32, i32)) -> Entity {
        let mut ret = Entity::default();
        ret.x = self.x + dr.0;
        ret.y = self.y + dr.1;

        ret
    }

    // Mutable (OOP) entity
    /*
    pub fn move_entity(&mut self, dr:(i32, i32)) {
        self.x += dr.0;
        self.y += dr.0;
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.x
    }
    pub fn get_char(&self) -> char {
        self.ch
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    */
}
