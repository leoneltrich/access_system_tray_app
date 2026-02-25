use tauri::{AppHandle, State};
use crate::core::services::autostart;
use crate::state::AppState;
use std::sync::atomic::Ordering;

#[tauri::command]
pub async fn set_dialog_status(state: State<'_, AppState>, is_open: bool) -> Result<(), String> {
    state.is_dialog_open.store(is_open, Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub fn fix_autostart_path(_app: AppHandle) {
    use std::env;

    // Adapter Logic: Get data from environment, pass to Domain Service
    if let Ok(exe_path) = env::current_exe() {
        match autostart::ensure_windows_autostart(exe_path) {
            Ok(_) => println!("Autostart path fixed successfully."),
            Err(e) => eprintln!("Failed to fix autostart: {}", e),
        }
    }
}