use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use ratatui::layout::Rect;

use crate::app::{App, Screen};

pub fn render(frame: &mut Frame, area: Rect, _app: &App) {
    render_body(frame, area, _app);
}

fn render_body(frame: &mut Frame, area: ratatui::layout::Rect, _app: &App) {
    let border_style = Style::default().fg(Color::Green);

    let content = format!("{}\n\n{}", Screen::EntityTypes.label(), "Placeholder");

    let body = Paragraph::new(content)
        .block(
            Block::default()
                .title("Details")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(body, area);
}
