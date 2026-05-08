pub enum Screen {
    Home,
}

pub enum AppMode {
    Normal,
}

pub enum Panel {
    List,
    Details,
}

pub struct App {
    pub should_quit: bool,
    pub current_screen: Screen,
    pub app_mode: AppMode,
    pub focus: Panel,
    pub selected_index: usize,
    pub status_message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            current_screen: Screen::Home,
            app_mode: AppMode::Normal,
            focus: Panel::List,
            selected_index: 0,
            status_message: String::new(),
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn focus_next(&mut self) {
        self.focus = match self.focus {
            Panel::List => Panel::Details,
            Panel::Details => Panel::List,
        };
    }
}
