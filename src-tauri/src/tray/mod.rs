use tauri::{
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton},
    Manager, Runtime, AppHandle,
};
use tauri_plugin_positioner::{WindowExt, Position}; // Ensure this is imported for Windows positioning

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
                        // --- MAC OS LOGIC ---
                        #[cfg(target_os = "macos")]
                        {
                            // ONLY do this for Mac.
                            // The plugin handles the "Toggle" (Show/Hide) and "Positioning" automatically.
                            // Do NOT manually call win.show() here, or you break the toggle.
                            tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);

                            // (Optional) If you have extra logic in events::handle_tray_click, run it here
                            events::handle_tray_click(tray.app_handle(), event);
                        }

                        // --- WINDOWS LOGIC ---
                        #[cfg(target_os = "windows")]
                        {
                            // 1. Toggle Logic: If visible -> Hide, If hidden -> Show & Move
                            if win.is_visible().unwrap_or(false) {
                                let _ = win.hide();
                            } else {
                                // Move to Bottom Right
                                let _ = win.move_window(Position::BottomRight);
                                // Show and Focus
                                let _ = win.show();
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