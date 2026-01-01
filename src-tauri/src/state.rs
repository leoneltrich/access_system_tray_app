use std::sync::atomic::AtomicBool;

pub struct AppState {
    pub is_quitting: AtomicBool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            is_quitting: AtomicBool::new(false),
        }
    }
}