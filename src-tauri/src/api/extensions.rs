use crate::state::AppState;
use std::fs;
use tauri::{AppHandle, Emitter, Manager, Runtime, State};

const VERSION_SEPARATOR: &str = " - ";

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub is_running: bool,
}

#[tauri::command]
pub async fn list_extensions<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
) -> Result<Vec<ExtensionInfo>, String> {
    let extensions_dir = construct_extensions_dir_path(&app)?;

    if !extensions_dir.exists() {
        return Ok(Vec::new());
    }

    let mut list = Vec::new();
    let mut running = state.running_extensions.lock().unwrap();

    for entry in fs::read_dir(extensions_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        let is_app = path.is_dir() && path.extension().map_or(false, |e| e == "app");

        if path.is_file() || is_app {
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                let id = filename.to_string();
                let mut is_running = false;

                if let Some(child) = running.get_mut(&id) {
                    match child.try_wait() {
                        Ok(None) => is_running = true,
                        Ok(Some(status)) => {
                            if !status.success() {
                                let name = get_base_name(filename);
                                let _ = app.emit(
                                    "extension-crash",
                                    format!("Extension '{}' exited with an error.", name),
                                );
                            }
                            running.remove(&id);
                        }
                        _ => {
                            running.remove(&id);
                        }
                    }
                }

                list.push(ExtensionInfo {
                    id: filename.to_string(),
                    name: get_base_name(filename),
                    version: get_version(filename),
                    is_running,
                });
            }
        }
    }

    list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(list)
}

#[tauri::command]
pub async fn upload_extension<R: Runtime>(
    app: AppHandle<R>,
    source_path: String,
) -> Result<(), String> {
    let source = std::path::PathBuf::from(&source_path);
    if !source.exists() {
        return Err(format!("Source path '{}' does not exist", source_path));
    }

    let name = source
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Invalid source filename".to_string())?
        .to_string();

    let extensions_dir = construct_extensions_dir_path(&app)?;
    fs::create_dir_all(&extensions_dir)
        .map_err(|e| format!("Failed to create extensions directory: {}", e))?;

    handle_single_version_constraint(&name, &extensions_dir)?;

    let mut target_path = extensions_dir.clone();
    target_path.push(&name);

    if source.is_dir() {
        copy_dir_recursive(&source, &target_path)
            .map_err(|e| format!("Failed to copy directory extension: {}", e))?;
    } else {
        fs::copy(&source, &target_path).map_err(|e| {
            format!(
                "Failed to copy extension file '{}': {}",
                target_path.display(),
                e
            )
        })?;
    }

    #[cfg(unix)]
    if target_path.is_file() {
        set_permissions_unix(&mut target_path)?;
    }

    Ok(())
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn run_extension<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mut running = state.running_extensions.lock().unwrap();

    if let Some(child) = running.get_mut(&id) {
        match child.try_wait() {
            Ok(None) => return Err(format!("Extension '{}' is already running", id)),
            _ => {
                running.remove(&id);
            }
        }
    }

    let mut path = construct_extensions_dir_path(&app)?;
    path.push(&id);

    if !path.exists() {
        return Err(format!("Extension '{}' not found", id));
    }

    let mut command = {
        #[cfg(target_os = "macos")]
        if path.is_dir() && path.extension().map_or(false, |e| e == "app") {
            let mut cmd = std::process::Command::new("open");
            cmd.arg("-W").arg(&path);
            cmd
        } else {
            std::process::Command::new(&path)
        }

        #[cfg(not(target_os = "macos"))]
        std::process::Command::new(&path)
    };

    let child = command
        .spawn()
        .map_err(|e| format!("Failed to spawn extension '{}': {}", id, e))?;

    running.insert(id, child);
    Ok(())
}

#[tauri::command]
pub async fn stop_extension<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mut running = state.running_extensions.lock().unwrap();

    if let Some(mut child) = running.remove(&id) {
        #[cfg(target_os = "macos")]
        if id.ends_with(".app") {
            if let Ok(mut path) = construct_extensions_dir_path(&app) {
                path.push(&id);
                let _ = std::process::Command::new("osascript")
                    .arg("-e")
                    .arg(format!("quit app \"{}\"", path.display()))
                    .spawn();
            }
        }
        let _ = child.kill();
    }

    Ok(())
}

pub fn cleanup_processes<R: Runtime>(app: &AppHandle<R>, state: &AppState) {
    let mut running = state.running_extensions.lock().unwrap();
    let extensions_dir = construct_extensions_dir_path(&app).ok();

    for (id, mut child) in running.drain() {
        #[cfg(target_os = "macos")]
        if id.ends_with(".app") {
            if let Some(mut path) = extensions_dir.clone() {
                path.push(&id);
                let _ = std::process::Command::new("osascript")
                    .arg("-e")
                    .arg(format!("quit app \"{}\"", path.display()))
                    .spawn();
            }
        }
        let _ = child.kill();
    }
}

fn handle_single_version_constraint(
    name: &String,
    extensions_dir: &std::path::PathBuf,
) -> Result<(), String> {
    let base_name = get_base_name(&name);
    clean_old_versions(&extensions_dir, &base_name)?;
    Ok(())
}

#[cfg(unix)]
fn set_permissions_unix(target_path: &mut std::path::PathBuf) -> Result<(), String> {
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

fn construct_extensions_dir_path<R: Runtime>(
    app: &AppHandle<R>,
) -> Result<std::path::PathBuf, String> {
    let mut extensions_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    extensions_dir.push("Extensions");
    Ok(extensions_dir)
}

#[tauri::command]
pub async fn delete_extension<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mut running = state.running_extensions.lock().unwrap();
    if let Some(mut child) = running.remove(&id) {
        #[cfg(target_os = "macos")]
        if id.ends_with(".app") {
            if let Ok(mut path) = construct_extensions_dir_path(&app) {
                path.push(&id);
                let _ = std::process::Command::new("osascript")
                    .arg("-e")
                    .arg(format!("quit app \"{}\"", path.display()))
                    .spawn();
            }
        }
        let _ = child.kill();
    }
    drop(running);

    let mut path = construct_extensions_dir_path(&app)?;
    path.push(&id);

    if path.exists() {
        if path.is_dir() {
            fs::remove_dir_all(&path).map_err(|e| format!("Failed to delete directory: {}", e))?;
        } else {
            fs::remove_file(&path).map_err(|e| format!("Failed to delete file: {}", e))?;
        }
    }

    Ok(())
}

fn get_version(full_file_name: &str) -> String {
    let path = std::path::PathBuf::from(full_file_name);
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(full_file_name);

    if let Some(index) = file_stem.rfind(VERSION_SEPARATOR) {
        file_stem[index + VERSION_SEPARATOR.len()..]
            .trim()
            .to_string()
    } else {
        "unknown".to_string()
    }
}

fn get_base_name(full_file_name: &str) -> String {
    let path = std::path::PathBuf::from(full_file_name);

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

fn clean_old_versions(extensions_dir: &std::path::PathBuf, base_name: &str) -> Result<(), String> {
    if !extensions_dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(extensions_dir)
        .map_err(|e| format!("Failed to read extensions directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        let is_app = path.is_dir() && path.extension().map_or(false, |e| e == "app");

        if path.is_file() || is_app {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if get_base_name(file_name) == base_name {
                    if path.is_dir() {
                        fs::remove_dir_all(&path).map_err(|e| {
                            format!("Failed to remove directory '{}': {}", file_name, e)
                        })?;
                    } else {
                        fs::remove_file(&path)
                            .map_err(|e| format!("Failed to remove file '{}': {}", file_name, e))?;
                    }
                }
            }
        }
    }

    Ok(())
}
