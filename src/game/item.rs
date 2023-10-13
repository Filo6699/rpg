use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ItemType {
    Sword,
    Shield,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemProperties {
    pub damage: u128,
    pub defence: u128,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub cost: u32,
    pub properties: ItemProperties,
}
