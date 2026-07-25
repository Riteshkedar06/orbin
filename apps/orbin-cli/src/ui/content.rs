use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
    let content = Paragraph::new(
        "Welcome to Orbin\n\nLinux Operations Workspace",
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(content, area);
}