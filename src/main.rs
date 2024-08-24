use std::io::{self, stdout, Result};

mod tui;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

fn main() -> io::Result<()> {
    let mut terminal= tui::init()?;

    let app_result = App::default().run(&mut terminal);

    tui::restore()?;

    app_result
}

impl App {
   /// runs the application's main loop until the user quits
   pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
    while !self.exit {
        terminal.draw(|frame| self.render_frame(frame))?;
        self.handle_events()?;
    }
    Ok(())
   }

   fn render_frame(&self, frame: &mut Frame) {
    todo!()
   }

   fn handle_events(&mut self) -> io::Result<()> {
    todo!()
   }
}