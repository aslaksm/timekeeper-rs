use crate::app::App;
use crate::i18n::I18n;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Cell, Row, Table};
use tui::Frame;

// Info screen that shows controls
pub fn draw_control_screen<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let t_width = f.size().width;

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        // .margin(t_width / 10)
        .split(f.size());

    let keys = I18n::key_labels(&app.conf.lang);
    let key_strings: Vec<Span> = keys
        .into_iter()
        .map(|a| {
            Span::styled(
                format!("{:>width$}", a, width = (t_width as usize * 2 / 10)),
                Style::default().fg(Color::Magenta),
            )
        })
        .collect();

    let actions = I18n::action_labels(&app.conf.lang);
    let action_strings: Vec<Span> = actions
        .into_iter()
        .map(|a| Span::styled(a, Style::default()))
        .collect();

    let rows: Vec<Row> = key_strings
        .into_iter()
        .zip(action_strings.into_iter())
        .map(|(a, b)| Row::new(vec![Cell::from(a), Cell::from(b)]))
        .collect();

    let cols = [
        Constraint::Length(t_width * 2 / 10),
        Constraint::Length(t_width * 8 / 10),
    ];

    let control_table = Table::new(rows)
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .widths(&cols);

    f.render_widget(control_table, main_layout[0]);
}
