use std::path::PathBuf;

#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

pub fn ensure_windows_autostart(current_exe: PathBuf) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let exe_str = current_exe.to_string_lossy().to_string();
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = std::path::Path::new("Software")
            .join("Microsoft")
            .join("Windows")
            .join("CurrentVersion")
            .join("Run");

        let key = hkcu.open_subkey_with_flags(&path, KEY_SET_VALUE | KEY_READ)
            .map_err(|e| e.to_string())?;

        let app_name = "ServeMe"; // TODO: Extract to config
        let quoted_path = format!("\"{}\"", exe_str);

        key.set_value(&app_name, &quoted_path)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}