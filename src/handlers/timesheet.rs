use super::common_key_events;
use crate::app::{App, State};
use crate::event::Key;

// TODO: Wtf is this filename

pub fn handle(key: Key, app: &mut App) {
    match app.get_state() {
        State::Browsing => handle_browsing(key, app),
        State::WritingComment => handle_comment_input(key, app),
    }
}

pub fn handle_browsing(key: Key, app: &mut App) {
    match key {
        k if common_key_events::down_event(k) => app.next_timecode(),
        k if common_key_events::up_event(k) => app.prev_timecode(),
        k if common_key_events::left_event(k) => app.prev_day(),
        k if common_key_events::right_event(k) => app.next_day(),
        k if common_key_events::save_event(k) => app.write(),
        k if common_key_events::toggle_comment_event(k) => app.toggle_writing_comment(),
        k if common_key_events::inc_event(k) => app.change_hours(0.5),
        k if common_key_events::dec_event(k) => app.change_hours(-0.5),
        Key::Char(c) if common_key_events::num_event(c) => {
            app.set_hours(c.to_digit(10).unwrap() as f32)
        }
        _ => (),
    }
}

pub fn handle_selected(key: Key, app: &mut App) {
    match key {
        // k if common_key_events::toggle_select_event(k) => app.toggle_select(),
        k if common_key_events::save_event(k) => app.write(),
        _ => (),
    }
}

pub fn handle_comment_input(key: Key, app: &mut App) {
    match key {
        Key::Char(c) => app.append_char_to_comment(c),
        Key::Esc => app.toggle_writing_comment(),
        Key::Backspace => app.delete_char_from_comment(),
        _ => (),
    }
}
