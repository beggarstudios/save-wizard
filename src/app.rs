pub enum Screen {
    Home,
	Manifests,
	EntityTypes,
	Saves
}

pub enum AppMode {
    Normal,
}

pub enum Panel {
    List,
    Details,
}

pub enum MenuItem {
    Manifests,
    EntityTypes,
    Saves,
}

pub struct App {
    pub should_quit: bool,
    pub current_screen: Screen,
    pub app_mode: AppMode,
    pub focus: Panel,
    pub status_message: String,

    // menus
    pub menu_items: Vec<MenuItem>,
    pub selected_menu_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            current_screen: Screen::Home,
            app_mode: AppMode::Normal,
            focus: Panel::List,
            selected_menu_index: 0,
            status_message: String::new(),
            menu_items: vec![
                MenuItem::Manifests,
                MenuItem::EntityTypes,
                MenuItem::Saves,
            ],
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

    pub fn list_next(&mut self) {
        if self.menu_items.is_empty() {
            return;
        }
        self.selected_menu_index = (self.selected_menu_index + 1) % self.menu_items.len();
    }

    pub fn list_previous(&mut self) {
        if self.menu_items.is_empty() {
            return;
        }
        if self.selected_menu_index == 0 {
            self.selected_menu_index = self.menu_items.len() - 1;
            return;
        }

        self.selected_menu_index -= 1;
    }

	pub fn activate_screen(&mut self) {

		let Some(selected_item) =
			self.menu_items.get(self.selected_menu_index)
		else {
			return;
		};
		let queued_screen = match selected_item {
			MenuItem::Manifests => Screen::Manifests,
			MenuItem::EntityTypes => Screen::EntityTypes,
			MenuItem::Saves => Screen::Saves,
		};

		self.current_screen = queued_screen;
	}
}

impl MenuItem {
    pub fn label(&self) -> &'static str {
        match self {
            MenuItem::Manifests => "Manifests",
            MenuItem::EntityTypes => "Entity types",
            MenuItem::Saves => "Save files",
        }
    }

	 pub fn heading(&self) -> &'static str {
        match self {
            MenuItem::Manifests => "Game manifests",
            MenuItem::EntityTypes => "Game entity types",
            MenuItem::Saves => "Save files",
        }
    }

	 pub fn description(&self) -> &'static str {
        match self {
            MenuItem::Manifests => r#"Import and manage manifests from your game.

Manifests define your game's available data structures that Save Wizard can read data from in order to build save structures."#,

            MenuItem::EntityTypes => r#"Manage entity types from your manifests.

Entity types are definitions of content from your game's manifests. These define the content that can appear in generated save files such as inventories, items or enemies."#,

            MenuItem::Saves => r#"Manage save files.

Save files can be built from your defined entities and saved and exported from this section"#,
        }
    }
}

impl Screen {
	 pub fn label(&self) -> &'static str {
        match self {
            Screen::Home => "Home",
            Screen::Manifests => "Manifests",
            Screen::EntityTypes => "Entity Types",
            Screen::Saves => "Saves",
        }
    }
}