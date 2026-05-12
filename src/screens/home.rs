use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use ratatui::layout::Rect;

use crate::app::{App, Panel};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    // Split the body vertically
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    // Render details panel
    render_list(frame, body_chunks[0], app);
    render_details(frame, body_chunks[1], app);
}

fn render_details(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    // Calculate and show current panel

    let border_style = if matches!(app.focus, Panel::Details) {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let Some(selection_context) = app.menu_items.get(app.selected_menu_index) else {
        return;
    };

    let content = format!(
        "{}\n\n{}",
        selection_context.heading(),
        selection_context.description()
    );

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

fn render_list(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    // Calculate and show current panel

    let border_style = if matches!(app.focus, Panel::Menu) {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let selected_menu_option_style = Style::default().fg(Color::Cyan);

    let lines = app
        .menu_items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            if index == app.selected_menu_index {
                Line::from(format!("> {}", item.label())).style(selected_menu_option_style)
            } else {
                Line::from(format!("  {}", item.label()))
            }
        })
        .collect::<Vec<Line>>();

    let body = Paragraph::new(Text::from(lines)).block(
        Block::default()
            .title("List")
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    frame.render_widget(body, area);
}
