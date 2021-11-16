use crate::app::{App, State};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
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
                /* XXX: Percentage divides the area into 5 chunks as best as it can.
                 * There are some inconsistencies depending on resolution, but using
                 * fixed size based on height doesn't really fix the issue.
                 */
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(*layout);

    let fav_style = Style::default().fg(Color::Green);

    let start = app.timecode_range[0];
    let end = app.timecode_range[1];

    let week = app.get_active_week().expect("ERR: Active week not found!");
    for (idx, tc) in week.0[start..end].iter().enumerate() {
        let offset_idx = idx + start;
        let style = if app.starred_timecodes.contains(&app.timecodes[offset_idx]) {
            fav_style
        } else {
            Style::default()
        };
        let tc_str =
            if app.active_timecode == (offset_idx) && app.get_state() != &State::AddingTimecode {
                Paragraph::new(tc.timecode.clone())
                    .wrap(Wrap { trim: true })
                    .style(Style::default().add_modifier(Modifier::BOLD))
                    .block(Block::default().borders(Borders::ALL).border_style(style))
            } else {
                Paragraph::new(tc.timecode.clone())
                    .wrap(Wrap { trim: true })
                    .block(Block::default().borders(Borders::ALL).border_style(style))
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
        f.render_widget(
            tc_str,
            tc_layout[app.timecode_range[1] - app.timecode_range[0]],
        )
    }
}
