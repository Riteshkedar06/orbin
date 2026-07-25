use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::state::App;

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) => app.quit(),

        (KeyCode::Char('c'), KeyModifiers::CONTROL) => app.quit(),

        (KeyCode::Down, _) | (KeyCode::Char('j'), _) => {
            app.next_screen();
        }

        (KeyCode::Up, _) | (KeyCode::Char('k'), _) => {
            app.previous_screen();
        }

        _ => {}
    }
}