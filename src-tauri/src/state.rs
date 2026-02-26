use std::collections::HashMap;
use std::process::Child;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

pub struct AppState {
    pub is_quitting: AtomicBool,
    pub is_dialog_open: AtomicBool,
    pub running_extensions: Mutex<HashMap<String, Child>>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            is_quitting: AtomicBool::new(false),
            is_dialog_open: AtomicBool::new(false),
            running_extensions: Mutex::new(HashMap::new())
        }
    }
}