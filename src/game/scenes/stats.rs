use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::Rect,
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    game::utils::{load_save, write_save},
    Frame,
};

use super::{Scene, SharedData};

pub struct StatisticsScene {
    choosen_text_id: i32,
    texts: [&'static str; 4],
    temp: Option<String>,
}
impl StatisticsScene {
    pub fn new() -> Self {
        StatisticsScene {
            choosen_text_id: 0,
            texts: ["Change nickname", "Save", "Load", "Exit"],
            temp: None,
        }
    }
}

impl Scene for StatisticsScene {
    fn scene_id(&self) -> i32 {
        1
    }

    fn render(&self, frame: &mut Frame, data: &SharedData) {
        let message = format!("Hello, {}!", data.player_data.get_name());
        let area = Rect {
            x: 0,
            y: 0,
            width: frame.size().width - 1,
            height: 1,
        };
        let paragraph = Paragraph::new(message.clone());
        frame.render_widget(paragraph, area);

        for text_id in 0..self.texts.len() {
            let mut text = Span::raw(self.texts[text_id]);
            let area = Rect {
                x: 0,
                y: (text_id + 2) as u16,
                width: frame.size().width - 1,
                height: 1,
            };
            if text_id as i32 == self.choosen_text_id {
                text = text.bg(Color::Cyan);
            }
            let paragraph = Paragraph::new(Line::from(text));
            frame.render_widget(paragraph, area)
        }

        if self.temp.is_none() {
            return;
        }
        // let save = load_save(&self.temp.clone().unwrap());
        let paragraph = Paragraph::new(self.temp.clone().unwrap());
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
            KeyCode::Down => {
                if self.choosen_text_id + 1 < (self.texts.len() as i32) {
                    self.choosen_text_id += 1;
                }
            }
            KeyCode::Up => {
                if self.choosen_text_id > 0 {
                    self.choosen_text_id -= 1;
                }
            }
            KeyCode::Enter => match self.choosen_text_id {
                0 => data.current_scene = 0,
                1 => write_save(&data.player_data),
                2 => {
                    if let Some(saved_data) = load_save() {
                        data.player_data = saved_data
                    }
                }
                3 => data.terminate = true,
                _ => (),
            },
            _ => (),
        }
    }

    fn update(&mut self, data: &mut SharedData) {
        data.tick += 1;
    }
}
