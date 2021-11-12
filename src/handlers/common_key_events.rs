use crate::event::Key;

/* XXX: Some of these are useful, particularly if i'm going to add
 * custom keybinds down the line. But atm a lot of these are kind of pointless
 */
pub fn down_event(key: Key) -> bool {
    matches!(key, Key::Down | Key::Char('j'))
}

pub fn up_event(key: Key) -> bool {
    matches!(key, Key::Up | Key::Char('k'))
}

pub fn left_event(key: Key) -> bool {
    matches!(key, Key::Left | Key::Char('h'))
}

pub fn right_event(key: Key) -> bool {
    matches!(key, Key::Right | Key::Char('l'))
}

pub fn save_event(key: Key) -> bool {
    matches!(key, Key::Char('w'))
}

pub fn toggle_comment_event(key: Key) -> bool {
    matches!(key, Key::Char('c') | Key::Enter)
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
pub fn new_timecode_event(key: Key) -> bool {
    matches!(key, Key::Char('N'))
}
pub fn num_event(c: char) -> bool {
    matches!(c, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')
}
