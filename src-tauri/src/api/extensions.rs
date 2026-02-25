use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

const VERSION_SEPARATOR: &str = " - ";

#[tauri::command]
pub async fn upload_extension<R: Runtime>(
    app: AppHandle<R>,
    name: String,
    data: Vec<u8>,
) -> Result<(), String> {
    let extensions_dir = construct_extensions_dir_path(app)?;

    fs::create_dir_all(&extensions_dir)
        .map_err(|e| format!("Failed to create extensions directory: {}", e))?;

    handle_single_version_constraint(&name, &extensions_dir)?;

    let mut target_path = extensions_dir.clone();
    target_path.push(name);

    fs::write(&target_path, data).map_err(|e| {
        format!(
            "Failed to write extension file '{}': {}",
            target_path.display(),
            e
        )
    })?;

    #[cfg(unix)]
    set_permissions_unix(&mut target_path)?;

    Ok(())
}

/**
Ensures that only a single version of an extension is installed.
**/
fn handle_single_version_constraint(name: &String, extensions_dir: &PathBuf) -> Result<(), String> {
    let base_name = get_base_name(&name);
    clean_old_versions(&extensions_dir, &base_name)?;
    Ok(())
}

fn set_permissions_unix(target_path: &mut PathBuf) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&target_path)
        .map_err(|e| {
            format!(
                "Failed to get metadata for {}: {}",
                target_path.display(),
                e
            )
        })?
        .permissions();

    perms.set_mode(0o755);
    fs::set_permissions(&target_path, perms).map_err(|e| {
        format!(
            "Failed to set permissions for {}: {}",
            target_path.display(),
            e
        )
    })?;
    Ok(())
}

fn construct_extensions_dir_path<R: Runtime>(app: AppHandle<R>) -> Result<PathBuf, String> {
    let mut extensions_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    extensions_dir.push("Extensions");
    Ok(extensions_dir)
}

#[derive(serde::Serialize)]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
}

#[tauri::command]
pub async fn list_extensions<R: Runtime>(app: AppHandle<R>) -> Result<Vec<ExtensionInfo>, String> {
    let extensions_dir = construct_extensions_dir_path(app)?;
    
    if !extensions_dir.exists() {
        return Ok(Vec::new());
    }

    let mut list = Vec::new();

    for entry in fs::read_dir(extensions_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                list.push(ExtensionInfo {
                    id: filename.to_string(),
                    name: get_base_name(filename),
                    version: get_version(filename),
                });
            }
        }
    }

    // Sort alphabetically by name
    list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(list)
}

fn get_version(full_file_name: &str) -> String {
    let path = PathBuf::from(full_file_name);
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(full_file_name);

    if let Some(index) = file_stem.rfind(VERSION_SEPARATOR) {
        // Extract everything after the separator
        file_stem[index + VERSION_SEPARATOR.len()..].trim().to_string()
    } else {
        "unknown".to_string()
    }
}

fn get_base_name(full_file_name: &str) -> String {
    let path = PathBuf::from(full_file_name);

    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(full_file_name);

    if let Some(index) = file_stem.rfind(VERSION_SEPARATOR) {
        file_stem[..index].trim().to_string()
    } else {
        file_stem.to_string()
    }
}

fn clean_old_versions(extensions_dir: &PathBuf, base_name: &str) -> Result<(), String> {
    if !extensions_dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(extensions_dir)
        .map_err(|e| format!("Failed to read extensions directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if get_base_name(file_name) == base_name {
                    fs::remove_file(&path)
                        .map_err(|e| format!("Failed to remove file '{}': {}", file_name, e))?;
                }
            }
        }
    }

    Ok(())
}
