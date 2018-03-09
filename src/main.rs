extern crate tcod;
extern crate rand;

use std::collections::HashMap;

use tcod::{RootConsole, FontLayout};
use tcod::map::FovAlgorithm;
use tcod::colors;

mod engine;
mod entity;
mod map_objects;
mod components;

use engine::fov_functions::{initialize_fov, recompute_fov};
use engine::game_states::GameStates;
use engine::input::{Action, handle_input};
use engine::render_functions::{render_all, clear_all};
use entity::{Entity, get_blocking_entities_at_location};
use map_objects::game_map::GameMap;
use components::ai::BasicMonster;
use components::fighter::Fighter;

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

    let max_monsters_per_room = 3;

    let mut colors = HashMap::new();

    colors.insert(String::from("dark_wall"), colors::Color::new(0, 0, 100));
    colors.insert(String::from("dark_ground"), colors::Color::new(50, 50, 100));
    colors.insert(String::from("light_wall"), colors::Color::new(130, 110, 50));
    colors.insert(String::from("light_ground"), colors::Color::new(200, 180, 50));

    let fighter_component = Fighter::new(30, 2, 5);
    let mut player = Entity::new(0, 0, '@', colors::WHITE,
                                 "Player".to_string(), true,
                                 fighter_component);

    let mut entities = Vec::new();

    let mut game_map = GameMap::new(map_width, map_height);
    game_map.make_map(max_rooms, room_min_size, room_max_size,
                      map_width, map_height, &mut player,
                      &mut entities, max_monsters_per_room);

    let mut fov_recompute = true;
    let mut fov_map = initialize_fov(&game_map);

    let mut root = RootConsole::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .size(screen_width, screen_height)
        .title("libtcod tutorial revisited")
        .fullscreen(false)
        .init();

    let mut game_state = GameStates::PlayersTurn;
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
        let mut dest = (0, 0);
        let mut displacement = (0, 0);
        match action {
            Action::Exit => break,
            Action::Move(dr) => {
                displacement = dr;
                dest = (player.x + dr.0, player.y + dr.1);
            },
            Action::Fullscreen => {
                let fullscreen = root.is_fullscreen();
                root.set_fullscreen(!fullscreen);
            },
            _ => {}
        }

        // let mut new_entities: Vec<Entity> = Vec::new();
        if game_state == GameStates::EnemyTurn {
            let entity_number = entities.len();
            let mut cnt = 0;
            let mut tmp_entities = Vec::new();
            while cnt < entity_number {
                let mut entity = match entities.pop() {
                    Some(en) => en,
                    None => continue
                };
                entity.take_turn(&player, &fov_map, &game_map, &entities);
                tmp_entities.push(entity);
                cnt += 1;
            }
            entities.append(&mut tmp_entities);
            game_state = GameStates::PlayersTurn;
        }
        // let entities = new_entities;
        if !game_map.is_blocked(dest.0, dest.1) && game_state == GameStates::PlayersTurn {
            let target = get_blocking_entities_at_location(&entities, dest.0, dest.1);
            match target {
                Some(ref m) => println!("You kick the {} in the shins, much to its annoyance!", m.name),
                None => {
                    player.move_entity(displacement);
                    fov_recompute = true;
                }
            }

            game_state = GameStates::EnemyTurn;
        }
    }
}
