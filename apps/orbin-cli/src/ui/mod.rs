use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame) {
    let area = frame.area();

    let block = Block::default()
        .title(" Orbin ")
        .borders(Borders::ALL);

    let welcome = Paragraph::new("Welcome to Orbin")
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(welcome, area);
}