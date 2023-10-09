mod game;
use std::error::Error;
use std::io::Stdout;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use game::scenes::stats::StatisticsScene;
use game::scenes::Scene;
use game::ui::{restore_terminal, setup_terminal};
use game::{player::Player, scenes::SharedData};
use ratatui::prelude::CrosstermBackend;

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn run(terminal: &mut Terminal) -> Result<()> {
    let shared_data = Arc::new(Mutex::new(SharedData::new(Player::default())));
    let mut game_scene = StatisticsScene::new();
    loop {
        if event::poll(Duration::from_millis(1000 / 60))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
                game_scene.handle_input(key, &mut shared_data.lock().unwrap());
            }
        }
        game_scene.update(&mut shared_data.lock().unwrap());
        let _ = terminal.draw(|f| game_scene.render(f, &shared_data.lock().unwrap()));
    }
}

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal);
    restore_terminal(terminal)?;

    if let Err(err) = result {
        eprintln!("{err:?}");
    }
    Ok(())
}

// fn main() -> Result<()> {
//     let mut terminal = setup_terminal()?;
//     let result = run(&mut terminal);
//     restore_terminal(terminal)?;

//     if let Err(err) = result {
//         eprintln!("{err:?}");
//     }
//     Ok(())
// }
