use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::Rect,
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::Frame;

use super::{Scene, SharedData};

pub struct StatisticsScene {
    choosen_text_id: i32,
    texts: [&'static str; 3],
}
impl StatisticsScene {
    pub fn new() -> Self {
        StatisticsScene {
            choosen_text_id: 0,
            texts: ["Change nickname", "Do nothing", "Exit"],
        }
    }

    // fn choosen_text(&self) -> &&str {
    //     let result = self.texts.get(self.choosen_text_id as usize);
    //     match result {
    //         Some(value) => value,
    //         None => &"None",
    //     }
    // }
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

        for text_id in 0..3 {
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
            KeyCode::Enter => {
                if self.choosen_text_id == 0 {
                    data.current_scene = 0
                }
                if self.choosen_text_id == 2 {
                    data.terminate = true;
                }
            }
            _ => (),
        }
    }

    fn update(&mut self, data: &mut SharedData) {
        data.tick += 1;
    }
}
