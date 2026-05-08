use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod input;
mod ui;

use crate::{app::App, ui::ui};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            input::handle_key_event(app, key);
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
