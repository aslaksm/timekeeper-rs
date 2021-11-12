use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

pub fn draw_day_headers<B>(f: &mut Frame<B>, app: &App, layout: &Rect)
where
    B: Backend,
{
    let t_width = f.size().width;

    // Headeren for dager (Der det står Mandag, Tirsdag etc)
    let day_header_layout = Layout::default()
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
                Constraint::Length(t_width / 10),
                Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .split(*layout);

    let to_span = |a| Span::styled(a, Style::default());
    let header = vec![
        to_span("Mandag"),
        to_span("Tirsdag"),
        to_span("Onsdag"),
        to_span("Torsdag"),
        to_span("Fredag"),
        to_span("Lørdag"),
        to_span("Søndag"),
    ];

    for (idx, day) in header.into_iter().enumerate() {
        let d_header = if app.active_day == idx as u8 {
            Paragraph::new(day).block(
                Block::default()
                    .style(Style::default().add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL),
            )
        } else {
            Paragraph::new(day).block(Block::default().borders(Borders::ALL))
        };
        f.render_widget(d_header, day_header_layout[idx + 1]);
    }
}
