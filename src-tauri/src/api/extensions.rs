use crate::state::AppState;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
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

// --- Commands ---

#[tauri::command]
pub async fn list_extensions<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
) -> Result<Vec<ExtensionInfo>, String> {
    let extensions_dir = get_extensions_dir(&app)?;
    if !extensions_dir.exists() {
        return Ok(Vec::new());
    }

    let mut list = Vec::new();
    let mut running = state.running_extensions.lock().unwrap();

    for entry in fs::read_dir(extensions_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !is_valid_extension_item(&path) {
            continue;
        }

        if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
            let id = filename.to_string();
            let is_running = check_and_update_running_status(&app, &mut running, &id);

            list.push(ExtensionInfo {
                id,
                name: get_base_name(filename),
                version: get_version(filename),
                is_running,
            });
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
    let source = PathBuf::from(&source_path);
    if !source.exists() {
        return Err(format!("Source path '{}' does not exist", source_path));
    }

    let name = source.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Invalid source filename".to_string())?
        .to_string();

    let extensions_dir = get_extensions_dir(&app)?;
    fs::create_dir_all(&extensions_dir)
        .map_err(|e| format!("Failed to create extensions directory: {}", e))?;

    handle_single_version_constraint(&name, &extensions_dir)?;

    let target_path = extensions_dir.join(&name);

    if source.is_dir() {
        copy_dir_recursive(&source, &target_path)
            .map_err(|e| format!("Failed to copy directory extension: {}", e))?;
    } else {
        fs::copy(&source, &target_path)
            .map_err(|e| format!("Failed to copy extension file: {}", e))?;
    }

    #[cfg(unix)]
    if target_path.is_file() {
        set_permissions_unix(&target_path)?;
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

    if is_already_running(&mut running, &id) {
        return Err(format!("Extension '{}' is already running", id));
    }

    let path = get_extension_path(&app, &id)?;
    if !path.exists() {
        return Err(format!("Extension '{}' not found", id));
    }

    let child = spawn_extension_process(&path, &id)?;
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
    if let Some(child) = running.remove(&id) {
        terminate_extension(&app, &id, child);
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_extension<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    {
        let mut running = state.running_extensions.lock().unwrap();
        if let Some(child) = running.remove(&id) {
            terminate_extension(&app, &id, child);
        }
    }

    let path = get_extension_path(&app, &id)?;
    if path.exists() {
        if path.is_dir() {
            fs::remove_dir_all(&path).map_err(|e| format!("Failed to delete directory: {}", e))?;
        } else {
            fs::remove_file(&path).map_err(|e| format!("Failed to delete file: {}", e))?;
        }
    }

    Ok(())
}

pub fn cleanup_processes<R: Runtime>(app: &AppHandle<R>, state: &AppState) {
    let mut running = state.running_extensions.lock().unwrap();
    for (id, child) in running.drain() {
        terminate_extension(app, &id, child);
    }
}

// --- Helpers: Process Management ---

fn spawn_extension_process(path: &Path, id: &str) -> Result<Child, String> {
    #[cfg(target_os = "macos")]
    {
        if is_macos_app(path) {
            let mut cmd = Command::new("open");
            cmd.arg("-W").arg(path);
            return cmd.spawn().map_err(|e| format!("Failed to spawn extension '{}': {}", id, e));
        }
    }

    #[cfg(target_os = "windows")]
    {
        if is_windows_script(path) {
            let mut cmd = Command::new("cmd");
            cmd.arg("/C").arg(path);
            return cmd.spawn().map_err(|e| format!("Failed to spawn extension '{}': {}", id, e));
        }
    }

    Command::new(path)
        .spawn()
        .map_err(|e| format!("Failed to spawn extension '{}': {}", id, e))
}

#[cfg(target_os = "windows")]
fn is_windows_script(path: &Path) -> bool {
    path.extension()
        .map(|ext| {
            let ext = ext.to_string_lossy().to_lowercase();
            ext == "bat" || ext == "cmd"
        })
        .unwrap_or(false)
}

fn terminate_extension<R: Runtime>(app: &AppHandle<R>, id: &str, mut child: Child) {
    #[cfg(target_os = "macos")]
    if id.ends_with(".app") {
        if let Ok(path) = get_extension_path(app, id) {
            let _ = Command::new("osascript")
                .arg("-e")
                .arg(format!("quit app \"{}\"", path.display()))
                .spawn();
        }
    }
    let _ = child.kill();
}

fn check_and_update_running_status<R: Runtime>(
    app: &AppHandle<R>,
    running: &mut std::sync::MutexGuard<std::collections::HashMap<String, Child>>,
    id: &str,
) -> bool {
    if let Some(child) = running.get_mut(id) {
        match child.try_wait() {
            Ok(None) => return true,
            Ok(Some(status)) => {
                if !status.success() {
                    let name = get_base_name(id);
                    let _ = app.emit("extension-crash", format!("Extension '{}' exited with an error.", name));
                }
                running.remove(id);
            }
            Err(_) => {
                running.remove(id);
            }
        }
    }
    false
}

fn is_already_running(
    running: &mut std::sync::MutexGuard<std::collections::HashMap<String, Child>>,
    id: &str
) -> bool {
    if let Some(child) = running.get_mut(id) {
        return child.try_wait().map(|s| s.is_none()).unwrap_or(false);
    }
    false
}

// --- Helpers: Filesystem ---

fn get_extensions_dir<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|mut p| {
            p.push("Extensions");
            p
        })
        .map_err(|e| format!("Failed to get app data directory: {}", e))
}

fn get_extension_path<R: Runtime>(app: &AppHandle<R>, id: &str) -> Result<PathBuf, String> {
    Ok(get_extensions_dir(app)?.join(id))
}

fn is_valid_extension_item(path: &Path) -> bool {
    path.is_file() || is_macos_app(path)
}

fn is_macos_app(path: &Path) -> bool {
    #[cfg(target_os = "macos")]
    return path.is_dir() && path.extension().map_or(false, |e| e == "app");
    
    #[cfg(not(target_os = "macos"))]
    false
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
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

#[cfg(unix)]
fn set_permissions_unix(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path)
        .map_err(|e| format!("Failed to get metadata: {}", e))?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms)
        .map_err(|e| format!("Failed to set permissions: {}", e))
}

// --- Helpers: Naming & Versioning ---

fn handle_single_version_constraint(name: &str, extensions_dir: &Path) -> Result<(), String> {
    let base_name = get_base_name(name);
    if extensions_dir.exists() {
        for entry in fs::read_dir(extensions_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if get_base_name(filename) == base_name {
                    if path.is_dir() { fs::remove_dir_all(&path).ok(); }
                    else { fs::remove_file(&path).ok(); }
                }
            }
        }
    }
    Ok(())
}

fn get_version(filename: &str) -> String {
    let stem = Path::new(filename).file_stem().and_then(|s| s.to_str()).unwrap_or(filename);
    if let Some(index) = stem.rfind(VERSION_SEPARATOR) {
        stem[index + VERSION_SEPARATOR.len()..].trim().to_string()
    } else {
        "unknown".to_string()
    }
}

fn get_base_name(filename: &str) -> String {
    let stem = Path::new(filename).file_stem().and_then(|s| s.to_str()).unwrap_or(filename);
    if let Some(index) = stem.rfind(VERSION_SEPARATOR) {
        stem[..index].trim().to_string()
    } else {
        stem.to_string()
    }
}
