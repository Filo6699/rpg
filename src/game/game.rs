use crossterm::event::{KeyEvent, KeyCode};

pub struct Game {
    pub tick_test: u32
}

impl Game {
    pub fn new() -> Game {
        Game {
            tick_test: 0
        }
    }

    pub fn update(&mut self) {
        self.tick_test += 1;
    }

    pub fn handle_key_press(&mut self, event: KeyEvent) {
        if KeyCode::Char('t') == event.code {
            self.tick_test = 0;
        }
    }
}