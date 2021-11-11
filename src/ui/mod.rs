use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Row, Table};
use tui::Frame;

use crate::app::{App, Day};

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(70),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(f.size());

    let day_header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(main_layout[1]);
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(main_layout[2]);
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
        .margin(1)
        .split(content_layout[0]);
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
            .margin(1)
            .split(content_layout[1 + idx])
    };

    let mut day_layouts = vec![];
    (0..7_usize).for_each(|i| day_layouts.push(make_day_layout(i)));

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

    let week = app.get_active_week().expect("ERR: Active week not found!");

    let mut codes = vec![];
    let mut days = vec![vec![]; 7];

    let mut handle = |day: &Option<Day>, day_idx: usize| match day {
        Some(m) => days[day_idx].push(m.hours),
        None => days[day_idx].push(-1.0_f32),
    };

    for (idx, tc) in week.0.iter().enumerate() {
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
                .style(Style::default().add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL))
        } else {
            Paragraph::new(tc.timecode.clone()).block(Block::default().borders(Borders::ALL))
        };
        f.render_widget(tc_str, tc_layout[idx])
    }

    // RENDER: Main content (hours and day-labels)
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

    // Bør kunne vises og skjules
    let controls = Paragraph::new("h - Venstre, l - høyre")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Kontrols")
                .border_type(BorderType::Plain),
        );
    f.render_widget(controls, main_layout[3]);
}