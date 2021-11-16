use crate::app::App;
use crate::i18n::I18n;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};

use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

pub fn draw_info<B>(f: &mut Frame<B>, app: &App, layout: &Rect)
where
    B: Backend,
{
    // TODO: Expand to show various info to user (error messages, saved status etc)
    if !app.conf.has_seen_info {
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
        let info = Paragraph::new(I18n::info_screen(&app.conf.lang))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Info")
                    .border_type(BorderType::Plain),
            );
        f.render_widget(info, info_layout[1]);

        if (app.timecode_range[1]) < app.timecodes.len() {
            let down_arrow = Paragraph::new("↓")
                .block(Block::default())
                .alignment(Alignment::Center);
            f.render_widget(down_arrow, info_layout[0]);
        }
    }
}
