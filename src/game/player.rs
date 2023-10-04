use super::{battle::Entity, utils::MessageQueue};

pub struct Player {
    level: u32,
    base_health: u32,
    base_damage: u32,
    xp: u64,
    needed_xp: u64,
    coins: u64,
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

    pub fn get_coins(&self) -> u64 {
        self.coins
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
        (level.pow(2) * 40 + 60).into()
    }

    fn stats_from_level(level: u32) -> (u32, u32) {
        (level * 30 + 100, level * 2 + 10)
    }

    pub fn add_coins(&mut self, coins: u64) {
        self.coins += coins;
    }

    pub fn add_xp(&mut self, xp: u64) {
        self.xp += xp;
        let prev_level = self.level;
        while self.xp >= self.needed_xp {
            self.level += 1;
            self.xp -= self.needed_xp;
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
            coins: 0,
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
