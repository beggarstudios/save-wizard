use std::fs;
use ratatui::widgets::ListState;

pub enum CurrentScreen {
    Main,
    Exiting
}

pub struct App<'a> {
    pub config: AppConfig,
    pub input: AppInput<'a>,
    pub input_files: Vec<String>,
    pub current_screen: CurrentScreen,
}

pub struct AppConfig {
    pub data_directory: String,
}

pub struct AppInput<'a> {
    pub input_list_state: ListState,
    pub active_list: Option<&'a ListState>,
}

impl AppConfig {
    pub fn new(args: &[String]) -> AppConfig {
        AppConfig {
            data_directory: args[1].to_string(),
        }
    }
}

impl AppInput<'_> {
    pub fn new() -> AppInput<'static> {
        AppInput {
            input_list_state: ListState::default(),
            active_list: None,
        }
    }

    pub fn next(&mut self) {
        self.input_list_state.select_next();
    }
}

impl App<'_> {
    /// Constructor function
    pub fn new(args: &[String]) -> App {
        App {
            config: AppConfig::new(args),
            input: AppInput::new(),
            input_files: Vec::new(),
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn initialize_data(&mut self) {
        let paths = fs::read_dir(&self.config.data_directory).unwrap();

        for path in paths {
            self.input_files.push(path.unwrap().path().display().to_string());
        }
    }
}