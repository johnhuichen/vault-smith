use borsh::BorshSerialize;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use snafu::{Location, ResultExt, Snafu};
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use tauri::ipc::InvokeError;

use crate::cipher::{Cipher, CipherError};
use crate::config::Config;
use crate::passwords::Passwords;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    pub name: String,
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

    #[snafu(display("Cipher Error"))]
    Cipher { source: CipherError },

    #[snafu(display("Failed to get app directory"))]
    AppDirAccess,
}

impl From<VaultError> for InvokeError {
    fn from(val: VaultError) -> Self {
        InvokeError::from(val.to_string())
    }
}

impl Vault {
    pub fn new(name: String) -> Result<Self, VaultError> {
        Self::validate_name(&name)?;
        Ok(Vault { name })
    }

    pub fn create(&self, masterkey: &str, config: &Config) -> Result<(), VaultError> {
        Self::validate_masterkey(masterkey)?;
        let pwd_path = self.get_pwd_path(config);
        Self::validate_path_not_exist(&pwd_path)?;

        // Create metadata
        let now = Local::now();
        let metadata_path = self.get_meta_path(config);
        let metadata = VaultMetadata {
            created_at: now,
            last_accessed: now,
        };

        // Save metadata to separate file
        let metadata = serde_json::to_string(&metadata)?;
        fs::write(&metadata_path, metadata)?;

        // Create empty encrypted vault content
        let cipher = Cipher::new(masterkey.to_string());
        let file = File::create(pwd_path)?;
        let mut writer = BufWriter::new(file);
        cipher
            .dump(Passwords::random().try_to_vec()?, &mut writer)
            .context(CipherSnafu)?;

        Ok(())
    }

    fn get_pwd_path(&self, config: &Config) -> PathBuf {
        config.app_data_dir.join(format!("{}.pwd", self.name))
    }

    fn get_meta_path(&self, config: &Config) -> PathBuf {
        config.app_data_dir.join(format!("{}.meta", self.name))
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

    // pub fn from_file_name(file_name: &str) -> Option<Self> {
    //     if !file_name.ends_with(".pwd") {
    //         return None;
    //     }
    //
    //     let name = file_name.strip_suffix(".pwd")?.to_string();
    //     let vaults_dir = get_vaults_dir().ok()?;
    //     let vault_path = vaults_dir.join(file_name);
    //     let metadata_path = Self::get_metadata_path(&vault_path);
    //
    //     None
    //     // // Read metadata if it exists
    //     // let metadata = if metadata_path.exists() {
    //     //     serde_json::from_reader(rdr)
    //     //     fs::read_to_string(&metadata_path)
    //     //         .ok()
    //     //         .and_then(|content| serde_json::from_str::<VaultMetadata>(&content).ok())
    //     // } else {
    //     //     None
    //     // };
    //     //
    //     // let last_accessed = metadata
    //     //     .map(|m| m.last_accessed.format("%Y-%m-%d").to_string())
    //     //     .unwrap_or_else(|| "Never".to_string());
    //     //
    //     // Some(Vault {
    //     //     id: name.clone(),
    //     //     name,
    //     //     last_accessed,
    //     // })
    // }
    //
    // pub fn update_last_accessed(&self) -> Result<(), VaultError> {
    //     let vaults_dir = get_vaults_dir()?;
    //     let vault_path = vaults_dir.join(format!("{}.pwd", self.name));
    //     let metadata_path = Self::get_metadata_path(&vault_path);
    //
    //     // Read existing metadata or create new
    //     let mut metadata = if metadata_path.exists() {
    //         let content = fs::read_to_string(&metadata_path)
    //             .map_err(|source| VaultError::ReadError { source })?;
    //         serde_json::from_str(&content).map_err(|_| VaultError::MetadataSerialize)?
    //     } else {
    //         VaultMetadata {
    //             created_at: Local::now(),
    //             last_accessed: Local::now(),
    //         }
    //     };
    //
    //     // Update last_accessed
    //     metadata.last_accessed = Local::now();
    //
    //     // Save updated metadata
    //     fs::write(
    //         &metadata_path,
    //         serde_json::to_string(&metadata).map_err(|_| VaultError::MetadataSerialize)?,
    //     )
    //     .map_err(|source| VaultError::WriteError { source })?;
    //
    //     Ok(())
    // }
}
