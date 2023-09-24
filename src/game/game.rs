use crossterm::event::{KeyCode, KeyEvent};

use super::utils::{ENEMY_WON, PLAYER_WON};

#[derive(Clone)]
struct Entity {
    health: u32,
    damage: u32,
    name: String,
}

impl Entity {
    pub fn new(hp: u32, dmg: u32, name: &String) -> Entity {
        Entity {
            health: hp,
            damage: dmg,
            name: name.clone(),
        }
    }

    pub fn default() -> Entity {
        Entity {
            health: 100,
            damage: 10,
            name: "Player".into(),
        }
    }
}

struct Battle {
    player: Entity,
    enemy: Entity,
    player_turn: bool,
    winner: Option<i32>,
}

impl Battle {
    fn new(player: &Entity, enemy: &Entity) -> Battle {
        Battle {
            player: player.clone(),
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

enum Screen {
    Battle(Battle),
    None
}

pub struct Game {
    player: Entity,
    message_queue: Vec<String>,
    screen: Screen,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Entity::default(),
            message_queue: vec![],
            screen: Screen::None,
        }
    }

    pub fn get_message(&mut self) -> Option<&String> {
        self.message_queue.get(0)
    }

    fn consume_message(&mut self) {
        self.message_queue.remove(0);
    }

    fn battle_text(battle: &mut Battle) -> String {
        let plr = &battle.player;
        let enemy = &battle.enemy;

        let mut data = format!("{} is fighting {}", plr.name, enemy.name);

        // Player's health
        data.push_str(&format!("\n{} hp: {}", plr.name, plr.health));

        // Enemy's health
        data.push_str(&format!("\n{} hp: {}", enemy.name, enemy.health));

        return data;
    }

    pub fn get_text(&mut self) -> Option<String> {
        return match &mut self.screen {
            Screen::Battle(b) => {
                Some(Game::battle_text(b))
            },
            Screen::None => None,
        };
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
                let msg: String;
                let winner = battle.winner.unwrap();
                match winner {
                    PLAYER_WON => msg = format!("You won the battle against {}!", battle.enemy.name),
                    ENEMY_WON => msg = format!("You died in a battle against {}.", battle.enemy.name),
                    _ => panic!("Wrong value of battle.winner: {}", winner),
                }
                self.message_queue.push(msg);
                self.screen = Screen::None;
                return;
            },
            Screen::None => {
                if KeyCode::Char('t') == event.code {
                    let enemy_name = String::from("Bebra");
                    let enemy = Entity::new(80, 12, &enemy_name);
                    self.screen = Screen::Battle(Battle::new(&self.player, &enemy));
                }
            }
        }
    }
}
