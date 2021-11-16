use crate::app::App;
use crate::i18n::I18n;
use crate::ui::day_headers::draw_day_headers;
use crate::ui::tc_labels::draw_timecode_labels;
use crate::ui::top_bar::draw_top_bar;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
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
        // .map(|(a, b)| Row::new(vec![*b, *b]))
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
    // .header(
    //     Row::new(vec!["Tast", "Kommando"])
    //         .style(Style::default().add_modifier(Modifier::BOLD))
    //         .bottom_margin(1),
    // );

    f.render_widget(control_table, main_layout[0]);
}
