use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct ShellLayout {
    pub header: Rect,
    pub sidebar: Rect,
    pub content: Rect,
    pub footer: Rect,
}

pub fn build(area: Rect) -> ShellLayout {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),
            Constraint::Min(1),
        ])
        .split(vertical[1]);

    ShellLayout {
        header: vertical[0],
        sidebar: middle[0],
        content: middle[1],
        footer: vertical[2],
    }
}