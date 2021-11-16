mod app;
mod config;
mod data;
mod event;
mod handlers;
mod i18n;
mod ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::env;
use std::error::Error;
use std::io::{self, stdout};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::State;

fn main() -> Result<(), Box<dyn Error>> {
    let path = format!(
        "{}/.config/timekeeper/timer.json",
        env::var("HOME").expect("ERR: HOME variable not set!"),
    );
    // let mut app = app::App::new("src/resources/timer.json")?;
    let mut app = app::App::new(path)?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = event::Events::new(250);
    let mut tick = 0;

    terminal.draw(|f| ui::draw_main_layout(f, &app))?;
    loop {
        if app.get_state() == &State::Quit {
            app.write();
            break;
        }
        match events.next()? {
            event::Event::Input(key) => {
                // Force quit
                if key == event::Key::Ctrl('c') {
                    break;
                    // Quit and save
                }
                handlers::handle_app(key, &mut app);
                terminal.draw(|f| ui::draw_main_layout(f, &app))?;
            }
            event::Event::Tick => {
                tick += 1;
                // Draw every so often in case of resize
                if tick % 2 == 0 {
                    terminal.draw(|f| ui::draw_main_layout(f, &app))?;
                }
            }
        }
    }

    terminal.show_cursor()?;
    close_application()?;
    Ok(())
}

fn close_application() -> std::io::Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
