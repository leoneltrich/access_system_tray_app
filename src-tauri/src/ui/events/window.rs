use tauri::{Runtime, WebviewWindow, WindowEvent};

pub fn handle_event<R: Runtime>(window: &WebviewWindow<R>, event: &WindowEvent) {
    match event {
        // Hide window when it loses focus (Standard Tray behavior)
        WindowEvent::Focused(is_focused) => {
            if !*is_focused {
                // We ignore the result here as the window might be closing
                let _ = window.hide();
            }
        }
        // Prevent the window from actually closing when user hits "X" or Alt+F4
        WindowEvent::CloseRequested { api, .. } => {
            let _ = window.hide();
            api.prevent_close();
        }
        _ => {}
    }
}