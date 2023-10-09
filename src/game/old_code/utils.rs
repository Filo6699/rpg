use std::sync::{Arc, Mutex};

pub type MessageQueue = Arc<Mutex<Vec<String>>>;

pub fn calculate_bar(value: u128, max_value: u128, precision: u32) -> (String, String) {
    let percentage: u128 = value * <u32 as Into<u128>>::into(precision) / max_value;
    let mut filled = String::from("");
    let mut empty = String::from("");
    for i in 0..precision {
        if percentage > i.into() {
            filled.push('■');
        } else {
            empty.push('■');
        }
    }
    (filled, empty)
}
