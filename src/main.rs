mod app;
mod event;
mod handlers;
mod ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io::{self, stdout};
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = app::App::new("src/resources/timer.json")?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = event::Events::new(250);

    loop {
        terminal.draw(|f| ui::draw_main_layout(f, &app))?;
        match events.next()? {
            event::Event::Input(key) => {
                if key == event::Key::Ctrl('c') {
                    break;
                } else {
                    handlers::handle_app(key, &mut app);
                }
            }
            event::Event::Tick => (),
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
