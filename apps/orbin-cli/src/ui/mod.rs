mod content;
mod footer;
mod header;
mod layout;
mod sidebar;

use ratatui::Frame;

pub fn render(frame: &mut Frame) {
    let shell = layout::build(frame.area());

    header::render(frame, shell.header);
    sidebar::render(frame, shell.sidebar);
    content::render(frame, shell.content);
    footer::render(frame, shell.footer);
}