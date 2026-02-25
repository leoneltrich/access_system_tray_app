use std::sync::atomic::AtomicBool;

pub struct AppState {
    pub is_quitting: AtomicBool,
    pub is_dialog_open: AtomicBool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            is_quitting: AtomicBool::new(false),
            is_dialog_open: AtomicBool::new(false),
        }
    }
}