use std::vec::Vec;

use tcod::Console;
use tcod::console::{blit, Offscreen};
use entity::Entity;
use tcod::console::BackgroundFlag;


pub fn render_all(mut con: &Console, entities: &Vec<Entity>,
                  screen_width: i32, screen_height: i32) {
    let mut off_screen = Offscreen::new(screen_width, screen_height);
    for entity in entities {
        draw_entity(&mut off_screen, &entity);
    }

    blit(&off_screen, (0, 0), (screen_width, screen_height), &mut con, (0, 0), 1.0, 1.0);
}


fn draw_entity(mut con: &Console, entity: &Entity) {
    con.set_default_foreground(entity.color);
    con.put_char(entity.x, entity.y, entity.ch, BackgroundFlag::None);
}


pub fn clear_all(mut con: &Console, entities: &Vec<Entity>) {
    for entity in entities {
        clear_entity(&mut con, entity);
    }
}

fn clear_entity(mut con: &Console, entity: &Entity) {
    con.put_char(entity.x, entity.y, ' ', BackgroundFlag::None);
}
