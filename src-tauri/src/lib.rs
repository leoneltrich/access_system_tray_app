mod constants;
mod tray;
// REMOVED: mod window; (We will do this logic here to be safe)

use crate::constants::MAIN_WINDOW_LABEL;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri_plugin_autostart::MacosLauncher;
#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

// 1. Define a global state to track if we REALLY want to quit
pub struct AppState {
    pub is_quitting: AtomicBool,
}


#[tauri::command]
fn fix_autostart_path(app: tauri::AppHandle) {
    #[cfg(target_os = "windows")]
    {
        use std::env;

        // 1. Get the current executable path
        if let Ok(exe_path) = env::current_exe() {
            let exe_str = exe_path.to_string_lossy().to_string();

            // 2. Open the Registry Key
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let path = std::path::Path::new("Software")
                .join("Microsoft")
                .join("Windows")
                .join("CurrentVersion")
                .join("Run");

            if let Ok(key) = hkcu.open_subkey_with_flags(&path, KEY_SET_VALUE | KEY_READ) {
                // 3. Get the App Name (Must match what tauri-plugin-autostart uses)
                // usually it is the productName from tauri.conf.json.
                // Based on your previous logs, check RegEdit for the exact name.
                // I will assume "Tray App" based on your window title,
                // BUT it is usually your "productName" in tauri.conf.json.
                let app_name = "ServeMe";

                // 4. Force overwrite with Quoted Path
                let quoted_path = format!("\"{}\"", exe_str);
                println!("Fixing Autostart for '{}': {}", app_name, quoted_path);

                let _ = key.set_value(&app_name, &quoted_path);
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Windows CWD Fix
    #[cfg(target_os = "windows")]
    {
        use std::env;
        if let Ok(current_exe) = env::current_exe() {
            if let Some(parent) = current_exe.parent() {
                let _ = env::set_current_dir(parent);
            }
        }
    }

    // inside pub fn run()
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fix_autostart_path]) // <-- ADD THIS
        // ... .plugin(...)
        // 2. Manage the Quit State
        .manage(AppState {
            is_quitting: AtomicBool::new(false),
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            let handle = app.handle();
            tray::setup(handle)?;

            // REMOVED: window::setup_events(handle); -> It was failing silently

            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            let mut builder = WebviewWindowBuilder::new(app, MAIN_WINDOW_LABEL, WebviewUrl::default())
                .title("Tray App")
                .inner_size(300.0, 400.0)
                .decorations(false)
                .visible(false)
                .skip_taskbar(true)
                .always_on_top(true)
                .shadow(true);

            #[cfg(target_os = "macos")]
            { builder = builder.transparent(true); }
            #[cfg(not(target_os = "macos"))]
            { builder = builder.transparent(false); }

            let win = builder.build().expect("Failed to build window");
            let win_clone = win.clone();

            // 3. Attach Window Events Correctly (After build)
            win.on_window_event(move |event| match event {
                WindowEvent::Focused(is_focused) => {
                    if !is_focused {
                        let _ = win_clone.hide();
                    }
                }
                // Handle "X" or Alt+F4 by hiding instead of closing
                WindowEvent::CloseRequested { api, .. } => {
                    let _ = win_clone.hide();
                    api.prevent_close();
                }
                _ => {}
            });

            let _ = win.set_visible_on_all_workspaces(true);
            let _ = win.set_always_on_top(true);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                // 4. The Logic: Only allow exit if the flag is TRUE
                let state = app_handle.state::<AppState>();
                if !state.is_quitting.load(Ordering::Relaxed) {
                    println!("Preventing exit - keeping background process alive.");
                    api.prevent_exit();
                } else {
                    println!("Allowing exit - user requested quit.");
                }
            }
            _ => {}
        });
}
