use crate::app::App;
use crate::data::Day;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

pub fn draw_days<B>(f: &mut Frame<B>, app: &App, content_layout: &Vec<Rect>)
where
    B: Backend,
{
    // Dag-inndelinger
    let make_day_layout = |idx: usize| {
        Layout::default()
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
            .split(content_layout[1 + idx])
    };

    let mut day_layouts = vec![];
    (0..7_usize).for_each(|i| day_layouts.push(make_day_layout(i)));

    let week = app.get_active_week().expect("ERR: Active week not found!");

    let mut codes = vec![];
    let mut days = vec![vec![]; 7];

    let mut handle = |day: &Option<Day>, day_idx: usize| match day {
        Some(m) => days[day_idx].push(m.hours),
        None => days[day_idx].push(-1.0_f32),
    };

    for (_idx, tc) in week.0.iter().enumerate() {
        codes.push(tc.timecode.clone());
        handle(&tc.monday, 0);
        handle(&tc.tuesday, 1);
        handle(&tc.wednesday, 2);
        handle(&tc.thursday, 3);
        handle(&tc.friday, 4);
        handle(&tc.saturday, 5);
        handle(&tc.sunday, 6);
    }

    // RENDER: Main content (hours and day-labels)
    for idx in 0..7 {
        for (tc_idx, d) in days[idx].iter().enumerate() {
            let (block, style) = if tc_idx == app.active_timecode && idx as u8 == app.active_day {
                (
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick),
                    Style::default().add_modifier(Modifier::BOLD),
                )
            } else {
                (Block::default().borders(Borders::ALL), Style::default())
            };
            if *d < 0.0 {
                let p = Paragraph::new("").block(block).style(style);
                f.render_widget(p, day_layouts[idx][tc_idx]);
            } else {
                let p = Paragraph::new(d.to_string()).block(block).style(style);
                f.render_widget(p, day_layouts[idx][tc_idx]);
            }
        }
    }
}
