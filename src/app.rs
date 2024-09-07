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
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>
}

impl App {

    /// Constructor function
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}