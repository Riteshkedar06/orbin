mod app;
mod terminal;
mod ui;

use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};

use app::state::App;
use terminal::TerminalGuard;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = TerminalGuard::new()?;
    let mut app = App::new();

    while app.running() {
        terminal.terminal().draw(|frame| ui::render(frame))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    app.quit();
                }
            }
        }
    }

    Ok(())
}
