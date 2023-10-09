use crossterm::event::KeyEvent;

use super::player::Player;
use crate::Frame;

pub mod stats;

pub struct SharedData {
    player_data: Player,
    tick: u32,
}

impl SharedData {
    pub fn new(player: Player) -> Self {
        SharedData {
            player_data: player,
            tick: 0,
        }
    }
}

pub trait Scene {
    fn render(&self, frame: &mut Frame, data: &SharedData);
    fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData);
    fn update(&mut self, data: &mut SharedData);
}
