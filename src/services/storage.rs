use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use aes_gcm::AeadCore;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::Result;
use gpui::{App, Global};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::errors::StorageError;

#[derive(Serialize, Deserialize)]
struct EncryptedData {
    nonce: Vec<u8>,
    data: Vec<u8>,
}

#[derive(Clone)]
pub struct Storage {
    data: Arc<Mutex<HashMap<String, String>>>,
    path: PathBuf,
    cipher: Aes256Gcm,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum StorageKey {
    // auth
    AuthEmail,
    AuthUserId,
    AuthAccessToken,
    AuthAccessTokenExpiresAt,
    AuthRefreshToken,

    // assistant
    AssistantId,
    AssistantModel,
    CustomAssistants,
    // provider specific
    AnthropicApiKey,
    AssistantOpenAiApiKey,

    // settings
    SettingsTheme,
    SettingsFontSize,
    SttingsTransparency,
    SettingsStartOnLogin,

    // shortcuts
    ShortcutsToogleVisibility,
    ShortcutsRunAssistant,
    ShortcutsNextPrompt,
    ShortcutsPrevPropmt,
}

impl StorageKey {
    pub fn stringify(&self) -> String {
        let str = self.to_string();

        // snake_case notation
        str.chars().fold(String::new(), |mut str, char| {
            if char.is_uppercase() || char.is_numeric() {
                if !str.is_empty() {
                    str.push('_');
                }
                str.push(char.to_ascii_lowercase());
            } else {
                str.push(char);
            }
            str
        })
    }
}

impl fmt::Display for StorageKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Storage {
    pub fn init(cx: &mut App) {
        let storage_salt = env!("STORAGE_SALT");

        if storage_salt.is_empty() {
            println!("Storage salt is not set as env variable. Storage won't be initalized.");
            // TODO: set state error
            return;
        }

        let mut app_dir = dirs::home_dir().expect("Could not find home directory");
        app_dir.push(".probee/storage.db");

        let storage = Storage::new(app_dir, storage_salt.as_bytes()).unwrap();

        cx.set_global(storage);
    }

    pub fn new(path: PathBuf, salt: &[u8]) -> Result<Self> {
        // Create the storage directory if it doesn't exist
        fs::create_dir_all(path.parent().unwrap())?;

        // Create a 32-byte key by hashing the salt
        let hash = Sha256::digest(salt);
        // Create a cipher using the provided hashed salt
        let cipher =
            Aes256Gcm::new_from_slice(&hash).map_err(|_| StorageError::StorageCreationError)?;

        let data = Arc::new(Mutex::new(HashMap::new()));
        let store = Self { data, path, cipher };

        // Load existing data if available
        store.load()?;

        Ok(store)
    }

    pub fn set(&self, key: StorageKey, value: String) -> Result<()> {
        {
            let mut data = self.data.lock().unwrap();
            data.insert(key.stringify(), value);
        }

        self.flush()?;

        Ok(())
    }

    pub fn get(&self, key: StorageKey) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(&key.stringify()).cloned()
    }

    pub fn delete(&self, key: StorageKey) -> Result<()> {
        {
            let mut data = self.data.lock().unwrap();
            data.remove(&key.stringify());
        }

        self.flush()?;

        Ok(())
    }

    // TODO: Implement non blocking flush
    fn flush(&self) -> Result<()> {
        let data = self.data.lock().unwrap();
        let serialized = serde_json::to_string(&*data)?;

        // Generate a random nonce
        let nonce = Aes256Gcm::generate_nonce(&mut rand::thread_rng());

        // Encrypt the data
        let encrypted = self
            .cipher
            .encrypt(&nonce, serialized.as_bytes())
            .map_err(|err| StorageError::Encryption(err))?;

        let encrypted_data = EncryptedData {
            nonce: nonce.to_vec(),
            data: encrypted,
        };

        // Serialize the encrypted data to JSON
        let final_data = serde_json::to_string(&encrypted_data)?;

        // Write to temporary file first
        let temp_path = self.path.with_extension("tmp");
        let mut file = File::create(&temp_path)?;
        file.write_all(final_data.as_bytes())?;
        file.sync_all()?;

        // Rename temporary file to actual file
        fs::rename(temp_path, &self.path)?;

        Ok(())
    }

    fn load(&self) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }

        let mut file = File::open(&self.path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let encrypted_data: EncryptedData = serde_json::from_str(&contents)?;

        // Create nonce from stored data
        let nonce = Nonce::from_slice(&encrypted_data.nonce);

        // Decrypt the data
        let decrypted = self
            .cipher
            .decrypt(nonce, encrypted_data.data.as_ref())
            .map_err(|_| StorageError::Decryption)?;

        // Convert decrypted bytes to string and parse JSON
        let decrypted_str = String::from_utf8(decrypted).map_err(|_| StorageError::Decryption)?;
        let loaded_data: HashMap<String, String> = serde_json::from_str(&decrypted_str)?;

        let mut data = self.data.lock().unwrap();
        *data = loaded_data;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_basic_operations() -> Result<()> {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.db");
        let salt = b"an-example-very-very-secret-key-32";

        let store = Storage::new(path.clone(), salt)?;

        // Test set and get
        store.set(StorageKey::AnthropicApiKey, "api_key_value_123".to_string())?;
        assert_eq!(
            store.get(StorageKey::AnthropicApiKey),
            Some("api_key_value_123".to_string())
        );

        // Test delete
        store.delete(StorageKey::AnthropicApiKey)?;
        assert_eq!(store.get(StorageKey::AnthropicApiKey), None);

        Ok(())
    }

    #[test]
    fn test_persistence() -> Result<()> {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.db");
        let salt = b"an-example-very-very-secret-key-32";

        // Create store and add data
        {
            let store = Storage::new(path.clone(), salt)?;
            store.set(StorageKey::SettingsFontSize, "small".to_string())?;
        }

        // Create new store instance and verify data
        let store2 = Storage::new(path.clone(), salt)?;
        assert_eq!(
            store2.get(StorageKey::SettingsFontSize),
            Some("small".to_string())
        );

        Ok(())
    }
}

impl Global for Storage {}
