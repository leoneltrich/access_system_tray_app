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

// 1. Define a global state to track if we REALLY want to quit
pub struct AppState {
    pub is_quitting: AtomicBool,
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

    tauri::Builder::default()
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