use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    AppHandle, Manager, Runtime, WebviewWindow,
};
use tauri_plugin_positioner::{Position, WindowExt};
use std::sync::atomic::Ordering;
use crate::AppState;
use crate::constants::MAIN_WINDOW_LABEL;

// --- Helper: Position & Show ---
fn position_and_show<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    // 1. Position
    #[cfg(target_os = "macos")]
    {
        window.move_window(Position::TrayCenter)?;
    }
    #[cfg(target_os = "windows")]
    {
        window.move_window(Position::BottomRight)?;
    }

    // 2. Show & Focus
    window.show()?;
    window.set_focus()?;
    window.set_always_on_top(true)?;

    Ok(())
}

// --- Handler: Left Click on Icon ---
pub fn handle_icon_click<R: Runtime>(app: &AppHandle<R>, event: TrayIconEvent) {
    // We only care about the specific Left Click UP event
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
            // Toggle Logic
            if window.is_visible().unwrap_or(false) {
                let _ = window.hide();
            } else {
                let _ = position_and_show(&window);
            }
        }
    }
}

// --- Handler: Right Click Menu Items ---
pub fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event_id: &str) {
    match event_id {
        "quit" => {
            let state = app.state::<AppState>();
            // Signal that we actually intend to quit now
            state.is_quitting.store(true, Ordering::Relaxed);
            app.exit(0);
        }
        "show" => {
            if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
                let _ = position_and_show(&window);
            }
        }
        _ => {}
    }
}