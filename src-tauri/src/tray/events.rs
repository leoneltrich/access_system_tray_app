use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    AppHandle, Manager, Runtime,
};
use tauri_plugin_positioner::{Position, WindowExt};
use crate::constants::MAIN_WINDOW_LABEL;

pub fn handle_tray_click<R: Runtime>(app: &AppHandle<R>, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
            // 1. Move to tray (Clean and stable)
            let _ = window.move_window(Position::TrayCenter);

            // 2. Toggle Visibility
            if window.is_visible().unwrap_or(false) {
                let _ = window.hide();
            } else {
                let _ = window.show();
                let _ = window.set_focus();
                let _ = window.set_always_on_top(true);
            }
        }
    }
}