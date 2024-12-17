use borsh::{BorshDeserialize, BorshSerialize};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use snafu::{Location, Snafu};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use tauri::ipc::InvokeError;

use crate::cipher::{Cipher, CipherError};
use crate::config::Config;
use crate::passwords::Passwords;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    pub name: String,
    pub metadata: VaultMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultMetadata {
    pub created_at: DateTime<Local>,
    pub last_accessed: DateTime<Local>,
}

#[derive(Debug, Snafu)]
pub enum VaultError {
    #[snafu(display("IO Error, loc: {location}"))]
    #[snafu(context(false))]
    IO {
        source: std::io::Error,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Serde Error, loc: {location}"))]
    #[snafu(context(false))]
    Serde {
        source: serde_json::Error,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Cipher Error"))]
    #[snafu(context(false))]
    Cipher {
        source: CipherError,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Vault file '{:?}' already exists", path))]
    VaultFileExists { path: PathBuf },

    #[snafu(display("Vault file '{:?}' does not exist", path))]
    VaultFileNotFound { path: PathBuf },

    #[snafu(display("Invalid vault name: {}", reason))]
    InvalidVaultName { reason: String },

    #[snafu(display("Cannot rename to '{}': vault already exists", name))]
    RenameTargetExists { name: String },

    #[snafu(display("Master key must be at least {} characters long", min_length))]
    MasterKeyTooShort { min_length: usize },

    #[snafu(display("Master key should not contain space in the beginning or the end"))]
    MasterKeyTrimSpace,

    #[snafu(display("Vault name cannot be empty"))]
    NameEmpty,

    #[snafu(display("Failed to get app directory"))]
    AppDirAccess,
}

impl From<VaultError> for InvokeError {
    fn from(val: VaultError) -> Self {
        InvokeError::from(val.to_string())
    }
}

impl Vault {
    pub fn new(config: &Config, name: &str, masterkey: &str) -> Result<Self, VaultError> {
        let name = name.trim();
        Self::validate_name(name)?;
        Self::validate_masterkey(masterkey)?;

        // Create empty encrypted vault content
        let pwd_path = Self::get_pwd_path(config, name);
        Self::validate_path_not_exist(&pwd_path)?;

        let cipher = Cipher::new(masterkey);
        let file = File::create(pwd_path)?;
        let mut writer = BufWriter::new(file);
        cipher.dump(Passwords::empty().try_to_vec()?, &mut writer)?;

        // Get or create metadata
        let metadata = Self::get_or_create_metadata(config, name)?;

        Ok(Vault {
            name: name.to_string(),
            metadata,
        })
    }

    pub fn list(config: &Config) -> Result<Vec<Self>, VaultError> {
        let entries = fs::read_dir(config.app_data_dir.clone())?;

        let mut vaults: Vec<Vault> = entries
            .flatten()
            .flat_map(|e| e.file_name().to_str().map(|s| s.to_string()))
            .filter(|name| name.ends_with(".pwd"))
            .flat_map(|name| {
                let end_index = name.len() - 4;
                let name = &name[..end_index];
                Vault::from_name(config, name).ok()
            })
            .collect();

        vaults.sort_by(|a, b| b.metadata.created_at.cmp(&a.metadata.created_at));
        Ok(vaults)
    }

    pub fn delete(config: &Config, name: &str) -> Result<(), VaultError> {
        let pwd_path = Self::get_pwd_path(config, name);
        let meta_path = Self::get_meta_path(config, name);

        if pwd_path.exists() {
            fs::remove_file(&pwd_path)?;
        }

        if meta_path.exists() {
            fs::remove_file(&meta_path)?;
        }
        Ok(())
    }

    pub fn update(
        config: &Config,
        name: &str,
        old_masterkey: &str,
        new_masterkey: &str,
    ) -> Result<(), VaultError> {
        Self::validate_masterkey(new_masterkey)?;

        let old_cipher = Cipher::new(old_masterkey);
        let pwd_path = Self::get_pwd_path(config, name);
        let file = File::open(pwd_path)?;
        let mut reader = BufReader::new(file);
        let encoded = old_cipher.parse(&mut reader)?;
        let decoded = Passwords::try_from_slice(&encoded)?;

        let new_cipher = Cipher::new(new_masterkey);
        let pwd_path = Self::get_pwd_path(config, name);
        let file = File::create(pwd_path)?;
        let mut writer = BufWriter::new(file);
        new_cipher.dump(decoded.try_to_vec()?, &mut writer)?;

        Ok(())
    }

    pub fn rename(config: &Config, name: &str, new_name: &str) -> Result<Vault, VaultError> {
        let new_name = new_name.trim();
        Self::validate_name(new_name)?;

        let old_pwd_path = Self::get_pwd_path(config, name);
        let new_pwd_path = Self::get_pwd_path(config, new_name);
        let old_meta_path = Self::get_meta_path(config, name);
        let new_meta_path = Self::get_meta_path(config, new_name);

        // Rename the file
        fs::rename(&old_pwd_path, &new_pwd_path)?;
        fs::rename(&old_meta_path, &new_meta_path)?;

        Vault::from_name(config, new_name)
    }

    fn from_name(config: &Config, name: &str) -> Result<Self, VaultError> {
        Self::validate_name(name)?;
        let metadata = Self::get_or_create_metadata(config, name)?;

        Ok(Vault {
            name: name.to_string(),
            metadata,
        })
    }

    fn get_or_create_metadata(config: &Config, name: &str) -> Result<VaultMetadata, VaultError> {
        let pwd_path = Self::get_pwd_path(config, name);
        Self::validate_path_exist(&pwd_path)?;

        let meta_path = Self::get_meta_path(config, name);
        if meta_path.exists() {
            let file = File::open(meta_path)?;
            let metadata = serde_json::from_reader(file)?;

            Ok(metadata)
        } else {
            let now = Local::now();
            let metadata_path = Self::get_meta_path(config, name);
            let metadata = VaultMetadata {
                created_at: now,
                last_accessed: now,
            };
            let metadata_str = serde_json::to_string(&metadata)?;
            fs::write(&metadata_path, metadata_str)?;

            Ok(metadata)
        }
    }

    fn get_pwd_path(config: &Config, name: &str) -> PathBuf {
        config.app_data_dir.join(format!("{}.pwd", name))
    }

    fn get_meta_path(config: &Config, name: &str) -> PathBuf {
        config.app_data_dir.join(format!("{}.meta", name))
    }

    fn validate_masterkey(masterkey: &str) -> Result<(), VaultError> {
        const MIN_LENGTH: usize = 12;

        if masterkey.trim().len() != masterkey.len() {
            return Err(VaultError::MasterKeyTrimSpace);
        }

        if masterkey.len() < MIN_LENGTH {
            return Err(VaultError::MasterKeyTooShort {
                min_length: MIN_LENGTH,
            });
        }

        Ok(())
    }

    fn validate_name(name: &str) -> Result<(), VaultError> {
        if name.trim().is_empty() {
            return Err(VaultError::NameEmpty);
        }

        Ok(())
    }

    fn validate_path_not_exist(path: &Path) -> Result<(), VaultError> {
        if path.exists() {
            return Err(VaultError::VaultFileExists {
                path: path.to_path_buf(),
            });
        }

        Ok(())
    }

    fn validate_path_exist(path: &Path) -> Result<(), VaultError> {
        if !path.exists() {
            return Err(VaultError::VaultFileNotFound {
                path: path.to_path_buf(),
            });
        }

        Ok(())
    }
}
