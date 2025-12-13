use tauri::{AppHandle, Manager, Runtime, WindowEvent, WebviewWindow};
use crate::constants::MAIN_WINDOW_LABEL;

pub fn setup_events<R: Runtime>(app: &AppHandle<R>) {
    // Explicitly define the type for the window variable to help the compiler
    if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let window_clone: WebviewWindow<R> = window.clone();

        // To make it visible on other full screen apps
        let _ = window.set_visible_on_all_workspaces(true);
        let _ = window.set_always_on_top(true);

        window.on_window_event(move |event| {
            if let WindowEvent::Focused(focused) = event {
                if !focused {
                    let _ = window_clone.hide();
                }
            }
        });
    }
}