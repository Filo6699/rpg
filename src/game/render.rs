use super::classes::{Game, Screen};

impl Game {
    pub fn get_screen_name(&mut self) -> String {
        match self.screen {
            Screen::Battle(_) => "Battle".into(),
            Screen::Menu => "Main menu".into(),
        }
    }

    pub fn render(&mut self) -> String {
        match &mut self.screen {
            Screen::Battle(battle) => {
                let plr = &battle.player;
                let enemy = &battle.enemy;
                let mut data = format!("{} is fighting {}", plr.name, enemy.name);

                // Player's health
                data.push_str(&format!("\n{} hp: {}", plr.name, plr.health));

                // Enemy's health
                data.push_str(&format!("\n{} hp: {}", enemy.name, enemy.health));

                data
            }
            Screen::Menu => "Press q to exit".into(),
        }
    }
}
