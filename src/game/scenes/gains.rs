use super::{stats::StatisticsScene, Scene, SharedData};
use crate::game::message_queue::MessageQueue;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};
use serde::{Deserialize, Serialize};

const SCENE_ID: i32 = 4;

#[derive(Serialize, Deserialize)]
pub struct Gains {
    pub player_won: bool,
    pub enemy_name: String,
    pub left_hp: u128,

    pub coins: u128,
    pub xp: u128,
}

pub struct GainsScene {
    gains: Gains,
    message_queue: MessageQueue,
}
impl GainsScene {
    pub fn new(data: &str) -> Self {
        let g: Gains = serde_json::from_str(data).unwrap();
        GainsScene {
            gains: g,
            message_queue: MessageQueue::default(),
        }
    }

    pub fn scene_id() -> i32 {
        SCENE_ID
    }
}

impl Scene for GainsScene {
    fn scene_id(&self) -> i32 {
        SCENE_ID
    }

    fn set_message_queue(&mut self, queue: crate::game::message_queue::MessageQueue) {
        self.message_queue = queue;
    }

    fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData) {
        if let KeyCode::Enter = key.code {
            data.current_scene = StatisticsScene::scene_id()
        }
    }

    fn render(&self, frame: &mut crate::Frame, _: &SharedData) {
        let mut lines: Vec<Line<'_>> = vec![];
        let empty = Line::from("");

        lines.push(Line::from("Battle gains".bold()));
        if self.gains.player_won {
            lines.push(Line::from(vec![
                Span::raw("You won a battle against "),
                Span::styled(&self.gains.enemy_name, Style::default().bold()),
            ]));
            lines.push(Line::from(vec![
                Span::raw("You had "),
                Span::styled(
                    self.gains.left_hp.to_string(),
                    Style::default().bold().light_red(),
                ),
                Span::raw(" hp left."),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::raw("You lost a battle against "),
                Span::styled(&self.gains.enemy_name, Style::default().bold()),
            ]));
            lines.push(Line::from(vec![
                Span::raw("Enemy had "),
                Span::styled(
                    self.gains.left_hp.to_string(),
                    Style::default().bold().light_red(),
                ),
                Span::raw(" hp left."),
            ]));
        }
        lines.push(empty.clone());
        if self.gains.coins > 0 {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("+{}", self.gains.coins),
                    Style::default().bold().light_green(),
                ),
                Span::styled(" coins", Style::default().bold().light_yellow()),
            ]));
        }
        if self.gains.xp > 0 {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("+{}", self.gains.xp),
                    Style::default().bold().light_green(),
                ),
                Span::styled(" xp", Style::default().bold().light_blue()),
            ]));
        }
        lines.push(empty.clone());
        lines.push(Line::from("Press Enter to continue..."));

        let p = Paragraph::new(lines);
        let area = Rect {
            x: 0,
            y: 0,
            width: frame.size().width - 1,
            height: frame.size().height - 1,
        };
        frame.render_widget(p, area);
    }

    fn update(&mut self, _: &mut SharedData) {}
}
