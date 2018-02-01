extern crate tcod;
extern crate rand;

use std::collections::HashMap;

use tcod::{RootConsole, FontLayout};
use tcod::map::FovAlgorithm;
use tcod::colors;

mod engine;
mod entity;
mod map_objects;

use engine::fov_functions::{initialize_fov, recompute_fov};
use engine::input::{Action, handle_input};
use engine::render_functions::{render_all, clear_all};
use entity::{Entity};
use map_objects::game_map::GameMap;

fn main() {
    let screen_width = 80;
    let screen_height = 50;
    let map_width = 80;
    let map_height = 45;
    let room_max_size = 10;
    let room_min_size = 6;
    let max_rooms = 30;

    let fov_algorithm = FovAlgorithm::Basic;
    let fov_light_walls = true;
    let fov_radius = 10;

    let mut colors = HashMap::new();

    colors.insert(String::from("dark_wall"), colors::Color::new(0, 0, 100));
    colors.insert(String::from("dark_ground"), colors::Color::new(50, 50, 100));
    colors.insert(String::from("light_wall"), colors::Color::new(130, 110, 50));
    colors.insert(String::from("light_ground"), colors::Color::new(200, 180, 50));

    let mut player = Entity {
        x: screen_width/2,
        y: screen_height/2,
        ch: '@',
        color: colors::WHITE
    };

    let npc = Entity {
        x: screen_width/2 - 5,
        y: screen_height/2,
        ch: '@',
        color: colors::YELLOW
    };

    let mut game_map = GameMap::new(map_width, map_height);
    game_map.make_map(max_rooms, room_min_size, room_max_size, map_width, map_height, &mut player);

    let mut fov_recompute = true;
    let mut fov_map = initialize_fov(&game_map);

    let mut entities = vec![npc];

    let mut root = RootConsole::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .size(screen_width, screen_height)
        .title("libtcod tutorial revisited")
        .fullscreen(false)
        .init();

    while !root.window_closed() {
        // Render the results
        if fov_recompute {
            recompute_fov(&mut fov_map, player.x, player.y,
                          fov_radius, fov_light_walls,
                          fov_algorithm);
        }
        entities.push(player);
        render_all(&mut root, &entities,
                   &mut game_map, &fov_map, fov_recompute,
                   screen_width, screen_height, &colors);
        player = entities.pop().unwrap();
        root.flush();
        clear_all(&mut root, &entities);

        // Handle user input
        let keypress = root.wait_for_keypress(true);
        let action = handle_input(keypress);

        // Update the game state
        let mut displacement = (0, 0);
        match action {
            Action::Exit => break,
            Action::Move(dr) => displacement = dr,
            Action::Fullscreen => {
                let fullscreen = root.is_fullscreen();
                root.set_fullscreen(!fullscreen);
            },
            _ => {}
        }
        if !game_map.is_blocked(player.x + displacement.0, player.y + displacement.1) {
            player.move_entity(displacement);
            fov_recompute = true;
        }
    }
}
