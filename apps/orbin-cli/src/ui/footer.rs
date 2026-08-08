use ratatui::{Frame, layout::Alignment, widgets::Paragraph};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
    let footer =
        Paragraph::new("q Quit                              v0.1.0-dev").alignment(Alignment::Left);

    frame.render_widget(footer, area);
}
