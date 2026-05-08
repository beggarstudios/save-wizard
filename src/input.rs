use ratatui::crossterm::event::{self, KeyCode, KeyEvent};

use crate::app::App;

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
        _ => {}
    }
}
