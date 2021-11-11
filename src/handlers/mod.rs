use crate::app::App;
use crate::event::Key;
pub mod common_key_events;
mod timesheet;

pub fn handle_app(key: Key, app: &mut App) {
    match key {
        _ => handle_block_events(key, app),
    }
}

// Handle event for the current active block
fn handle_block_events(key: Key, app: &mut App) {
    match key {
        _ => timesheet::handle(key, app),
    }
}
