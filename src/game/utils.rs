use std::sync::{Arc, Mutex};

pub type MessageQueue = Arc<Mutex<Vec<String>>>;

pub fn calculate_bar(value: u64, max_value: u64, precision: u32) -> (String, String) {
    let percentage: u64 = value * <u32 as Into<u64>>::into(precision) / max_value;
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
