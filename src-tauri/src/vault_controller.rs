use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use snafu::{Location, Snafu};
use std::fs::{self};
use std::path::{Path, PathBuf};
use tauri::ipc::InvokeError;

use crate::config::Config;
use crate::passwords::Passwords;
use crate::vault::{Vault, VaultError};

pub struct VaultController;

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultMetadata {
    pub created_at: DateTime<Local>,
    pub last_accessed: DateTime<Local>,
}

#[derive(Debug, Snafu)]
pub enum VaultControllerError {
    #[snafu(display("IO Error, loc: {location}"))]
    #[snafu(context(false))]
    IO {
        source: std::io::Error,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Vault error"))]
    #[snafu(context(false))]
    Vault {
        source: VaultError,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Vault file '{:?}' already exists", file_name))]
    VaultFileExists { file_name: String },

    #[snafu(display("Vault file '{:?}' does not exist", file_name))]
    VaultFileNotFound { file_name: String },

    #[snafu(display("Invalid vault name: {}", reason))]
    InvalidVaultName { reason: String },

    #[snafu(display("Cannot rename to '{}': vault already exists", name))]
    RenameTargetExists { name: String },

    #[snafu(display("Master key must be at least {} characters long", min_length))]
    MasterKeyTooShort { min_length: usize },

    #[snafu(display("Master key should not contain space in the beginning or the end"))]
    MasterKeySpaceAtEnds,

    #[snafu(display("Confirm master key does not match"))]
    ConfirmMasterKeyMismatch,

    #[snafu(display("New master key is the same as the current master key"))]
    NewMasterKeySameValue,

    #[snafu(display("Vault name cannot be empty"))]
    VaultNameEmpty,
}

impl From<VaultControllerError> for InvokeError {
    fn from(val: VaultControllerError) -> Self {
        let message = match val {
            VaultControllerError::IO {
                source: _,
                location: _,
            } => "Unexpected io error occured!".to_string(),
            VaultControllerError::Vault {
                source,
                location: _,
            } => match source {
                VaultError::IO {
                    source: _,
                    location: _,
                } => "Unexpected io error occured!".to_string(),
                VaultError::Cipher {
                    source: _,
                    location: _,
                } => "Incorrect master key".to_string(),
                VaultError::VaultMeta {
                    source: _,
                    location: _,
                } => "Unexpected metadata error occured!".to_string(),
            },
            _ => val.to_string(),
        };
        InvokeError::from(message)
    }
}

impl VaultController {
    pub fn create_vault(
        config: &Config,
        name: &str,
        masterkey: &str,
        confirm_masterkey: &str,
    ) -> Result<Vault, VaultControllerError> {
        let name = name.trim();
        Self::validate_name(name)?;
        Self::validate_masterkey(masterkey, confirm_masterkey)?;

        // Create empty encrypted vault content
        let vault = Vault::new(config, name.to_string())?;
        Self::validate_path_not_exist(&vault.file_path)?;

        let passwords = Passwords::empty();
        vault.save_to_file(masterkey, passwords)?;

        Ok(vault)
    }

    pub fn list(config: &Config) -> Result<Vec<Vault>, VaultControllerError> {
        let entries = fs::read_dir(config.app_data_dir.clone())?;

        let mut vaults: Vec<Vault> = entries
            .flatten()
            .flat_map(|e| e.file_name().to_str().map(|s| s.to_string()))
            .filter(|name| name.ends_with(".pwd"))
            .map(|name| {
                let i = name.len() - 4;
                name[..i].to_string()
            })
            .flat_map(|name| Vault::new(config, name).ok())
            .collect();

        vaults.sort_by(|a, b| b.metadata.created_at.cmp(&a.metadata.created_at));
        Ok(vaults)
    }

    pub fn delete(config: &Config, name: &str) -> Result<(), VaultControllerError> {
        let vault = Vault::new(config, name.to_string())?;
        vault.delete_file()?;
        Ok(())
    }

    pub fn update_masterkey(
        config: &Config,
        name: &str,
        old_masterkey: &str,
        new_masterkey: &str,
        confirm_new_masterkey: &str,
    ) -> Result<(), VaultControllerError> {
        Self::validate_new_masterkey(old_masterkey, new_masterkey, confirm_new_masterkey)?;
        let vault = Vault::new(config, name.to_string())?;
        vault.update_masterkey(old_masterkey, new_masterkey)?;
        Ok(())
    }

    pub fn rename(
        config: &Config,
        name: &str,
        new_name: &str,
    ) -> Result<Vault, VaultControllerError> {
        let new_name = new_name.trim();
        Self::validate_name(new_name)?;

        let old_vault = Vault::new(config, name.to_string())?;
        let new_vault = Vault::new(config, new_name.to_string())?;

        Self::validate_path_exist(&old_vault.file_path)?;
        Self::validate_path_not_exist(&new_vault.file_path)?;

        // Rename the file
        fs::rename(&old_vault.file_path, &new_vault.file_path)?;
        fs::rename(&old_vault.metadata.file_path, &new_vault.metadata.file_path)?;

        Ok(new_vault)
    }

    fn validate_new_masterkey(
        old_masterkey: &str,
        new_masterkey: &str,
        confirm_masterkey: &str,
    ) -> Result<(), VaultControllerError> {
        if old_masterkey == new_masterkey {
            return Err(VaultControllerError::NewMasterKeySameValue);
        }

        Self::validate_masterkey(new_masterkey, confirm_masterkey)
    }

    fn validate_masterkey(
        masterkey: &str,
        confirm_masterkey: &str,
    ) -> Result<(), VaultControllerError> {
        const MIN_LENGTH: usize = 12;

        if masterkey != confirm_masterkey {
            return Err(VaultControllerError::ConfirmMasterKeyMismatch);
        }

        if masterkey.trim().len() != masterkey.len() {
            return Err(VaultControllerError::MasterKeySpaceAtEnds);
        }

        if masterkey.len() < MIN_LENGTH {
            return Err(VaultControllerError::MasterKeyTooShort {
                min_length: MIN_LENGTH,
            });
        }

        Ok(())
    }

    fn validate_name(name: &str) -> Result<(), VaultControllerError> {
        if name.trim().is_empty() {
            return Err(VaultControllerError::VaultNameEmpty);
        }

        Ok(())
    }

    fn validate_path_not_exist(path: &Path) -> Result<(), VaultControllerError> {
        if path.exists() {
            let file_name = path.file_name().unwrap();
            let file_name = file_name.to_str().unwrap().to_string();
            return Err(VaultControllerError::VaultFileExists { file_name });
        }

        Ok(())
    }

    fn validate_path_exist(path: &Path) -> Result<(), VaultControllerError> {
        if !path.exists() {
            let file_name = path.file_name().unwrap();
            let file_name = file_name.to_str().unwrap().to_string();
            return Err(VaultControllerError::VaultFileNotFound { file_name });
        }

        Ok(())
    }
}
