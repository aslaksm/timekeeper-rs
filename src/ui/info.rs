use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

pub fn draw_info<B>(f: &mut Frame<B>, app: &App, layout: &Rect)
where
    B: Backend,
{
    let t_width = f.size().width;

    // Headeren for dager (Der det står Mandag, Tirsdag etc)
    let info_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Length(t_width * 6 / 10),
                Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .split(*layout);

    // Bør kunne vises og skjules
    let info = Paragraph::new("Navigering: hjkl eller piltaster")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Info")
                .border_type(BorderType::Plain),
        );
    f.render_widget(info, info_layout[1]);

    if (app.timecode_offset + 5) < app.timecodes.len() {
        let down_arrow = Paragraph::new("↓")
            .block(Block::default())
            .alignment(Alignment::Center);
        f.render_widget(down_arrow, info_layout[0]);
    }
}
