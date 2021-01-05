use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone)]
pub struct State {
    db: Arc<Mutex<HashMap<String, Vec<String>>>>
}

impl State {
    pub fn new() -> State {
        State {
            db: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

