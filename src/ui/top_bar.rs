use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use tui::Frame;

use crate::app::{App, Day};

pub fn draw_top_bar<B>(f: &mut Frame<B>, app: &App, layout: &Rect)
where
    B: Backend,
{
    let t_width = f.size().width;

    let top_bar_layout = Layout::default()
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
        .split(*layout);

    // RENDER: Cur week and year
    let year_week_spans = Spans::from(vec![
        Span::styled(app.active_year.to_string(), Style::default()),
        Span::styled(format!(", Uke {}", app.active_week), Style::default()),
    ]);
    let year_week_p = Paragraph::new(year_week_spans)
        .wrap(Wrap { trim: true })
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(year_week_p, top_bar_layout[0]);
}
