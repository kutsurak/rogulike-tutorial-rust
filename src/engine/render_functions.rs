use std::vec::Vec;
use std::collections::HashMap;

use tcod::colors::Color;
use tcod::Console;
use tcod::console::{blit, Offscreen};
use tcod::console::BackgroundFlag;
use tcod::map::Map;

use entity::Entity;
use map_objects::game_map::GameMap;

pub fn render_all(mut con: &Console, entities: &Vec<Entity>,
                  game_map: &mut GameMap, fov_map: &Map, fov_recompute: bool,
                  screen_width: i32, screen_height: i32,
                  colors: &HashMap<String, Color>) {
    let mut off_screen = Offscreen::new(screen_width, screen_height);

    // draw the map
    if  fov_recompute {
        for tile in &mut game_map.tiles {
            let x = tile.x;
            let y = tile.y;
            let wall = tile.blocked;

            if fov_map.is_in_fov(x, y) {
                if wall {
                    let draw_color = colors.get(&String::from("light_wall")).unwrap();
                    off_screen.set_char_background(x, y, *draw_color, BackgroundFlag::Set);
                }
                else {
                    let draw_color = colors.get(&String::from("light_ground")).unwrap();
                    off_screen.set_char_background(x, y, *draw_color, BackgroundFlag::Set);
                }
                tile.explored = true;
            }
            else {
                if tile.explored {
                    if wall {
                        let draw_color = colors.get(&String::from("dark_wall")).unwrap();
                        off_screen.set_char_background(x, y, *draw_color, BackgroundFlag::Set);
                    }
                    else {
                        let draw_color = colors.get(&String::from("dark_ground")).unwrap();
                        off_screen.set_char_background(x, y, *draw_color, BackgroundFlag::Set);
                    }
                }
            }
        }
    }

    // draw the entities
    for entity in entities {
        draw_entity(&mut off_screen, &entity, fov_map);
    }

    blit(&off_screen,
         (0, 0),
         (screen_width, screen_height),
         &mut con,
         (0, 0),
         1.0,
         1.0);
}


fn draw_entity(mut con: &Console, entity: &Entity,
               fov_map: &Map) {
    if fov_map.is_in_fov(entity.x, entity.y) {
        con.set_default_foreground(entity.color);
        con.put_char(entity.x, entity.y, entity.ch, BackgroundFlag::None);
    }
}


pub fn clear_all(mut con: &Console, entities: &Vec<Entity>) {
    for entity in entities {
        clear_entity(&mut con, entity);
    }
}

fn clear_entity(mut con: &Console, entity: &Entity) {
    con.put_char(entity.x, entity.y, ' ', BackgroundFlag::None);
}
