use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Clone, Serialize, Deserialize)]
pub struct Entity {
    health: u128,
    damage: u128,
    name: String,
}

#[allow(unused)]
impl Entity {
    pub fn new(hp: u128, dmg: u128, name: &str) -> Entity {
        Entity {
            health: hp,
            damage: dmg,
            name: name.into(),
        }
    }

    pub fn default() -> Entity {
        Entity {
            health: 100,
            damage: 10,
            name: "Dummy".into(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_health(&self) -> u128 {
        self.health
    }
}

type LeftHp = u128;

#[derive(Copy, Clone)]
pub enum BattleWinner {
    Player(LeftHp),
    Enemy(LeftHp),
}

#[derive(Serialize, Deserialize)]
pub struct Battle {
    pub player: Entity,
    pub enemy: Entity,

    #[serde(skip_serializing, skip_deserializing)]
    player_turn: bool,
    #[serde(skip_serializing, skip_deserializing)]
    winner: Option<BattleWinner>,
    // #[serde(skip_serializing, skip_deserializing)]
    // msg_queue: Option<MessageQueue>,
}

impl Battle {
    pub fn new(player: &Player, enemy: &Entity) -> Battle {
        Battle {
            player: player.to_entity(),
            enemy: enemy.clone(),
            player_turn: true,
            winner: None,
            // msg_queue: None,
        }
    }

    pub fn is_players_turn(&self) -> bool {
        self.player_turn
    }

    pub fn get_winner(&mut self) -> Option<BattleWinner> {
        self.winner
    }

    // pub fn set_message_queue(&mut self, message_queue: MessageQueue) {
    //     self.msg_queue = Some(message_queue);
    // }

    pub fn tick(&mut self) {
        if self.player_turn {
            if self.enemy.health <= self.player.damage {
                self.winner = Some(BattleWinner::Player(self.player.health));
                return;
            }
            self.enemy.health -= self.player.damage
        } else {
            if self.player.health <= self.enemy.damage {
                self.winner = Some(BattleWinner::Enemy(self.enemy.health));
                return;
            }
            self.player.health -= self.enemy.damage
        }
        self.player_turn = !self.player_turn;
    }
}
