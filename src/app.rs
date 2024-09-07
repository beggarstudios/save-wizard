use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting
}

pub enum CurrentlyEditing {
    Key,
    Value
}

pub struct App {
    pub config: App_config,
    pub input_text: String,
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>
}

pub struct App_config {
    pub data_directory: String,
}

impl App_config {
    pub fn new(args: &[String]) -> App_config {
        App_config {
            data_directory: args[1].to_string(),
        }
    }
}

impl App {
    /// Constructor function
    pub fn new(args: &[String]) -> App {
        App {
            config: App_config::new(args),
            input_text: String::new(),
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}