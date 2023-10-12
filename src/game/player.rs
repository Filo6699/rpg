use super::{battle::Entity, message_queue::MessageQueue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    level: u128,
    base_health: u128,
    base_damage: u128,
    xp: u128,
    needed_xp: u128,
    coins: u128,
    name: String,

    #[serde(skip_serializing, skip_deserializing)]
    msg_queue: MessageQueue,
}

impl Player {
    pub fn get_level(&self) -> u128 {
        self.level
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_coins(&self) -> u128 {
        self.coins
    }

    pub fn get_health(&self) -> u128 {
        self.base_health
    }

    pub fn get_damage(&self) -> u128 {
        self.base_damage
    }

    pub fn get_xp(&self) -> u128 {
        self.xp
    }

    pub fn get_nxp(&self) -> u128 {
        self.needed_xp
    }

    fn calculate_needed_xp(level: u128) -> u128 {
        level.pow(2) * 40 + 60
    }

    fn stats_from_level(level: u128) -> (u128, u128) {
        (level * 30 + 100, level * 2 + 10)
    }

    pub fn add_coins(&mut self, coins: u128) {
        self.coins += coins;
    }

    pub fn add_xp(&mut self, xp: u128) {
        self.xp += xp;
        let prev_level = self.level;
        while self.xp >= self.needed_xp {
            self.level += 1;
            self.xp -= self.needed_xp;
            (self.base_health, self.base_damage) = Player::stats_from_level(self.level);
            self.needed_xp = Player::calculate_needed_xp(self.level);
        }
        if self.level != prev_level {
            if let Some(q) = &mut self.msg_queue.unwrap_queue() {
                q.push("Level up!".into());
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
            coins: 0,
            needed_xp: Player::calculate_needed_xp(1),

            msg_queue: MessageQueue::default(),
        }
    }

    pub fn get_message_queue(&self) -> MessageQueue {
        self.msg_queue.clone()
    }

    pub fn set_message_queue(&mut self, msg_queue: MessageQueue) {
        self.msg_queue = msg_queue;
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn to_entity(&self) -> Entity {
        Entity::new(self.base_health, self.base_damage, &self.name)
    }
}
