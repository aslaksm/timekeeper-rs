mod comment;
mod controls;
mod day_headers;
mod days;
mod info;
mod tc_labels;
mod top_bar;

use self::comment::draw_comment;
use self::controls::draw_control_screen;
use self::days::draw_days;
use self::info::draw_info;
use crate::app::{App, State};
use crate::ui::day_headers::draw_day_headers;
use crate::ui::tc_labels::draw_timecode_labels;
use crate::ui::top_bar::draw_top_bar;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let t_width = f.size().width;

    if app.get_state() == &State::ControlScreen {
        draw_control_screen(f, app);
        return;
    }

    // Main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Content layout (timecode-label, hours, comments)
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Length(t_width / 10),
                Constraint::Length(t_width / 10),
                Constraint::Length(t_width / 10),
                Constraint::Length(t_width / 10),
                Constraint::Length(t_width / 10),
                Constraint::Length(t_width / 10),
                Constraint::Length(t_width / 10),
                Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .split(main_layout[2]);

    draw_top_bar(f, app, &main_layout[0]);
    draw_timecode_labels(f, app, &content_layout[0]);
    draw_day_headers(f, app, &main_layout[1]);
    draw_days(f, app, &content_layout);
    draw_comment(f, app, &content_layout[8]);
    draw_info(f, app, &main_layout[3]);
}
