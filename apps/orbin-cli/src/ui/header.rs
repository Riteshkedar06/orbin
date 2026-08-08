use ratatui::{Frame, layout::Alignment, widgets::Paragraph};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
    let header = Paragraph::new(" Orbin                               localhost ")
        .alignment(Alignment::Left);

    frame.render_widget(header, area);
}
