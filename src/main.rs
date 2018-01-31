extern crate tcod;

use tcod::{RootConsole, FontLayout};
use tcod::colors;

mod input;
mod entity;
mod render_functions;
mod map_objects;

use input::{Action, handle_input};
use entity::{Entity};
use render_functions::{render_all, clear_all};
use map_objects::game_map::GameMap;

fn main() {
    let screen_width = 80;
    let screen_height = 50;
    let map_width = 80;
    let map_height = 45;

    let colors = vec![colors::CYAN, colors::MAGENTA];
    let game_map = GameMap::new(map_width, map_height);

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

    let mut entities = vec![npc];

    let mut root = RootConsole::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .size(screen_width, screen_height)
        .title("libtcod tutorial revisited")
        .fullscreen(false)
        .init();

    while !root.window_closed() {
        // Render the results
        entities.push(player);
        render_all(&mut root, &entities, &game_map, screen_width, screen_height, &colors);
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
        }
    }
}
