mod constants;
mod tray;
mod window;

use tauri::Manager;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            // Get the handle from the app instance
            let handle = app.handle();

            // 1. Initialize System Tray (pass the handle)
            tray::setup(handle)?;

            // 2. Initialize Window Behaviors (pass the handle)
            window::setup_events(handle);

            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(ActivationPolicy::Accessory);
            }

            if let Some(window) = app.get_webview_window("main") {
                // This ensures the window floats above Fullscreen apps (like Terminal)
                // and doesn't pull you back to the Desktop.
                let _ = window.set_visible_on_all_workspaces(true);
                let _ = window.set_always_on_top(true);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}