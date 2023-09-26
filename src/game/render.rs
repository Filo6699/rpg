use super::game_class::{Game, Screen};

impl Game {
    pub fn get_screen_name(&mut self) -> String {
        match self.screen {
            Screen::Battle(_) => "[Battle]".into(),
            Screen::Stats => "[Main menu]".into(),
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
            Screen::Stats => format!(
                "{}\nHP: {}\nDamage: {}\nLevel {}: {}/{}\n\nPress q to exit",
                self.player.get_name(),
                self.player.get_health(),
                self.player.get_damage(),
                self.player.get_level(),
                self.player.get_xp(),
                self.player.get_nxp(),
            ),
        }
    }
}
