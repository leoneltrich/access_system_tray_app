use tauri::{
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};
use tauri_plugin_positioner::WindowExt;

mod events;

pub fn setup<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let icon = app.default_window_icon().unwrap().clone();

    TrayIconBuilder::new()
        .icon(icon)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let Some(win) = tray.app_handle().get_webview_window("main") {
                match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } => {
                        // --- MAC OS LOGIC (Unchanged) ---
                        #[cfg(target_os = "macos")]
                        {
                            tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                            events::handle_tray_click(tray.app_handle(), event);
                        }

                        // --- WINDOWS LOGIC (With Delay Fix) ---
                        #[cfg(target_os = "windows")]
                        {
                            if win.is_visible().unwrap_or(false) {
                                // If already open, close it immediately
                                let _ = win.hide();
                            } else {
                                // CLONE the window handle to move it into the thread
                                let win_clone = win.clone();

                                // SPAWN a thread to wait 150ms
                                thread::spawn(move || {
                                    // 1. Wait for the Taskbar click event to "settle"
                                    thread::sleep(Duration::from_millis(150));

                                    // 2. Now execute the logic
                                    // Move to bottom right
                                    let _ = win_clone.move_window(Position::BottomRight);

                                    // Show
                                    let _ = win_clone.show();

                                    // Force Focus (This closes the Tray Overflow menu)
                                    let _ = win_clone.set_focus();
                                    let _ = win_clone.set_always_on_top(true);
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