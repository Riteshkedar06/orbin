use crate::app::{
    screen::Screen,
    state::App,
};

use ratatui::{
    layout::Rect,
    text::Line,
    widgets::Paragraph,
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let lines: Vec<Line> = Screen::ALL
        .iter()
        .map(|screen| {
            let prefix = if *screen == app.screen() {
                "▶ "
            } else {
                "  "
            };

            Line::from(format!("{}{}", prefix, screen.title()))
        })
        .collect();

    let sidebar = Paragraph::new(lines);

    frame.render_widget(sidebar, area);
}