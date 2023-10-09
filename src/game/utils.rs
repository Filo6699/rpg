use std::sync::{Arc, Mutex};

pub type MessageQueue = Arc<Mutex<Vec<String>>>;
