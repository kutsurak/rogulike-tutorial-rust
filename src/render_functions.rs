use std::vec::Vec;

use tcod::Console;
use tcod::console::{blit, Offscreen};
use tcod::console::BackgroundFlag;
use tcod::colors::Color;

use entity::Entity;
use map_objects::game_map::GameMap;

pub fn render_all(mut con: &Console, entities: &Vec<Entity>,
                  game_map: &GameMap,
                  screen_width: i32, screen_height: i32, colors: &Vec<Color>) {
    let mut off_screen = Offscreen::new(screen_width, screen_height);

    // draw the map
    for tile in &game_map.tiles {
        let x = tile.x;
        let y = tile.y;
        let wall = tile.blocked;

        if wall {
            off_screen.set_char_background(x, y, colors[0], BackgroundFlag::Set);
        }
        else {
            off_screen.set_char_background(x, y, colors[1], BackgroundFlag::Set);
        }
        //}
    }

    // draw the entities
    for entity in entities {
        draw_entity(&mut off_screen, &entity);
    }

    blit(&off_screen,
         (0, 0),
         (screen_width, screen_height),
         &mut con,
         (0, 0),
         1.0,
         1.0);
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
