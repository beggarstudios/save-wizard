use figlet_rs::FIGfont;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Panel};

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
    render_body(frame, chunks[1], app);
    render_footer(frame, chunks[2]);
}

fn render_title(frame: &mut Frame, area: ratatui::layout::Rect) {
    let standard_font = FIGfont::standard().unwrap();
    let ascii_title = standard_font
        .convert("Save-Wizard")
        .map(|figure| figure.to_string())
        .unwrap_or_else(|| "Save-Wizard".to_string());

    let title = Paragraph::new(Text::styled(
        ascii_title,
        Style::default().fg(Color::Green),
    ))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(title, area);
}

fn render_body(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
	// Calculate and show current panel

	let focused_panel = match app.focus {
		Panel::Main => "Main",
		Panel::List => "List",
	};

    let body = Paragraph::new(Text::from(vec![
        Line::from("Bootstrap application shell"),
        Line::from(""),	
        Line::from("The TUI runtime is working."),
        Line::from("Next step: add app state and the first screen."),
        Line::from(""),
        Line::from(focused_panel),
    ]))
    .block(Block::default().title("Home").borders(Borders::ALL));

    frame.render_widget(body, area);
}

fn render_footer(frame: &mut Frame, area: ratatui::layout::Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("q", Style::default().fg(Color::Red)),
        Span::raw(" / "),
        Span::styled("Esc", Style::default().fg(Color::Red)),
        Span::raw(" quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}