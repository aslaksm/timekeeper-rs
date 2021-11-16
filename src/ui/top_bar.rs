use crate::app::App;
use crate::i18n::I18n;
use chrono::Datelike;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

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
        Span::styled(
            format!("{} {} ", I18n::week_label(&app.conf.lang), app.active_week),
            Style::default(),
        ),
        Span::styled(app.active_year.to_string(), Style::default()),
    ]);
    let year_week_p = Paragraph::new(year_week_spans)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(year_week_p, top_bar_layout[0]);

    let mut weekday = chrono::Weekday::Mon;
    for idx in 0..7 {
        let day =
            chrono::NaiveDate::from_isoywd(app.active_year as i32, app.active_week as u32, weekday);
        let d = Paragraph::new(format!("{}/{}", day.day(), day.month()))
            .wrap(Wrap { trim: true })
            .style(if idx as u8 == app.active_day {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            })
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(d, top_bar_layout[1 + idx]);
        weekday = weekday.succ(); // SUCC
    }
}
