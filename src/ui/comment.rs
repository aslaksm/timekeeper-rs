use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use tui::Frame;

pub fn draw_comment<B>(f: &mut Frame<B>, app: &App, layout: &Rect)
where
    B: Backend,
{
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
    f.render_widget(comment_box, *layout);
}
