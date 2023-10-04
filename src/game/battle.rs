use std::sync::{Arc, Mutex};

use crossterm::event::KeyCode;

use super::game_struct::Player;

type MessageQueue = Arc<Mutex<Vec<String>>>;

#[derive(Clone)]
pub struct Entity {
    pub health: u32,
    damage: u32,
    pub name: String,
}

impl Entity {
    pub fn new(hp: u32, dmg: u32, name: &str) -> Entity {
        Entity {
            health: hp,
            damage: dmg,
            name: name.into(),
        }
    }

    #[allow(dead_code)]
    pub fn default() -> Entity {
        Entity {
            health: 3,
            damage: 10,
            name: "Entity".into(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum BattleWinner {
    Player,
    Enemy,
}

pub struct Battle {
    pub player: Entity,
    pub enemy: Entity,
    player_turn: bool,
    winner: Option<BattleWinner>,

    msg_queue: Option<MessageQueue>,
}

impl Battle {
    pub fn new(player: &Player, enemy: &Entity) -> Battle {
        Battle {
            player: player.to_entity(),
            enemy: enemy.clone(),
            player_turn: true,
            winner: None,

            msg_queue: None,
        }
    }

    pub fn get_winner(&mut self) -> Option<BattleWinner> {
        self.winner
    }

    pub fn set_message_queue(&mut self, message_queue: MessageQueue) {
        self.msg_queue = Some(message_queue);
    }

    fn battle_finished(&mut self) {
        if let Some(q) = &mut self.msg_queue {
            let mut queue = q.lock().unwrap();
            let msg = match self.winner.unwrap() {
                BattleWinner::Player => format!("You won the battle against {}!", self.enemy.name),
                BattleWinner::Enemy => format!("You died in a battle against {}.", self.enemy.name),
            };
            queue.push(msg);
        };
    }

    pub fn tick(&mut self) {
        if self.player_turn {
            if self.enemy.health <= self.player.damage {
                self.winner = Some(BattleWinner::Player);
                self.battle_finished();
                return;
            }
            self.enemy.health -= self.player.damage
        } else {
            if self.player.health <= self.enemy.damage {
                self.winner = Some(BattleWinner::Enemy);
                self.battle_finished();
                return;
            }
            self.player.health -= self.enemy.damage
        }
        self.player_turn = !self.player_turn;
    }

    pub fn handle_key(&mut self, _: KeyCode) {}
}

pub struct Gains {
    xp_gain: u64,
    coins_gain: u64,
}

impl Gains {
    pub fn new(xp: u64, coins: u64) -> Gains {
        Gains {
            xp_gain: xp,
            coins_gain: coins,
        }
    }

    pub fn get_xp(&self) -> u64 {
        self.xp_gain
    }

    pub fn get_coins(&self) -> u64 {
        self.coins_gain
    }
}
