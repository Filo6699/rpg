use super::{
    gains::{Gains, GainsScene},
    Scene, SharedData,
};
use crate::game::{
    battle::{Battle, BattleWinner},
    message_queue::MessageQueue,
};
use crossterm::event::{KeyEvent, KeyEventKind};
use ratatui::{
    prelude::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

const SCENE_ID: i32 = 2;

pub struct BattleScene {
    battle: Battle,
    message_queue: MessageQueue,
}
impl BattleScene {
    pub fn new(data: &SharedData) -> Self {
        let str_data = if let Some(data) = &data.scene_data_transfer {
            data
        } else {
            panic!("No data provided to create battle screen");
        };
        let bat: Battle = serde_json::from_str(str_data).unwrap();
        BattleScene {
            battle: bat,
            message_queue: MessageQueue::default(),
        }
    }

    pub fn scene_id() -> i32 {
        SCENE_ID
    }
}

impl Scene for BattleScene {
    fn scene_id(&self) -> i32 {
        SCENE_ID
    }

    fn set_message_queue(&mut self, queue: crate::game::message_queue::MessageQueue) {
        self.message_queue = queue;
    }

    fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        self.battle.tick();
        if let Some(winner) = self.battle.get_winner() {
            let xp_gain: u128;
            let coins_gain: u128;
            let won: bool;
            let left_health: u128;
            match winner {
                BattleWinner::Player(_) => {
                    xp_gain = 120;
                    coins_gain = 15;
                    won = true;
                    left_health = self.battle.player.get_health();
                }
                BattleWinner::Enemy(_) => {
                    xp_gain = 50;
                    coins_gain = 0;
                    won = false;
                    left_health = self.battle.enemy.get_health();
                }
            }
            data.player_data.add_xp(xp_gain);
            data.player_data.add_coins(coins_gain);

            let gains = Gains {
                player_won: won,
                enemy_name: self.battle.enemy.get_name().to_string(),
                left_hp: left_health,

                xp: xp_gain,
                coins: coins_gain,
            };
            data.scene_data_transfer = Some(serde_json::to_string(&gains).unwrap());
            data.current_scene = GainsScene::scene_id();
        }
    }

    fn render(&self, frame: &mut crate::Frame, _: &SharedData) {
        let mut lines: Vec<Line<'_>> = vec![];
        let player = &self.battle.player;
        let enemy = &self.battle.enemy;
        let empty = Line::from("");

        lines.push(Line::from(vec![
            player.get_name().bold(),
            Span::raw(if self.battle.is_players_turn() {
                " <"
            } else {
                ""
            }),
        ]));
        lines.push(Line::from(vec![
            Span::raw(player.get_health().to_string()),
            Span::styled(" HP", Style::default().light_red().bold()),
        ]));
        lines.push(empty.clone());
        lines.push(Line::from(vec![
            enemy.get_name().bold(),
            Span::raw(if !self.battle.is_players_turn() {
                " <"
            } else {
                ""
            }),
        ]));
        lines.push(Line::from(vec![
            Span::raw(enemy.get_health().to_string()),
            Span::styled(" HP", Style::default().light_red().bold()),
        ]));

        let p = Paragraph::new(lines);
        let area = Rect {
            x: 0,
            y: 0,
            width: frame.size().width - 1,
            height: frame.size().height - 1,
        };
        frame.render_widget(p, area)
    }

    fn update(&mut self, _: &mut SharedData) {}
}
