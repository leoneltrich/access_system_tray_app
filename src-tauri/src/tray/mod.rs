use tauri::{
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton},
    Manager, Runtime, AppHandle,
};
use tauri_plugin_positioner::{WindowExt, Position};

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

                        // --- WINDOWS LOGIC (Fixed) ---
                        #[cfg(target_os = "windows")]
                        {
                            if win.is_visible().unwrap_or(false) {
                                // If visible, hide it
                                let _ = win.hide();
                            } else {
                                // 1. Show the window FIRST so it can grab resources
                                let _ = win.show();

                                // 2. Move it to the correct spot
                                let _ = win.move_window(Position::BottomRight);

                                // 3. FORCE FOCUS
                                // We re-apply always_on_top to force the OS to acknowledge it
                                let _ = win.set_always_on_top(true);
                                let _ = win.set_focus();
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