mod app;
mod events;
mod terminal;
mod ui;

use std::time::Duration;

use app::state::App;
use color_eyre::Result;
use crossterm::event::{self, Event};
use terminal::TerminalGuard;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = TerminalGuard::new()?;
    let mut app = App::new();

    while app.running() {
        terminal
            .terminal()
            .draw(|frame| ui::render(frame, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                events::handle_key(&mut app, key);
            }
        }
    }

    Ok(())
}