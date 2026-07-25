use ratatui::{
    layout::Alignment,
    widgets::Paragraph,
    Frame,
};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
    let header = Paragraph::new(" Orbin                               localhost ")
        .alignment(Alignment::Left);

    frame.render_widget(header, area);
}