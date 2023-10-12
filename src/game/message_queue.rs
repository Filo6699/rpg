use std::sync::{Arc, Mutex};

#[derive(Clone, Default, Debug)]
pub struct MessageQueue {
    pub msgs: Option<Arc<Mutex<Vec<String>>>>,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            msgs: Some(Arc::new(Mutex::new(vec![]))),
        }
    }

    pub fn unwrap_queue(&self) -> Option<std::sync::MutexGuard<'_, Vec<String>>> {
        if let Some(queue) = &self.msgs {
            return Some(queue.lock().unwrap());
        }
        None
    }

    pub fn get_message(&self) -> Option<String> {
        if let Some(queue) = &self.msgs {
            let unwrapped = queue.lock().unwrap();
            let output = unwrapped.get(0);
            return output.cloned();
        }
        None
    }

    pub fn has_message(&self) -> bool {
        if let Some(queue) = &self.msgs {
            let unwrapped = queue.lock().unwrap();
            return unwrapped.is_empty();
        }
        false
    }

    pub fn pop_message(&mut self) {
        if let Some(queue) = &self.msgs {
            let mut unwrapped = queue.lock().unwrap();
            if unwrapped.len() > 0 {
                unwrapped.remove(0);
            }
        }
    }
}
