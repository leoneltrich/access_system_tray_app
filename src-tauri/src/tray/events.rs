use crate::constants::MAIN_WINDOW_LABEL;
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    AppHandle, Manager, Runtime, WebviewWindow,
};
use tauri_plugin_positioner::{Position, WindowExt};

// --- SHARED HELPER ---
// FIX: Added <R: Runtime> so it accepts the generic window type
pub fn position_and_show<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    // 1. Position based on OS
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

// --- MAIN HANDLER ---
pub fn handle_tray_click<R: Runtime>(app: &AppHandle<R>, event: TrayIconEvent) {
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
                // Now this matches because both use <R>
                let _ = position_and_show(&window);
            }
        }
    }
}