use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Alignment, Rect},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph,
    },
};

use crate::Frame;

use super::{Scene, SharedData};

pub struct StatisticsScene;
impl StatisticsScene {
    pub fn new() -> Self {
        StatisticsScene
    }
}

impl Scene for StatisticsScene {
    fn render(&self, frame: &mut Frame, data: &SharedData) {
        let message = format!("Frame #{}", data.tick);
        let half = (std::cmp::max(message.len() / 2, 5)) as u16;
        let area = Rect {
            x: frame.size().width / 2 - half - 1,
            y: frame.size().height / 2 - 2,
            width: std::cmp::max(message.len() as u16 + 2, 15),
            height: 4,
        };
        let paragraph = Paragraph::new(message.clone());
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(Title::from("[Message]").alignment(Alignment::Center))
            .title(
                Title::from("[Press Enter]")
                    .position(Position::Bottom)
                    .alignment(Alignment::Center),
            );
        frame.render_widget(paragraph.clone().block(block), area);
    }

    fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData) {}

    fn update(&mut self, data: &mut SharedData) {
        data.tick += 1;
    }
}
