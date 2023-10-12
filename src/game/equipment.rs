use serde::{Deserialize, Serialize};

use super::item::Item;

#[derive(Default, Serialize, Deserialize)]
pub struct Equipment {
    pub sword: Option<Item>,
    pub shield: Option<Item>,
}
