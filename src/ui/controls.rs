use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use tui::Frame;

use crate::app::{App, Day};

pub fn draw_controls<B>(f: &mut Frame<B>, app: &App, layout: &Rect)
where
    B: Backend,
{
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
    f.render_widget(controls, *layout);
}
