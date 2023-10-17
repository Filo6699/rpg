mod game;
use std::error::Error;
use std::io::{stdout, Stdout};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crossterm::event::{
    self, Event, KeyEventKind, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use game::scenes::stats::StatisticsScene;
use game::scenes::{Scene, SceneManager};
use game::ui::{restore_terminal, setup_terminal};
use game::{player::Player, scenes::SharedData};
use ratatui::prelude::CrosstermBackend;

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn run(terminal: &mut Terminal) -> Result<()> {
    let mut stdout = stdout();
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                .union(KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES)
        ),
    )?;
    let starting_scene = StatisticsScene::new();
    let scene_id = starting_scene.scene_id();
    let mut scene_manager = SceneManager::new(starting_scene);

    let mut player = Player::default();
    player.set_message_queue(scene_manager.get_message_queue());
    let shared_data = Arc::new(Mutex::new(SharedData::new(player, scene_id)));

    loop {
        if event::poll(Duration::from_millis(1000 / 60))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Repeat {
                    scene_manager.handle_input(key, &mut shared_data.lock().unwrap());
                }
                let data: &SharedData = &shared_data.lock().unwrap();
                if data.is_terminating() {
                    return Ok(());
                }
            }
        }
        scene_manager.update(&mut shared_data.lock().unwrap());
        let _ = terminal.draw(|f| scene_manager.render(f, &shared_data.lock().unwrap()));
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
