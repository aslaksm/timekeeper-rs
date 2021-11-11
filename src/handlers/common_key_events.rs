use super::super::app::App;
use crate::event::Key;

pub fn down_event(key: Key) -> bool {
    matches!(key, Key::Down | Key::Char('j') | Key::Ctrl('n'))
}

pub fn up_event(key: Key) -> bool {
    matches!(key, Key::Up | Key::Char('k') | Key::Ctrl('p'))
}

pub fn left_event(key: Key) -> bool {
    matches!(key, Key::Left | Key::Char('h') | Key::Ctrl('b'))
}

pub fn right_event(key: Key) -> bool {
    matches!(key, Key::Right | Key::Char('l') | Key::Ctrl('f'))
}

pub fn toggle_select_event(key: Key) -> bool {
    matches!(key, Key::Char(' ') | Key::Enter | Key::Char('a'))
}

pub fn write_event(key: Key) -> bool {
    matches!(key, Key::Char('w'))
}

pub fn toggle_comment_event(key: Key) -> bool {
    matches!(key, Key::Char('c'))
}
pub fn quit_event(key: Key) -> bool {
    matches!(key, Key::Char('q') | Key::Esc)
}

pub fn inc_event(key: Key) -> bool {
    matches!(key, Key::Char('K') | Key::ShiftUp)
}
pub fn dec_event(key: Key) -> bool {
    matches!(key, Key::Char('J') | Key::ShiftDown)
}
pub fn num_event(c: char) -> bool {
    matches!(c, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')
}
