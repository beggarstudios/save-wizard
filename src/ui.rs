use figlet_rs::FIGfont;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    app::{App, Screen},
    screens::{entity_types, home, manifests, saves},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_title(frame, chunks[0]);

    match app.current_screen {
        Screen::Home => home::render(frame, chunks[1], app),
        Screen::Manifests => manifests::render(frame, chunks[1], app),
        Screen::EntityTypes => entity_types::render(frame, chunks[1], app),
        Screen::Saves => saves::render(frame, chunks[1], app),
    }

    render_footer(frame, chunks[2], app);
}

fn render_title(frame: &mut Frame, area: ratatui::layout::Rect) {
    let standard_font = FIGfont::standard().unwrap();
    let ascii_title = standard_font
        .convert("Save-Wizard")
        .map(|figure| figure.to_string())
        .unwrap_or_else(|| "Save-Wizard".to_string());

    let title = Paragraph::new(Text::styled(ascii_title, Style::default().fg(Color::Green)))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(title, area);
}

fn render_footer(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let go_back_message = match app.current_screen {
        Screen::Home => "Quit",
        _ => "Go back",
    };

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("q", Style::default().fg(Color::Red)),
        Span::raw(" / "),
        Span::styled("Esc", Style::default().fg(Color::Red)),
        Span::raw(format!(" {}", go_back_message)),
        Span::raw(" - "),
        Span::raw(" Status: "),
        Span::raw(&app.status_message),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}
