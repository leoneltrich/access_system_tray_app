use tauri::{AppHandle, Runtime, WebviewWindowBuilder};

pub fn configure<R: Runtime>(
    builder: WebviewWindowBuilder<R, AppHandle<R>>
) -> WebviewWindowBuilder<R, AppHandle<R>> {
    let mut b = builder
        .title("Tray App")
        .inner_size(325.0, 400.0)
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