use tcod::input::Key;
use tcod::input::KeyCode::*;

pub enum Action {
    Empty,
    Exit,
    Move((i32, i32)),
    Fullscreen
}


pub fn handle_input(keypress: Key) -> Action {
    match keypress {
        Key { code: Escape, ..} => Action::Exit,
        Key { code: Up, ..}        | Key { code: NumPad8, ..} => Action::Move((0, -1)),
        Key { code: Down, ..}      | Key { code: NumPad2, ..} => Action::Move((0, 1)),
        Key { code: Left, ..}      | Key { code: NumPad4, ..} => Action::Move((-1, 0)),
        Key { code: Right, ..}     | Key { code: NumPad6, ..} => Action::Move((1, 0)),
        Key { code: Home, ..}      | Key { code: NumPad7, ..} => Action::Move((-1, -1)),
        Key { code: PageUp, ..}    | Key { code: NumPad9, ..} => Action::Move((1, -1)),
        Key { code: End, ..}       | Key { code: NumPad1, ..} => Action::Move((-1, 1)),
        Key { code: PageDown, ..}  | Key { code: NumPad3, ..} => Action::Move((1, 1)),
        Key { code: Enter, left_alt: true, .. }  => Action::Fullscreen,
        _ => Action::Empty,
    }
}
