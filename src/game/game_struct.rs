use crossterm::event::{KeyCode, KeyEvent};
use std::sync::{Arc, Mutex};

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
    fn new(player: &Player, enemy: &Entity) -> Battle {
        Battle {
            player: player.to_entity(),
            enemy: enemy.clone(),
            player_turn: true,
            winner: None,

            msg_queue: None,
        }
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

    fn tick(&mut self) {
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

    fn handle_key(&mut self, _: KeyCode) {}
}

pub enum Screen {
    Battle(Battle),
    Stats,
}

pub struct Player {
    level: u32,
    base_health: u32,
    base_damage: u32,
    xp: u64,
    needed_xp: u64,
    name: String,

    msg_queue: Option<MessageQueue>,
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
        (level.pow(2) * 100).into()
    }

    fn stats_from_level(level: u32) -> (u32, u32) {
        (level * 30 + 100, level * 2 + 10)
    }

    pub fn add_xp(&mut self, xp: u64) {
        self.xp += xp;
        let prev_level = self.level;
        while self.xp >= self.needed_xp {
            self.level += 1;
            (self.base_health, self.base_damage) = Player::stats_from_level(self.level);
            self.needed_xp = Player::calculate_needed_xp(self.level);
        }
        if self.level != prev_level {
            if let Some(q) = &mut self.msg_queue {
                let mut queue = q.lock().unwrap();
                queue.push("Level up!".into());
            }
        }
    }

    pub fn default() -> Player {
        let (health, damage) = Player::stats_from_level(1);
        Player {
            level: 1,
            base_health: health,
            base_damage: damage,
            name: "Player".into(),
            xp: 0,
            needed_xp: Player::calculate_needed_xp(1),

            msg_queue: None,
        }
    }

    pub fn set_message_queue(&mut self, msg_queue: MessageQueue) {
        self.msg_queue = Some(msg_queue);
    }

    pub fn to_entity(&self) -> Entity {
        Entity::new(self.base_health, self.base_damage, &self.name)
    }
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

    // pub fn add_message(&mut self, msg: String) {
    //     let re = Arc::clone(&self.message_queue);
    //     let mut queue = re.lock().unwrap();
    //     queue.push(msg);
    // }

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
                if battle.winner.is_none() {
                    return;
                }
                let xp_gain = match battle.winner.unwrap() {
                    BattleWinner::Player => 120,
                    BattleWinner::Enemy => 50,
                };
                self.player.add_xp(xp_gain);
                self.screen = Screen::Stats;
            }
            Screen::Stats => {
                if KeyCode::Char('t') == event.code {
                    let enemy_name = String::from("Bebra");
                    let enemy = Entity::new(100, 30, &enemy_name);
                    let mut battle = Battle::new(&self.player, &enemy);
                    battle.set_message_queue(self.create_msgq_reference());
                    self.screen = Screen::Battle(battle);
                }
            }
        }
    }
}
