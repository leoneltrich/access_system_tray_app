use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder, WebviewWindow};
use crate::ui::definitions::WindowType;
use crate::ui::events;

pub fn create<R: Runtime>(app: &AppHandle<R>, win_type: WindowType) -> tauri::Result<WebviewWindow<R>> {
    let label = win_type.label();

    // 1. Idempotency Check: Return existing if open
    if let Some(win) = app.get_webview_window(label) {
        return Ok(win);
    }

    // 2. Create the Builder
    let builder = WebviewWindowBuilder::new(app, label, WebviewUrl::default());

    // 3. Apply Configuration (From Definitions)
    let builder = win_type.configure(builder);

    // 4. Build the Window
    let window = builder.build()?;

    // 5. Attach Event Listener (From Events)
    let win_clone = window.clone();
    window.on_window_event(move |event| {
        events::window::handle_event(&win_clone, event);
    });

    Ok(window)
}