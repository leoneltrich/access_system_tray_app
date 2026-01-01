use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Runtime,
};
use crate::ui::events;

pub fn setup<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    // 1. Get the App Icon
    // Ensure you have icons in src-tauri/icons/ or this unwrap might panic in dev
    let icon = app.default_window_icon().expect("No app icon found").clone();

    // 2. Define Menu Items
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Open Dashboard", true, None::<&str>)?;

    // 3. Build Menu
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

    // 4. Build Tray
    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)

        // Delegate Menu Events (Right-click items)
        .on_menu_event(move |app, event| {
            events::tray::handle_menu_event(app, event.id.as_ref());
        })

        // Delegate Icon Events (Left-click toggle)
        .on_tray_icon_event(|tray, event| {
            // Mac handling is unique due to OS behavior
            #[cfg(target_os = "macos")]
            tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);

            events::tray::handle_icon_click(tray.app_handle(), event);
        })
        .build(app)?;

    Ok(())
}