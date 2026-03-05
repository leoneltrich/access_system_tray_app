use keyring::Entry;
use serde::{Deserialize, Serialize};
use rand::{rng, Rng};
use base64::{engine::general_purpose, Engine as _};

const SERVICE_SESSION: &str = "ServeMe_Session";
const SERVICE_INTERNAL: &str = "ServeMe_Internal";
const ACCOUNT_NAME: &str = "current_session";
const KEY_NAME: &str = "master_key";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct KeychainService;

impl KeychainService {
    pub fn save_session(access: &str, refresh: &str) -> Result<(), String> {
        let session = Session {
            access_token: access.to_string(),
            refresh_token: refresh.to_string(),
        };
        let json = serde_json::to_string(&session).map_err(|e| e.to_string())?;
        
        let entry = Entry::new(SERVICE_SESSION, ACCOUNT_NAME).map_err(|e| e.to_string())?;
        entry.set_password(&json).map_err(|e| e.to_string())
    }

    pub fn get_session() -> Result<Session, String> {
        let entry = Entry::new(SERVICE_SESSION, ACCOUNT_NAME).map_err(|e| e.to_string())?;
        let json = entry.get_password().map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }

    pub fn purge_session() -> Result<(), String> {
        let entry = Entry::new(SERVICE_SESSION, ACCOUNT_NAME).map_err(|e| e.to_string())?;
        match entry.delete_credential() {
            Ok(_) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_or_create_master_key() -> Result<Vec<u8>, String> {
        let entry = Entry::new(SERVICE_INTERNAL, KEY_NAME).map_err(|e| e.to_string())?;
        
        match entry.get_password() {
            Ok(base64_key) => {
                general_purpose::STANDARD.decode(base64_key).map_err(|e| e.to_string())
            }
            Err(keyring::Error::NoEntry) => {
                let mut key = [0u8; 32];
                rng().fill_bytes(&mut key);
                let base64_key = general_purpose::STANDARD.encode(key);
                
                entry.set_password(&base64_key).map_err(|e| e.to_string())?;
                Ok(key.to_vec())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
