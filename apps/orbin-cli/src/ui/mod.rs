mod content;
mod footer;
mod header;
mod layout;
mod sidebar;

use crate::app::state::App;
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &App) {
    let shell = layout::build(frame.area());

    header::render(frame, shell.header);
    sidebar::render(frame, shell.sidebar, app);
    content::render(frame, shell.content, app);
    footer::render(frame, shell.footer);
}
