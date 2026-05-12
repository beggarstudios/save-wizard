use ratatui::crossterm::event::{self, KeyCode, KeyEvent};

use crate::app::{App, Panel};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    if key.kind == event::KeyEventKind::Release {
        return;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Tab => {
            app.focus_next();
        }
        _ => match app.focus {
            Panel::List => match key.code {
                KeyCode::Up => app.list_previous(),
                KeyCode::Down => app.list_next(),
				KeyCode::Enter => app.activate_screen(),
                _ => {}
            },
            Panel::Details => {}
        },
    }
}
