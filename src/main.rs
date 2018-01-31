extern crate tcod;

use tcod::{Console, RootConsole, FontLayout};
use tcod::colors;

mod input;

use input::{Action, handle_input};

fn main() {
    let screen_width = 80;
    let screen_height = 50;

    let mut player_x = screen_width/2;
    let mut player_y = screen_height/2;

    let mut root = RootConsole::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .size(screen_width, screen_height)
        .title("libtcod tutorial revisited")
        .fullscreen(false)
        .init();

    while !root.window_closed() {
        // Handle the results
        root.set_default_foreground(colors::WHITE);
        root.put_char(player_x, player_y, '@', tcod::console::BackgroundFlag::None);
        root.flush();
        root.put_char(player_x, player_y, ' ', tcod::console::BackgroundFlag::None);
        // Handle user input
        let keypress = root.wait_for_keypress(true);
        let action = handle_input(keypress);

        // Update the game state
        match action {
            Action::Exit => break,
            Action::Move((dx, dy)) => {
                player_x += dx;
                player_y += dy;
            },
            Action::Fullscreen => {
                let fullscreen = root.is_fullscreen();
                root.set_fullscreen(!fullscreen);
            },
            _ => {}
        }
    }
}
