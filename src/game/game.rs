use crossterm::event::{KeyEvent, KeyCode};

#[derive(Clone)]
struct Entity {
    pub health: u32,
    damage: u32,
    name: String
}

impl Entity {
    pub fn new(hp: u32, dmg: u32, name: &String) -> Entity {
        Entity {
            health: hp,
            damage: dmg, 
            name: name.clone()
        }
    }

    pub fn damage(mut self, damage_value: u32) {
        self.health -= damage_value;
    }
}

struct Battle {
    player: Entity,
    enemy: Entity,
    player_turn: bool,
}

impl Battle {
    fn new(player: &Entity, enemy: &Entity) -> Battle {
        Battle {
            player: player.clone(),
            enemy: enemy.clone(),
            player_turn: true
        }
    }

    fn tick(self) {
        if self.player_turn {
            let attack_side = self.player;
            let mut def_side = self.enemy;

            def_side.health -= attack_side.damage;
        } else {
            let attack_side = self.enemy;
            let mut def_side = self.player;

            def_side.health -= attack_side.damage;
        }
    }
}

pub struct Game {
    player: Entity,
    battle: Option<Battle>,
}

impl Game {
    pub fn new() -> Game {
        let player_name = String::from("Player");
        Game {
            player: Entity::new(100, 10, &player_name),
            battle: None
        }
    }

    pub fn get_text(self) -> String {
        if self.battle.is_some() {
            let battle = &self.battle.unwrap();
            let plr = &battle.player;
            let enemy = &battle.enemy;
            
            let mut data = format!("{} is fighting {}", plr.name, enemy.name);

            // Player's health
            data.push_str(&format!("{} hp: {}", plr.name, plr.health));

            // Enemy's health
            data.push_str(&format!("{} hp: {}", enemy.name, enemy.health));

            return data
        } else {
            return String::from("");
        }
    }

    // pub fn update(self) {
        
    // }

    pub fn handle_key_press(&mut self, event: KeyEvent) {
        if KeyCode::Char('t') == event.code {
            if self.battle.is_none() {
                let enemy_name = String::from("Bebra");
                let enemy = Entity::new(50, 5, &enemy_name);
                self.battle = Some(Battle::new(&self.player, &enemy));
            }
        }
        // if KeyCode::Char('a') == event.code {
        //     if self.battle.is_some() {
        //         let battle = self.battle.unwrap();
        //         battle.tick();
        //     }
        // }
    }
}