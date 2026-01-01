use tauri::AppHandle;
use crate::core::services::autostart;

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