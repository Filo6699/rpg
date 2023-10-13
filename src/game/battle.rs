use super::{equipment::Equipment, player::Player};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Entity {
    health: u128,
    damage: u128,
    name: String,
    equipment: Equipment,
}

impl Entity {
    pub fn new(hp: u128, dmg: u128, name: &str, equipment: Option<Equipment>) -> Entity {
        let equip = match equipment {
            Some(e) => e,
            None => Equipment::default(),
        };
        Entity {
            health: hp,
            damage: dmg,
            name: name.into(),
            equipment: equip,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_health(&self) -> u128 {
        self.health
    }
}

impl Default for Entity {
    fn default() -> Entity {
        Entity {
            health: 100,
            damage: 10,
            name: "Dummy".into(),
            equipment: Equipment::default(),
        }
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
}

impl Battle {
    pub fn new(player: &Player, enemy: &Entity) -> Battle {
        Battle {
            player: player.to_entity(),
            enemy: enemy.clone(),
            player_turn: true,
            winner: None,
        }
    }

    pub fn is_players_turn(&self) -> bool {
        self.player_turn
    }

    pub fn get_winner(&mut self) -> Option<BattleWinner> {
        self.winner
    }

    pub fn tick(&mut self) {
        if self.player_turn {
            let mut damage = self.player.damage;
            match &self.player.equipment.sword {
                Some(sword) => damage += sword.properties.damage,
                None => (),
            }
            let enemy_defence = match &self.enemy.equipment.shield {
                Some(shield) => shield.properties.defence,
                None => 0,
            };
            if damage > enemy_defence {
                damage -= enemy_defence;
                if self.enemy.health <= damage {
                    self.winner = Some(BattleWinner::Player(self.player.health));
                    return;
                }
                self.enemy.health -= damage;
            }
        } else {
            let mut damage = self.enemy.damage;
            match &self.enemy.equipment.sword {
                Some(sword) => damage += sword.properties.damage,
                None => (),
            }
            let player_defence = match &self.player.equipment.shield {
                Some(shield) => shield.properties.defence,
                None => 0,
            };
            if damage > player_defence {
                damage -= player_defence;
                if self.player.health <= damage {
                    self.winner = Some(BattleWinner::Enemy(self.enemy.health));
                    return;
                }
                self.player.health -= damage;
            }
        }
        self.player_turn = !self.player_turn;
    }
}
