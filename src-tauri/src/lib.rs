mod api;
mod constants;
mod core;
mod state;
mod ui;

use std::sync::atomic::Ordering;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

use state::AppState;
use ui::definitions::WindowType;

use crate::api::auth::*;
use crate::api::extensions::{cleanup_processes, list_extensions, run_extension, stop_extension, upload_extension, delete_extension};
use crate::api::system::{fix_autostart_path, set_dialog_status};
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::RunEvent::{Exit, ExitRequested};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Register Commands
        .invoke_handler(tauri::generate_handler![
            fix_autostart_path,
            upload_extension,
            list_extensions,
            run_extension,
            stop_extension,
            delete_extension,
            set_dialog_status,
            save_tokens,
            get_tokens,
            get_access_token,
            get_refresh_token,
            purge_tokens,
        ])
        // Manage State
        .manage(AppState::new())
        // Register Plugins
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            let handle = app.handle();

            ui::tray::setup(handle)?;
            ui::windows::create(handle, WindowType::Dashboard)?;

            Ok(())
        })
        // Run Loop & Exit Handling
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            let state = app_handle.state::<AppState>();

            match event {
                ExitRequested { api, .. } => {
                    if !state.is_quitting.load(Ordering::Relaxed) {
                        api.prevent_exit();
                    }
                }
                Exit => {
                    cleanup_processes(&state);
                }
                _ => {}
            }
        });
}
