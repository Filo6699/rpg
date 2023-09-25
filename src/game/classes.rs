use crossterm::event::{KeyCode, KeyEvent};

use super::utils::{ENEMY_WON, PLAYER_WON};

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

    pub fn default() -> Entity {
        Entity {
            health: 3,
            damage: 10,
            name: "Entity".into(),
        }
    }
}

pub struct Battle {
    pub player: Entity,
    pub enemy: Entity,
    player_turn: bool,
    pub winner: Option<i32>,
}

impl Battle {
    fn new(player: &Player, enemy: &Entity) -> Battle {
        Battle {
            player: player.to_entity(),
            enemy: enemy.clone(),
            player_turn: true,
            winner: None,
        }
    }

    fn tick(&mut self) {
        if self.player_turn {
            if self.enemy.health <= self.player.damage {
                self.winner = Some(PLAYER_WON);
                return;
            }
            self.enemy.health -= self.player.damage
        } else {
            if self.player.health <= self.enemy.damage {
                self.winner = Some(ENEMY_WON);
                return;
            }
            self.player.health -= self.enemy.damage
        }
        self.player_turn = !self.player_turn;
    }

    fn handle_key(&mut self, _: KeyCode) {}
}

pub enum Screen {
    Battle(Battle),
    Menu,
}

pub struct Player {
    level: u32,
    base_health: u32,
    base_damage: u32,
    xp: u64,
    needed_xp: u64,
    name: String,
}

impl Player {
    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_health(&self) -> u32 {
        self.base_health
    }

    pub fn get_damage(&self) -> u32 {
        self.base_damage
    }

    pub fn get_xp(&self) -> u64 {
        self.xp
    }

    pub fn get_nxp(&self) -> u64 {
        self.needed_xp
    }

    fn calculate_needed_xp(level: u32) -> u64 {
        (level.pow(2)*100).into()
    }

    fn stats_from_level(level: u32) -> (u32, u32) {
        ( level * 30 + 100, level * 2 + 10)
    }

    pub fn add_xp(&mut self, xp: u64) {
        self.xp += xp;
        while self.xp >= self.needed_xp {
            self.level += 1;
            ( self.base_health, self.base_damage ) = Player::stats_from_level(self.level);
            self.needed_xp = Player::calculate_needed_xp(self.level);
        };
    }

    pub fn default() -> Player {
        let ( health, damage ) = Player::stats_from_level(1);
        Player {
            level: 1,
            base_health: health,
            base_damage: damage,
            name: "Player".into(),
            xp: 0,
            needed_xp: Player::calculate_needed_xp(1),
        }
    }

    pub fn to_entity(&self) -> Entity {
        Entity::new(
            self.base_health,
            self.base_damage,
            &self.name,
        )
    }
}

pub struct Game {
    pub player: Player,
    message_queue: Vec<String>,
    pub screen: Screen,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::default(),
            message_queue: vec![],
            screen: Screen::Menu,
        }
    }

    pub fn get_message(&mut self) -> Option<&String> {
        self.message_queue.get(0)
    }

    fn consume_message(&mut self) {
        self.message_queue.remove(0);
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
                if battle.winner.is_none() {
                    return;
                }
                let winner = battle.winner.unwrap();
                let msg = match winner {
                    PLAYER_WON => format!("You won the battle against {}!", battle.enemy.name),
                    ENEMY_WON => format!("You died in a battle against {}.", battle.enemy.name),
                    _ => panic!("Wrong value of battle.winner: {}", winner),
                };
                self.player.add_xp(5000);
                self.message_queue.push(msg);
                self.screen = Screen::Menu;
            }
            Screen::Menu => {
                if KeyCode::Char('t') == event.code {
                    let enemy = Entity::default();
                    // let enemy_name = String::from("Bebra");
                    // let enemy = Entity::new(80, 12, &enemy_name);
                    self.screen = Screen::Battle(Battle::new(&self.player, &enemy));
                }
            }
        }
    }
}
