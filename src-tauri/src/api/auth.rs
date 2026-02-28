use keyring::Entry;
use serde::{Deserialize, Serialize};
const SERVICE_ACCESS: &str = "ServeMe_Access";
const SERVICE_REFRESH: &str = "ServeMe_Refresh";

const ACCOUNT_NAME: &str = "current_session";

#[derive(Deserialize, Serialize)]
pub struct TokenSet {
    access: String,
    refresh: String,
}

#[tauri::command]
pub async fn save_tokens(access_token: &str, refresh_token: &str) -> Result<(), String> {
    let access_entry = Entry::new(SERVICE_ACCESS, ACCOUNT_NAME).map_err(|e| e.to_string())?;
    access_entry
        .set_password(access_token)
        .map_err(|e| e.to_string())?;

    let refresh_entry = Entry::new(SERVICE_REFRESH, ACCOUNT_NAME).map_err(|e| e.to_string())?;
    refresh_entry
        .set_password(refresh_token)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_tokens() -> Result<TokenSet, String> {
    let access_entry = Entry::new(SERVICE_ACCESS, ACCOUNT_NAME).map_err(|e| e.to_string())?;
    let access_token = access_entry.get_password().map_err(|e| e.to_string())?;

    let refresh_entry = Entry::new(SERVICE_REFRESH, ACCOUNT_NAME).map_err(|e| e.to_string())?;
    let refresh_token = refresh_entry.get_password().map_err(|e| e.to_string())?;
    Ok(TokenSet {
        access: access_token,
        refresh: refresh_token,
    })
}

#[tauri::command]
pub async fn get_refresh_token() -> Result<String, String> {
    let refresh_entry = Entry::new(SERVICE_REFRESH, ACCOUNT_NAME).map_err(|e| e.to_string())?;
    let refresh_token = refresh_entry.get_password().map_err(|e| e.to_string())?;

    Ok(refresh_token)
}

#[tauri::command]
pub async fn get_access_token() -> Result<String, String> {
    let access_entry = Entry::new(SERVICE_ACCESS, ACCOUNT_NAME).map_err(|e| e.to_string())?;
    let access_token = access_entry.get_password().map_err(|e| e.to_string())?;

    Ok(access_token)
}

#[tauri::command]
pub async fn purge_tokens() -> Result<(), String> {
    let access_entry = Entry::new(SERVICE_ACCESS, ACCOUNT_NAME).map_err(|e| e.to_string())?;
    let _ = access_entry.delete_credential().map_err(|e| e.to_string())?;

    let refresh_entry = Entry::new(SERVICE_REFRESH, ACCOUNT_NAME).map_err(|e| e.to_string())?;
    let _ = refresh_entry.delete_credential().map_err(|e| e.to_string())?;
    Ok(())
}