use tauri::{tray::TrayIconBuilder, AppHandle, Runtime};
// Removed unused "WindowExt" import

mod events;

pub fn setup<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let icon = app.default_window_icon().unwrap().clone();

    TrayIconBuilder::new()
        .icon(icon)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
            events::handle_tray_click(tray.app_handle(), event);
        })
        .build(app)?;

    Ok(())
}
