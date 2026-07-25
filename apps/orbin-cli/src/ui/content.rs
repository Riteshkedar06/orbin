use crate::app::state::App;

use ratatui::{
    layout::{Alignment, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let content = Paragraph::new(app.screen().title())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Content")
                .borders(Borders::ALL),
        );

    frame.render_widget(content, area);
}