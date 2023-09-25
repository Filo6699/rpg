use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{CrosstermBackend, Rect},
    widgets::Paragraph,
};
use std::{error::Error, io::Stdout, ops::ControlFlow, time::Duration};

use super::super::game::classes::Game;
use super::utils::{render_border_type, render_message};

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn run(terminal: &mut Terminal) -> Result<()> {
    let mut game = Game::new();
    loop {
        if handle_events(&mut game)?.is_break() {
            return Ok(());
        }

        terminal.draw(|f| ui(f, &mut game))?;
    }
}

fn ui(frame: &mut Frame, game: &mut Game) {
    let area = Rect {
        x: 0,
        y: 0,
        width: frame.size().width,
        height: frame.size().height,
    };
    let tick_text = game.render();

    let title = game.get_screen_name();
    let paragraph = Paragraph::new(tick_text);
    render_border_type(&paragraph, &title, frame, area);

    if let Some(message) = game.get_message() {
        render_message(message, frame);
    }
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
