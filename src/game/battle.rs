#[allow(unused)]
#[derive(Clone)]
pub struct Entity {
    pub health: u128,
    damage: u128,
    pub name: String,
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
}
