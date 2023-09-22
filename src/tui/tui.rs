use std::{
    error::Error,
    io::Stdout, ops::ControlFlow, time::Duration
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{widgets::{Paragraph, BorderType}, prelude::{CrosstermBackend, Rect}};

use crate::game::game::Game;
use super::utils::render_border_type;

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;


pub fn run(terminal: &mut Terminal) -> Result<()> {
    let mut game = Game::new();
    loop {
        if handle_events(&mut game)?.is_break() {
            return Ok(());
        }

        game.update();

        let mut tick_text = game.tick_test.to_string();
        tick_text.push_str("\n\nPress q to exit");
        let title = String::from("Ticks");
        let parag = Paragraph::new(tick_text);
        terminal.draw(|f| ui(f, &title, &parag))?;
    }
}

fn ui(frame: &mut Frame, title: &String, paragraph: &Paragraph) {
  let area = Rect {
    x: 0,
    y: 0,
    width: frame.size().width,
    height: frame.size().height
  };
  render_border_type(&paragraph, &title, BorderType::Rounded, frame, area);
}

fn handle_events(game: &mut Game) -> Result<ControlFlow<()>> {
  if event::poll(Duration::from_millis(100))? {
      if let Event::Key(key) = event::read()? {
          game.handle_key_press(key);
          if let KeyCode::Char('q') = key.code {
              return Ok(ControlFlow::Break(()));
          }
      }
  }
  Ok(ControlFlow::Continue(()))
}