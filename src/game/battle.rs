use crossterm::event::KeyCode;

use super::{player::Player, utils::MessageQueue};

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
            health: 100,
            damage: 10,
            name: "Dummy".into(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

type LeftHp = u32;

#[derive(Copy, Clone)]
pub enum BattleWinner {
    Player(LeftHp),
    Enemy(LeftHp),
}

pub struct BattleScreen {
    pub player: Entity,
    pub enemy: Entity,
    player_turn: bool,
    winner: Option<BattleWinner>,

    msg_queue: Option<MessageQueue>,
}

impl BattleScreen {
    pub fn new(player: &Player, enemy: &Entity) -> BattleScreen {
        BattleScreen {
            player: player.to_entity(),
            enemy: enemy.clone(),
            player_turn: true,
            winner: None,

            msg_queue: None,
        }
    }

    pub fn is_players_turn(&self) -> bool {
        self.player_turn
    }

    pub fn get_winner(&mut self) -> Option<BattleWinner> {
        self.winner
    }

    pub fn set_message_queue(&mut self, message_queue: MessageQueue) {
        self.msg_queue = Some(message_queue);
    }

    // fn battle_finished(&mut self) {
    //     if let Some(q) = &mut self.msg_queue {
    //         let mut queue = q.lock().unwrap();
    //         let msg = match self.winner.unwrap() {
    //             BattleWinner::Player(_) => {
    //                 format!("You won the battle against {}!", self.enemy.name)
    //             }
    //             BattleWinner::Enemy(_) => {
    //                 format!("You died in a battle against {}.", self.enemy.name)
    //             }
    //         };
    //         queue.push(msg);
    //     };
    // }

    pub fn tick(&mut self) {
        if self.player_turn {
            if self.enemy.health <= self.player.damage {
                self.winner = Some(BattleWinner::Player(self.player.health));
                // self.battle_finished();
                return;
            }
            self.enemy.health -= self.player.damage
        } else {
            if self.player.health <= self.enemy.damage {
                self.winner = Some(BattleWinner::Enemy(self.enemy.health));
                // self.battle_finished();
                return;
            }
            self.player.health -= self.enemy.damage
        }
        self.player_turn = !self.player_turn;
    }

    pub fn handle_key(&mut self, _: KeyCode) {}
}

pub struct GainsScreen {
    xp_gain: u64,
    coins_gain: u64,
    left_hp: u32,
    en_name: String,
    player_won: bool,
}

impl GainsScreen {
    pub fn new(xp: u64, coins: u64, hp: u32, enemy_name: String, pl_won: bool) -> GainsScreen {
        GainsScreen {
            xp_gain: xp,
            coins_gain: coins,
            left_hp: hp,
            en_name: enemy_name,
            player_won: pl_won,
        }
    }

    pub fn player_won(&self) -> bool {
        self.player_won
    }

    pub fn get_enemy_name(&self) -> &str {
        &self.en_name
    }

    pub fn get_xp(&self) -> u64 {
        self.xp_gain
    }

    pub fn get_coins(&self) -> u64 {
        self.coins_gain
    }

    pub fn get_left_hp(&self) -> u32 {
        self.left_hp
    }
}
