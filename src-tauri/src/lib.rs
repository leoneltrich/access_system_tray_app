mod constants;
mod tray;
mod window;

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};

use crate::constants::MAIN_WINDOW_LABEL;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]) // No extra arguments passed on startup
        ))
        .setup(|app| {
            let handle = app.handle();

            // 1. Initialize System Tray
            tray::setup(handle)?;

            // 2. Initialize Window Behaviors
            window::setup_events(handle);

            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(ActivationPolicy::Accessory);
            }

            // 3. CREATE WINDOW PROGRAMMATICALLY
            // This replaces the "windows" block in tauri.conf.json
            let mut builder =
                WebviewWindowBuilder::new(app, MAIN_WINDOW_LABEL, WebviewUrl::default())
                    .title("Tray App")
                    .inner_size(300.0, 400.0)
                    .decorations(false)
                    .visible(false) // Start hidden (like your config)
                    .skip_taskbar(true)
                    .always_on_top(true)
                    .shadow(true);

            // CONDITIONAL TRANSPARENCY
            // macOS: Transparent (allows floating rounded corners via CSS)
            #[cfg(target_os = "macos")]
            {
                builder = builder.transparent(true);
            }

            // Windows: Opaque (prevents the white border artifact)
            #[cfg(not(target_os = "macos"))]
            {
                builder = builder.transparent(false);
            }

            // Build the window
            let win = builder.build().expect("Failed to build window");

            let win_clone = win.clone();
            win.on_window_event(move |event| match event {
                WindowEvent::Focused(is_focused) => {
                    if !is_focused {
                        let _ = win_clone.hide();
                    }
                }
                _ => {}
            });

            // 4. Runtime Configurations
            // We set these here to be safe as they work on the window instance
            let _ = win.set_visible_on_all_workspaces(true);

            // Re-enforce always on top just in case
            let _ = win.set_always_on_top(true);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
