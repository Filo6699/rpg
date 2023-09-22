use std::{
    error::Error,
    io::Stdout, ops::ControlFlow, time::Duration
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{widgets::{Paragraph, BorderType}, prelude::{CrosstermBackend, Rect}};

use crate::game::game::Game;
use super::utils::{calculate_layout, render_border_type};

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;


pub fn run(terminal: &mut Terminal) -> Result<()> {
    let mut game = Game::new();
    loop {
        let parag = Paragraph::new(game.tick_test.to_string());
        terminal.draw(|f| ui(f, &parag))?;
        if handle_events()?.is_break() {
            return Ok(());
        }
        game.tick_test += 1;
    }
}

fn handle_events() -> Result<ControlFlow<()>> {
  if event::poll(Duration::from_millis(100))? {
      if let Event::Key(key) = event::read()? {
          if let KeyCode::Char('q') = key.code {
              return Ok(ControlFlow::Break(()));
          }
      }
  }
  Ok(ControlFlow::Continue(()))
}

fn ui(frame: &mut Frame, paragraph: &Paragraph) {
  let (_, _layout) = calculate_layout(frame.size());

  let area = Rect {
    x: 0,
    y: 0,
    width: frame.size().width,
    height: frame.size().height
  };
  let title = String::from("Ticks");
  render_border_type(&paragraph, &title, BorderType::Rounded, frame, area);
}