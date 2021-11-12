use crate::app::{App, State};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

pub fn draw_timecode_labels<B>(f: &mut Frame<B>, app: &App, layout: &Rect)
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
        .split(*layout);

    let week = app.get_active_week().expect("ERR: Active week not found!");
    for (idx, tc) in week.0.iter().enumerate() {
        // RENDER: Timecode labels
        let tc_str = if app.active_timecode == idx && app.get_state() != &State::AddingTimecode {
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

    if app.get_state() == &State::AddingTimecode {
        let text = Spans::from(vec![
            Span::styled(app.timecode_buffer.clone(), Style::default()),
            Span::styled(
                String::from("|"),
                Style::default().add_modifier(Modifier::SLOW_BLINK),
            ),
        ]);

        let tc_str = Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(tc_str, tc_layout[week.0.len()])
    }
}
