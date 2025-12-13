mod constants;
mod tray;
mod window;

use tauri::ActivationPolicy;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
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
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}