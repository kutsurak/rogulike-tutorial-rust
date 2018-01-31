use tcod::input::Key;
use tcod::input::KeyCode::{Enter, Escape, Up, Down, Right, Left};

pub enum Action {
    Empty,
    Exit,
    Move((i32, i32)),
    Fullscreen
}


pub fn handle_input(keypress: Key) -> Action {
    match keypress {
        Key { code: Escape, .. } => Action::Exit,
        Key { code: Up, .. }     => Action::Move((0, -1)),
        Key { code: Down, .. }   => Action::Move((0, 1)),
        Key { code: Left, .. }   => Action::Move((-1, 0)),
        Key { code: Right, .. }  => Action::Move((1, 0)),
        Key { code: Enter, left_alt: true, .. }  => Action::Fullscreen,
        _ => Action::Empty,
    }
}
