use tauri::State;

use crate::config::Config;
use crate::passwords::Passwords;
use crate::vault::{Vault, VaultError};

use snafu::{Location, Snafu};
use std::fs::{self};
use std::path::Path;
use tauri::ipc::InvokeError;

#[derive(Debug, Snafu)]
pub enum VaultCommandsError {
    #[snafu(display("IO Error, loc: {location}"))]
    #[snafu(context(false))]
    IO {
        source: std::io::Error,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Vault error, loc: {location}"))]
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

impl From<VaultCommandsError> for InvokeError {
    fn from(val: VaultCommandsError) -> Self {
        let message = match val {
            VaultCommandsError::IO {
                source: _,
                location: _,
            } => "Unexpected io error occured!".to_string(),
            VaultCommandsError::Vault {
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

#[tauri::command]
pub async fn create_vault(
    state: State<'_, Config>,
    name: String,
    masterkey: String,
    confirm_masterkey: String,
) -> Result<Vault, VaultCommandsError> {
    let config = state.inner();
    let name = name.trim();
    validate_name(name)?;
    validate_masterkey(&masterkey, &confirm_masterkey)?;

    // Create empty encrypted vault content
    let vault = Vault::new(config, name.to_string())?;
    validate_path_not_exist(&vault.file_path)?;

    let passwords = Passwords::random_one();
    vault.encrypt(&masterkey, passwords)?;

    Ok(vault)
}

#[tauri::command]
pub async fn list_vaults(state: State<'_, Config>) -> Result<Vec<Vault>, VaultCommandsError> {
    let config = state.inner();
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

#[tauri::command]
pub async fn delete_vault(
    state: State<'_, Config>,
    name: String,
) -> Result<(), VaultCommandsError> {
    let config = state.inner();
    let vault = Vault::new(config, name)?;
    vault.delete_file()?;
    Ok(())
}

#[tauri::command]
pub async fn update_vault(
    state: State<'_, Config>,
    name: String,
    old_masterkey: String,
    new_masterkey: String,
    confirm_new_masterkey: String,
) -> Result<(), VaultCommandsError> {
    let config = state.inner();
    validate_new_masterkey(&old_masterkey, &new_masterkey, &confirm_new_masterkey)?;
    let vault = Vault::new(config, name)?;
    vault.update_masterkey(&old_masterkey, &new_masterkey)?;
    Ok(())
}

#[tauri::command]
pub async fn rename_vault(
    state: State<'_, Config>,
    name: String,
    new_name: String,
) -> Result<Vault, VaultCommandsError> {
    let config = state.inner();

    let new_name = new_name.trim();
    validate_name(new_name)?;

    let old_vault = Vault::new(config, name)?;
    let new_vault = Vault::new(config, new_name.to_string())?;

    validate_path_exist(&old_vault.file_path)?;
    validate_path_not_exist(&new_vault.file_path)?;

    // Rename the file
    fs::rename(&old_vault.file_path, &new_vault.file_path)?;
    fs::rename(&old_vault.metadata.file_path, &new_vault.metadata.file_path)?;

    Ok(new_vault)
}

fn validate_new_masterkey(
    old_masterkey: &str,
    new_masterkey: &str,
    confirm_masterkey: &str,
) -> Result<(), VaultCommandsError> {
    if old_masterkey == new_masterkey {
        return Err(VaultCommandsError::NewMasterKeySameValue);
    }

    validate_masterkey(new_masterkey, confirm_masterkey)
}

fn validate_masterkey(masterkey: &str, confirm_masterkey: &str) -> Result<(), VaultCommandsError> {
    const MIN_LENGTH: usize = 12;

    if masterkey != confirm_masterkey {
        return Err(VaultCommandsError::ConfirmMasterKeyMismatch);
    }

    if masterkey.trim().len() != masterkey.len() {
        return Err(VaultCommandsError::MasterKeySpaceAtEnds);
    }

    if masterkey.len() < MIN_LENGTH {
        return Err(VaultCommandsError::MasterKeyTooShort {
            min_length: MIN_LENGTH,
        });
    }

    Ok(())
}

fn validate_name(name: &str) -> Result<(), VaultCommandsError> {
    if name.trim().is_empty() {
        return Err(VaultCommandsError::VaultNameEmpty);
    }

    Ok(())
}

fn validate_path_not_exist(path: &Path) -> Result<(), VaultCommandsError> {
    if path.exists() {
        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_str().unwrap().to_string();
        return Err(VaultCommandsError::VaultFileExists { file_name });
    }

    Ok(())
}

fn validate_path_exist(path: &Path) -> Result<(), VaultCommandsError> {
    if !path.exists() {
        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_str().unwrap().to_string();
        return Err(VaultCommandsError::VaultFileNotFound { file_name });
    }

    Ok(())
}
