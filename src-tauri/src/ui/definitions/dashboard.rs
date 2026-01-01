use tauri::{AppHandle, Runtime, WebviewWindowBuilder};

// CHANGED: Added <'a> for consistency
pub fn configure<'a, R: Runtime>(
    builder: WebviewWindowBuilder<'a, R, AppHandle<R>>
) -> WebviewWindowBuilder<'a, R, AppHandle<R>> {
    let mut b = builder
        .title("Tray App")
        .inner_size(300.0, 400.0)
        .decorations(false)
        .visible(false)
        .skip_taskbar(true)
        .always_on_top(true)
        .shadow(true);

    #[cfg(target_os = "macos")]
    { b = b.transparent(true); }
    #[cfg(not(target_os = "macos"))]
    { b = b.transparent(false); }

    b
}