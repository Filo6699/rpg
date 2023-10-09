use crossterm::event::KeyEvent;

use self::{stats::StatisticsScene, username::UsernameScene};

use super::player::Player;
use crate::Frame;

pub mod stats;
pub mod username;

pub struct SharedData {
    player_data: Player,
    tick: u32,
    current_scene: i32,
    terminate: bool,
}

impl SharedData {
    pub fn new(player: Player, scene_id: i32) -> Self {
        SharedData {
            player_data: player,
            tick: 0,
            current_scene: scene_id,
            terminate: false,
        }
    }

    pub fn is_terminating(&self) -> bool {
        self.terminate
    }
}

pub trait Scene {
    fn scene_id(&self) -> i32;

    fn render(&self, frame: &mut Frame, data: &SharedData);
    fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData);
    fn update(&mut self, data: &mut SharedData);
}

pub struct SceneManager {
    current_scene: Box<dyn Scene>,
}
impl SceneManager {
    pub fn new(scene: impl Scene + 'static) -> Self {
        SceneManager {
            current_scene: Box::new(scene),
        }
    }

    pub fn render(&self, frame: &mut Frame, data: &SharedData) {
        self.current_scene.render(frame, data);
    }
    pub fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData) {
        let scene = &mut self.current_scene;
        scene.handle_input(key, data);
    }
    pub fn update(&mut self, data: &mut SharedData) {
        self.current_scene.update(data);

        if self.current_scene.scene_id() != data.current_scene {
            match data.current_scene {
                0 => self.current_scene = Box::new(UsernameScene::new()),
                1 => self.current_scene = Box::new(StatisticsScene::new()),
                _ => panic!("Not valid scene_id"),
            }
        }
    }
}
