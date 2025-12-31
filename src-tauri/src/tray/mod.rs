use std::sync::atomic::Ordering;
use crate::constants::MAIN_WINDOW_LABEL;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};
use crate::AppState;

mod events;

pub fn setup<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let icon = app.default_window_icon().unwrap().clone();

    // 1. Context Menu
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Open Dashboard", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)

        // 2. Menu Events (Right Click)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                // FLIP THE SWITCH
                let state = app.state::<AppState>();
                state.is_quitting.store(true, Ordering::Relaxed);

                app.exit(0); // Now this will pass the check in lib.rs
            },
            "show" => {
                if let Some(win) = app.get_webview_window(MAIN_WINDOW_LABEL) {
                    let _ = events::position_and_show(&win);
                }
            }
            _ => {}
        })

        // 3. Tray Icon Events (Left Click)
        .on_tray_icon_event(|tray, event| {
            if let Some(win) = tray.app_handle().get_webview_window(MAIN_WINDOW_LABEL) {
                match event {
                    TrayIconEvent::Click { button: MouseButton::Left, .. } => {

                        // --- MAC OS ---
                        #[cfg(target_os = "macos")]
                        {
                            // Required for Mac tray behavior
                            tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                            // Delegate to events.rs
                            events::handle_tray_click(tray.app_handle(), event);
                        }

                        // --- WINDOWS ---
                        #[cfg(target_os = "windows")]
                        {
                            use std::thread;
                            use std::time::Duration;
                            
                            if win.is_visible().unwrap_or(false) {
                                let _ = win.hide();
                            } else {
                                let win_clone = win.clone();
                                thread::spawn(move || {
                                    thread::sleep(Duration::from_millis(150));
                                    // Reuse the helper inside the thread
                                    let _ = events::position_and_show(&win_clone);
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        })
        .build(app)?;

    Ok(())
}