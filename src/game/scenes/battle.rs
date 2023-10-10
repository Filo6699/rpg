use crossterm::event::KeyEvent;
use ratatui::{
    prelude::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::game::battle::{Battle, BattleWinner};

use super::{stats::StatisticsScene, Scene, SharedData};

const SCENE_ID: i32 = 2;

pub struct BattleScene {
    battle: Battle,
    tick: i32,
}
impl BattleScene {
    pub fn new(data: &str) -> Self {
        let bat: Battle = serde_json::from_str(data).unwrap();
        BattleScene {
            battle: bat,
            tick: 0,
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

    fn handle_input(&mut self, _: KeyEvent, data: &mut SharedData) {
        self.tick += 1;
        self.battle.tick();
        if let Some(winner) = self.battle.get_winner() {
            let xp_gain: u128;
            let coins_gain: u128;

            match winner {
                BattleWinner::Player(_) => {
                    xp_gain = 120;
                    coins_gain = 15;
                }
                BattleWinner::Enemy(_) => {
                    xp_gain = 50;
                    coins_gain = 0;
                }
            }

            data.player_data.add_xp(xp_gain);
            data.player_data.add_coins(coins_gain);

            data.current_scene = StatisticsScene::scene_id();
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
