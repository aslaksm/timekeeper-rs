use super::common_key_events;
use crate::app::{App, State};
use crate::event::Key;

pub fn handle(key: Key, app: &mut App) {
    match app.state {
        State::Browsing => handle_browsing(key, app),
        State::Selected => handle_selected(key, app),
    }
}

pub fn handle_browsing(key: Key, app: &mut App) {
    match key {
        k if common_key_events::down_event(k) => app.next_timecode(),
        k if common_key_events::up_event(k) => app.prev_timecode(),
        k if common_key_events::left_event(k) => app.prev_day(),
        k if common_key_events::right_event(k) => app.next_day(),
        k if common_key_events::select_event(k) => app.toggle_select(),
        k if common_key_events::write_event(k) => app.write(),
        _ => (),
    }
}

pub fn handle_selected(key: Key, app: &mut App) {
    match key {
        k if common_key_events::up_event(k) => app.change_hours(0.5),
        k if common_key_events::down_event(k) => app.change_hours(-0.5),
        k if common_key_events::select_event(k) => app.toggle_select(),
        k if common_key_events::write_event(k) => app.write(),
        _ => (),
    }
}
