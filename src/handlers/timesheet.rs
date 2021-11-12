use super::common_key_events;
use crate::app::{App, State};
use crate::event::Key;

// TODO: Wtf is this filename

pub fn handle(key: Key, app: &mut App) {
    match app.get_state() {
        State::Browsing => handle_browsing(key, app),
        State::WritingComment => handle_comment_input(key, app),
        State::AddingTimecode => handle_timecode_input(key, app),
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
        k if common_key_events::new_timecode_event(k) => app.add_timecode(String::from("Halla")),
        Key::Char(c) if common_key_events::num_event(c) => {
            app.set_hours(c.to_digit(10).unwrap() as f32)
        }
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

pub fn handle_timecode_input(key: Key, app: &mut App) {
    match key {
        Key::Char(c) => app.append_char_to_timecode_buffer(c),
        Key::Esc => app.toggle_adding_timecode(),
        Key::Backspace => app.delete_char_from_timecode_buffer(),
        _ => (),
    }
}
