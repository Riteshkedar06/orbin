#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Dashboard,
    Processes,
    Logs,
    Services,
    Network,
    Alerts,
    Cases,
    Settings,
}

impl Screen {
    pub const ALL: [Screen; 8] = [
        Screen::Dashboard,
        Screen::Processes,
        Screen::Logs,
        Screen::Services,
        Screen::Network,
        Screen::Alerts,
        Screen::Cases,
        Screen::Settings,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Screen::Dashboard => "Dashboard",
            Screen::Processes => "Processes",
            Screen::Logs => "Logs",
            Screen::Services => "Services",
            Screen::Network => "Network",
            Screen::Alerts => "Alerts",
            Screen::Cases => "Cases",
            Screen::Settings => "Settings",
        }
    }
}
