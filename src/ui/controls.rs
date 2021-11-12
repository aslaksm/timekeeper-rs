use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

pub fn draw_controls<B>(f: &mut Frame<B>, _app: &App, layout: &Rect)
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
