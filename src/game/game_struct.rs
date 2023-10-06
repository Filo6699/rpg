use crossterm::event::{KeyCode, KeyEvent};
use std::sync::{Arc, Mutex};

use super::{
    battle::{BattleScreen, BattleWinner, Entity, GainsScreen},
    player::Player,
    utils::MessageQueue,
};

pub enum Screen {
    Stats,
    Battle(BattleScreen),
    Gains(GainsScreen),
}

pub struct Game {
    pub player: Player,
    message_queue: MessageQueue,
    pub screen: Screen,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            player: Player::default(),
            message_queue: Arc::new(Mutex::new(vec![])),
            screen: Screen::Stats,
        };
        game.player.set_message_queue(game.create_msgq_reference());
        game
    }

    fn create_msgq_reference(&self) -> MessageQueue {
        Arc::clone(&self.message_queue)
    }

    pub fn get_message(&mut self) -> Option<String> {
        let queue = self.message_queue.lock().unwrap();
        queue.get(0).cloned()
    }

    fn consume_message(&mut self) {
        let mut queue = self.message_queue.lock().unwrap();
        queue.remove(0);
    }

    pub fn handle_key_press(&mut self, event: KeyEvent) {
        if self.get_message().is_some() {
            if KeyCode::Enter == event.code {
                self.consume_message();
            }
            return;
        }
        match &mut self.screen {
            Screen::Battle(battle) => {
                battle.handle_key(event.code);
                battle.tick();
                if battle.get_winner().is_none() {
                    return;
                }
                let xp_gain: u128;
                let hp_left: u128;
                let player_won: bool;
                match battle.get_winner().unwrap() {
                    BattleWinner::Player(bhp_left) => {
                        xp_gain = 340;
                        hp_left = bhp_left;
                        player_won = true;
                    }
                    BattleWinner::Enemy(bhp_left) => {
                        xp_gain = 120;
                        hp_left = bhp_left;
                        player_won = false;
                    }
                };
                let mut coins_gain: u128 = 0;
                if let BattleWinner::Player(_) = battle.get_winner().unwrap() {
                    coins_gain = 15;
                }
                self.player.add_coins(coins_gain);
                self.player.add_xp(xp_gain);

                self.screen = Screen::Gains(GainsScreen::new(
                    xp_gain,
                    coins_gain,
                    hp_left,
                    battle.enemy.name.clone(),
                    player_won,
                ));
            }
            Screen::Stats => {
                if KeyCode::Char('t') == event.code {
                    let enemy_name = String::from("Bebra");
                    let enemy = Entity::new(100, 30, &enemy_name);
                    let mut battle = BattleScreen::new(&self.player, &enemy);
                    battle.set_message_queue(self.create_msgq_reference());
                    self.screen = Screen::Battle(battle);
                }
            }
            Screen::Gains(_) => {
                if KeyCode::Enter == event.code {
                    self.screen = Screen::Stats;
                }
            }
        }
    }
}
