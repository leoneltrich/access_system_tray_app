mod api;
mod constants;
mod core;
mod state;
mod ui;

use std::sync::atomic::Ordering;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

// Import our new modules
use state::AppState;
use ui::definitions::WindowType;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use crate::api::system::fix_autostart_path;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 1. Register Commands
        .invoke_handler(tauri::generate_handler![
            fix_autostart_path
        ])

        // 2. Manage State
        .manage(AppState::new())

        // 3. Register Plugins
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))

        // 4. Setup Hook (The Boot Sequence)
        .setup(|app| {
            // 1. DO MUTABLE WORK FIRST
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            // 2. NOW GET THE IMMUTABLE HANDLE
            let handle = app.handle();

            // 3. PROCEED
            ui::tray::setup(handle)?;
            ui::windows::create(handle, WindowType::Dashboard)?;

            Ok(())
        })

        // 5. Run Loop & Exit Handling
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                let state = app_handle.state::<AppState>();

                // Only allow exit if the explicit "Quit" button was clicked
                if !state.is_quitting.load(Ordering::Relaxed) {
                    api.prevent_exit();
                }
            }
        });
}