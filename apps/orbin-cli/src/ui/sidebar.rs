use ratatui::{
    text::Line,
    widgets::Paragraph,
    Frame,
};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
    let sidebar = Paragraph::new(vec![
        Line::from("Dashboard"),
        Line::from("Processes"),
        Line::from("Logs"),
        Line::from("Services"),
        Line::from("Network"),
        Line::from("Alerts"),
        Line::from("Cases"),
        Line::from("Settings"),
    ]);

    frame.render_widget(sidebar, area);
}