use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    game::{
        battle::{Battle, Entity},
        message_queue::MessageQueue,
        utils::{calculate_bar, load_save, write_save},
    },
    Frame,
};

use super::{battle::BattleScene, Scene, SharedData};

const SCENE_ID: i32 = 1;

pub struct StatisticsScene {
    choosen_text_id: i32,
    texts: [&'static str; 5],
    message_queue: MessageQueue,
}
impl StatisticsScene {
    pub fn new() -> Self {
        StatisticsScene {
            choosen_text_id: 0,
            texts: ["Battle", "Change nickname", "Save", "Load", "Exit"],
            message_queue: MessageQueue::default(),
        }
    }

    pub fn scene_id() -> i32 {
        SCENE_ID
    }
}

impl Scene for StatisticsScene {
    fn scene_id(&self) -> i32 {
        SCENE_ID
    }

    fn set_message_queue(&mut self, queue: crate::game::message_queue::MessageQueue) {
        self.message_queue = queue;
    }

    fn render(&self, frame: &mut Frame, data: &SharedData) {
        let empty = Line::from("");
        let playername = Line::from(format!("Name   | {}", data.player_data.get_name()));
        let health = Line::from(vec![
            Span::raw("Health | "),
            Span::styled(
                data.player_data.get_health().to_string(),
                Style::default().bold().green(),
            ),
        ]);
        let damage = Line::from(vec![
            Span::raw("Damage | "),
            Span::styled(
                data.player_data.get_damage().to_string(),
                Style::default().bold().fg(Color::LightRed),
            ),
        ]);
        let level = Line::from(vec![
            Span::raw("Level  | "),
            Span::styled(
                data.player_data.get_level().to_string(),
                Style::default().bold().fg(Color::Yellow),
            ),
        ]);
        let (filled, missing) =
            calculate_bar(data.player_data.get_xp(), data.player_data.get_nxp(), 10);
        let xpbar = Line::from(vec![
            Span::raw("       | ["),
            Span::styled(filled, Style::default().bold().fg(Color::Gray)),
            Span::styled(missing, Style::default().bold().fg(Color::DarkGray)),
            Span::raw("]  "),
            Span::raw(data.player_data.get_xp().to_string()),
            Span::styled("/", Style::default().fg(Color::DarkGray)),
            Span::raw(data.player_data.get_nxp().to_string()),
            Span::styled(" XP", Style::default().fg(Color::DarkGray)),
        ]);
        let coins = Line::from(vec![
            Span::raw("Coins  | "),
            Span::styled(
                data.player_data.get_coins().to_string(),
                Style::default().bold().fg(Color::LightYellow),
            ),
        ]);
        let mut buttons_spans: Vec<Span<'_>> = vec![];
        for text_id in 0..self.texts.len() {
            if text_id > 0 {
                buttons_spans.push(Span::from("  "));
            }
            let mut style = Style::default();
            if text_id as i32 == self.choosen_text_id {
                style = style.bg(Color::Cyan);
            };
            buttons_spans.push(Span::styled(self.texts[text_id].to_string(), style));
        }

        let buttons = Line::from(buttons_spans);
        let paragraph = Paragraph::new(vec![
            playername,
            empty.clone(),
            health,
            damage,
            coins,
            level,
            xpbar,
            empty.clone(),
            buttons,
        ]);
        let area = Rect {
            x: 0,
            y: 0,
            width: frame.size().width - 1,
            height: frame.size().height - 1,
        };
        frame.render_widget(paragraph, area)
    }

    fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData) {
        match key.code {
            KeyCode::Right => {
                if self.choosen_text_id + 1 < (self.texts.len() as i32) {
                    self.choosen_text_id += 1;
                }
            }
            KeyCode::Left => {
                if self.choosen_text_id > 0 {
                    self.choosen_text_id -= 1;
                }
            }
            KeyCode::Enter => match self.texts[self.choosen_text_id as usize] {
                "Battle" => {
                    let bat: Battle =
                        Battle::new(&data.player_data, &Entity::new(150, 11, "Bebra"));
                    let json_battle = match serde_json::to_string(&bat) {
                        Ok(json) => json,
                        Err(err) => panic!("Wasn't able to parse battle json: {}", err),
                    };
                    data.scene_data_transfer = Some(json_battle);
                    data.current_scene = BattleScene::scene_id()
                }
                "Change nickname" => data.current_scene = 0,
                "Save" => write_save(&data.player_data),
                "Load" => {
                    if let Some(saved_data) = load_save() {
                        let msg_queue = data.player_data.get_message_queue();
                        data.player_data = saved_data;
                        data.player_data.set_message_queue(msg_queue);
                    }
                }
                "Exit" => data.terminate = true,
                _ => (),
            },
            _ => (),
        }
    }

    fn update(&mut self, _: &mut SharedData) {}
}
