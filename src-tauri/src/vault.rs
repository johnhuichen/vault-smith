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
    pub fn new(name: String, masterkey: String, config: &Config) -> Result<Self, VaultError> {
        Self::validate_masterkey(&masterkey)?;

        // Create empty encrypted vault content
        let pwd_path = Self::get_pwd_path(&name, config);
        Self::validate_path_not_exist(&pwd_path)?;

        let cipher = Cipher::new(masterkey.to_string());
        let file = File::create(pwd_path)?;
        let mut writer = BufWriter::new(file);
        cipher
            .dump(Passwords::random().try_to_vec()?, &mut writer)
            .context(CipherSnafu)?;

        // Get or create metadata
        let metadata = Self::get_metadata(&name, config)?;

        Ok(Vault { name, metadata })
    }

    pub fn list(config: &Config) -> Result<Vec<Self>, VaultError> {
        let entries = fs::read_dir(config.app_data_dir.clone())?;

        let mut vaults: Vec<Vault> = entries
            .flatten()
            .flat_map(|e| e.file_name().to_str().map(|s| s.to_string()))
            .filter(|name| name.ends_with(".pwd"))
            .flat_map(|name| {
                let end_index = name.len() - 4;
                let name = String::from(&name[..end_index]);
                Vault::from_name(name, config).ok()
            })
            .collect();

        vaults.sort_by(|a, b| b.metadata.created_at.cmp(&a.metadata.created_at));
        Ok(vaults)
    }

    fn from_name(name: String, config: &Config) -> Result<Self, VaultError> {
        let metadata = Self::get_metadata(&name, config)?;

        Ok(Vault { name, metadata })
    }

    fn get_metadata(name: &str, config: &Config) -> Result<VaultMetadata, VaultError> {
        let pwd_path = Self::get_pwd_path(name, config);
        Self::validate_path_exist(&pwd_path)?;

        let meta_path = Self::get_meta_path(name, config);
        if meta_path.exists() {
            let file = File::open(meta_path)?;
            let metadata = serde_json::from_reader(file)?;

            Ok(metadata)
        } else {
            let now = Local::now();
            let metadata_path = Self::get_meta_path(name, config);
            let metadata = VaultMetadata {
                created_at: now,
                last_accessed: now,
            };
            let metadata_str = serde_json::to_string(&metadata)?;
            fs::write(&metadata_path, metadata_str)?;

            Ok(metadata)
        }
    }

    fn get_pwd_path(name: &str, config: &Config) -> PathBuf {
        config.app_data_dir.join(format!("{}.pwd", name))
    }

    fn get_meta_path(name: &str, config: &Config) -> PathBuf {
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
