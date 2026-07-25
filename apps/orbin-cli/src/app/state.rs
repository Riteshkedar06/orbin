use super::screen::Screen;

pub struct App {
    running: bool,
    current_screen: Screen,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            current_screen: Screen::Dashboard,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn screen(&self) -> Screen {
        self.current_screen
    }

    pub fn set_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }
}
