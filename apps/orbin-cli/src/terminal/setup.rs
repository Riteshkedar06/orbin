use std::io::{self, Stdout};

use color_eyre::Result;
use crossterm::{
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

pub type OrbinTerminal = Terminal<CrosstermBackend<Stdout>>;

/// Owns the terminal lifecycle.
///
/// When dropped, the terminal is automatically restored.
pub struct TerminalGuard {
    terminal: OrbinTerminal,
}

impl TerminalGuard {
    /// Initialize the terminal.
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    /// Mutable access to the Ratatui terminal.
    pub fn terminal(&mut self) -> &mut OrbinTerminal {
        &mut self.terminal
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}