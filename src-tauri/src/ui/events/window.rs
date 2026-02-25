use tauri::{Manager, Runtime, WebviewWindow, WindowEvent};
use crate::state::AppState;
use std::sync::atomic::Ordering;

pub fn handle_event<R: Runtime>(window: &WebviewWindow<R>, event: &WindowEvent) {
    match event {
        // Hide window when it loses focus (Standard Tray behavior)
        WindowEvent::Focused(is_focused) => {
            if !*is_focused {
                let state = window.state::<AppState>();
                
                // Only hide if we aren't currently showing a system dialog
                if !state.is_dialog_open.load(Ordering::Relaxed) {
                    let _ = window.hide();
                }
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