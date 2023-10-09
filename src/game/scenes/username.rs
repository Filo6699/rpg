use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::Rect, widgets::Paragraph};

use super::{Scene, SharedData};

pub struct UsernameScene {
    name: String,
}
impl UsernameScene {
    pub fn new() -> Self {
        UsernameScene { name: "".into() }
    }
}

impl Scene for UsernameScene {
    fn scene_id(&self) -> i32 {
        0
    }

    fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData) {
        if let KeyCode::Char(key) = key.code {
            self.name.push(key);
        }
        if key.code == KeyCode::Backspace {
            self.name.pop();
        }
        if key.code == KeyCode::Enter {
            data.player_data.set_name(self.name.clone());
            data.current_scene = 1;
        }
    }
    fn render(&self, frame: &mut crate::Frame, _: &SharedData) {
        let p = Paragraph::new(format!("Enter your new nickname:\n{}", self.name));
        let area = Rect {
            x: 0,
            y: 0,
            width: frame.size().width - 1,
            height: 4,
        };
        frame.render_widget(p, area)
    }
    fn update(&mut self, _: &mut SharedData) {}
}
