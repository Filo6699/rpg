use crossterm::event::KeyEvent;
use ratatui::{prelude::Rect, style::Stylize, widgets::Paragraph};

use self::{battle::BattleScene, stats::StatisticsScene, username::UsernameScene};

use super::{message_queue::MessageQueue, player::Player};
use crate::Frame;

pub mod battle;
pub mod stats;
pub mod username;

pub struct SharedData {
    player_data: Player,
    tick: u32,
    current_scene: i32,
    scene_data_transfer: Option<String>,
    terminate: bool,
}

impl SharedData {
    pub fn new(player: Player, scene_id: i32) -> Self {
        SharedData {
            player_data: player,
            tick: 0,
            current_scene: scene_id,
            scene_data_transfer: None,
            terminate: false,
        }
    }

    pub fn is_terminating(&self) -> bool {
        self.terminate
    }
}

pub trait Scene {
    fn scene_id(&self) -> i32;
    fn set_message_queue(&mut self, queue: MessageQueue);

    fn render(&self, frame: &mut Frame, data: &SharedData);
    fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData);
    fn update(&mut self, data: &mut SharedData);
}

pub struct SceneManager {
    current_scene: Box<dyn Scene>,
    message_queue: MessageQueue,
}
impl SceneManager {
    pub fn new(scene: impl Scene + 'static) -> Self {
        let mut manager = SceneManager {
            current_scene: Box::new(scene),
            message_queue: MessageQueue::new(),
        };
        manager
            .current_scene
            .set_message_queue(manager.message_queue.clone());
        manager
    }

    pub fn get_message_queue(&self) -> MessageQueue {
        self.message_queue.clone()
    }

    fn render_message(&mut self, frame: &mut Frame, msg: String) {
        let length = msg.len() as u16;
        let p = Paragraph::new(msg.clone()).on_white().black().bold();
        let area = Rect {
            x: frame.size().width / 2 - length / 2,
            y: frame.size().height - 1,
            width: length,
            height: 1,
        };
        frame.render_widget(p, area);
    }

    pub fn render(&mut self, frame: &mut Frame, data: &SharedData) {
        self.current_scene.render(frame, data);

        if let Some(msg) = self.message_queue.get_message() {
            self.render_message(frame, msg)
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData) {
        let scene = &mut self.current_scene;
        scene.handle_input(key, data);
    }

    pub fn update(&mut self, data: &mut SharedData) {
        self.current_scene.update(data);

        if self.current_scene.scene_id() != data.current_scene {
            match data.current_scene {
                _id if _id == UsernameScene::scene_id() => {
                    self.current_scene = Box::new(UsernameScene::new())
                }
                _id if _id == StatisticsScene::scene_id() => {
                    self.current_scene = Box::new(StatisticsScene::new())
                }
                _id if _id == BattleScene::scene_id() => {
                    self.current_scene = {
                        if let Some(transfer) = &data.scene_data_transfer {
                            Box::new(BattleScene::new(transfer))
                        } else {
                            panic!("No data provided to create battle screen");
                        }
                    };
                    data.scene_data_transfer = None;
                }
                _ => panic!("Not valid scene_id"),
            }

            self.current_scene
                .set_message_queue(self.message_queue.clone())
        }
    }
}
