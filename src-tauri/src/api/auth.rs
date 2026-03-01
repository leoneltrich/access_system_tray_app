use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use keyring::{Entry, Error as KeyringError};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

const SERVICE_ACCESS: &str = "ServeMe_Access";
const SERVICE_REFRESH: &str = "ServeMe_Refresh";
const ACCOUNT_NAME: &str = "current_session";

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenSet {
    pub access: String,
    pub refresh: String,
}

#[derive(Deserialize)]
struct RefreshResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
struct Config {
    server_url: String,
    username: String,
}

#[derive(Deserialize)]
struct Claims {
    exp: i64,
}

pub fn spawn_background_refresh(handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(300)).await;
            let _ = refresh_if_needed(&handle).await.map_err(|e| {
                eprintln!("[Background Refresh] Failed: {}", e);
            });
        }
    });
}

async fn refresh_if_needed(handle: &AppHandle) -> Result<(), String> {
    let tokens = match get_tokens().await {
        Ok(t) => t,
        Err(_) => return Ok(()),
    };

    if !should_refresh(&tokens.access)? {
        return Ok(());
    }

    let config = load_config(handle)?;
    let response = request_new_tokens(&config, &tokens.refresh).await?;
    
    save_tokens(&response.access_token, &response.refresh_token).await?;

    let new_tokens = TokenSet {
        access: response.access_token,
        refresh: response.refresh_token,
    };
    let _ = handle.emit("tokens-refreshed", new_tokens);

    Ok(())
}

fn should_refresh(access_token: &str) -> Result<bool, String> {
    let parts: Vec<&str> = access_token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid JWT".into());
    }

    let payload = general_purpose::URL_SAFE_NO_PAD
        .decode(parts[1])
        .map_err(|e| e.to_string())?;

    let claims: Claims = serde_json::from_slice(&payload).map_err(|e| e.to_string())?;
    let now = Utc::now().timestamp();
    
    Ok(claims.exp - now <= 600)
}

fn load_config(handle: &AppHandle) -> Result<Config, String> {
    let path = handle.path().app_config_dir()
        .map_err(|e| e.to_string())?
        .join("settings.json");
    
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

async fn request_new_tokens(config: &Config, refresh_token: &str) -> Result<RefreshResponse, String> {
    let url = format!("{}/api/v1/token/refresh", config.server_url.trim_end_matches('/'));
    
    reqwest::Client::new()
        .post(url)
        .json(&serde_json::json!({
            "username": config.username,
            "refresh_token": refresh_token
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<RefreshResponse>()
        .await
        .map_err(|e| e.to_string())
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
    match access_entry.delete_credential() {
        Ok(_) => {}
        Err(KeyringError::NoEntry) => {}
        Err(e) => return Err(e.to_string()),
    }

    let refresh_entry = Entry::new(SERVICE_REFRESH, ACCOUNT_NAME).map_err(|e| e.to_string())?;
    match refresh_entry.delete_credential() {
        Ok(_) => {}
        Err(KeyringError::NoEntry) => {}
        Err(e) => return Err(e.to_string()),
    }

    Ok(())
}
