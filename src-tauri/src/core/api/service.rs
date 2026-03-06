use serde::Serialize;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use base64::{engine::general_purpose, Engine as _};
use rand::{rng, Rng};
use tauri::{AppHandle, Manager};
use crate::core::keychain::KeychainService;

#[derive(Serialize)]
pub struct TokenResponse {
    pub ciphertext: String,
    pub nonce: String,
}

#[derive(Serialize)]
pub struct ConfigResponse {
    pub server_url: String,
}

pub struct TokenService;

impl TokenService {
    pub async fn get_encrypted_token() -> Result<TokenResponse, String> {
        let master_key_bytes = KeychainService::get_or_create_master_key()?;
        
        let session = KeychainService::get_session()
            .map_err(|_| "No active session found. Please login.".to_string())?;
        
        let key = Aes256Gcm::new_from_slice(&master_key_bytes)
            .map_err(|e| format!("Encryption key error: {}", e))?;
        
        let mut nonce_bytes = [0u8; 12];
        rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = key
            .encrypt(nonce, session.access_token.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        Ok(TokenResponse {
            ciphertext: general_purpose::STANDARD.encode(ciphertext),
            nonce: general_purpose::STANDARD.encode(nonce_bytes),
        })
    }
}

pub struct ConfigService;

impl ConfigService {
    pub fn get_config(handle: &AppHandle) -> Result<ConfigResponse, String> {
        let path = handle.path().app_config_dir()
            .map_err(|e| e.to_string())?
            .join("settings.json");

        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config: {}", e))?;

        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        let server_url = json.get("server_url")
            .and_then(|v| v.as_str())
            .ok_or("server_url not found in config")?
            .to_string();

        Ok(ConfigResponse { server_url })
    }
}
