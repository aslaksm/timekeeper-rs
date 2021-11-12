use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use tui::Frame;

use crate::app::{App, Day};

pub fn draw_top_bar<B>(f: &mut Frame<B>, app: &App, main_layout: &Vec<Rect>)
where
    B: Backend,
{
    // Timecode-label inndeling
    let tc_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(content_layout[0]);

    for (idx, tc) in app.get_active_week().iter().enumerate() {
        codes.push(tc.timecode.clone());
        handle(&tc.monday, 0);
        handle(&tc.tuesday, 1);
        handle(&tc.wednesday, 2);
        handle(&tc.thursday, 3);
        handle(&tc.friday, 4);
        handle(&tc.saturday, 5);
        handle(&tc.sunday, 6);

        // RENDER: Timecode labels
        let tc_str = if app.active_timecode == idx {
            Paragraph::new(tc.timecode.clone())
                .wrap(Wrap { trim: true })
                .style(Style::default().add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL))
        } else {
            Paragraph::new(tc.timecode.clone())
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL))
        };
        f.render_widget(tc_str, tc_layout[idx])
    }
}
