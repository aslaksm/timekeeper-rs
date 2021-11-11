use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use tui::Frame;

use crate::app::{App, Day, State};

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    // Hovedinndelinga
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Min(5),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .margin(0)
        .split(f.size());

    // Headeren for dager (Der det står Mandag, Tirsdag etc)
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
        .margin(0)
        .split(main_layout[1]);

    // Innholdsinndelinga (timecode-label, timer, kommentar)
    let comment_col_width = 10;
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
                Constraint::Length(comment_col_width),
            ]
            .as_ref(),
        )
        .margin(0)
        .split(main_layout[2]);

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
        .margin(0)
        .split(content_layout[0]);

    // Kommentar (høyre side)
    let comment_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(0)
        .split(content_layout[8]);

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
            .margin(0)
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

    let comment = match app.get_active_day() {
        Some(d) => d.comment.clone(),
        None => String::from(""),
    };
    let styled_comment = Spans::from(if app.should_show_cursor() {
        vec![
            Span::styled(comment, Style::default().fg(Color::LightYellow)),
            Span::styled(
                String::from("|"),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::SLOW_BLINK),
            ),
        ]
    } else {
        vec![Span::styled(String::from(comment), Style::default())]
    });
    let comment_box = Paragraph::new(styled_comment)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::LEFT)
                .style(Style::default().fg(Color::White))
                .title("Kommentar")
                .border_type(BorderType::Plain),
        );
    f.render_widget(comment_box, comment_layout[0]);

    // Bør kunne vises og skjules
    let controls = Paragraph::new("Navigering: hjkl eller piltaster")
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
