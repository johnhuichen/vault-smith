use tauri::ipc::InvokeError;
use tauri::State;

use crate::config::Config;
use crate::passwords::Password;
use crate::vault::{Vault, VaultError};

use snafu::{Location, Snafu};

#[derive(Debug, Snafu)]
pub enum PasswordCommandsError {
    #[snafu(display("Vault error, loc: {location}"))]
    #[snafu(context(false))]
    Vault {
        source: VaultError,
        #[snafu(implicit)]
        location: Location,
    },
}

impl From<PasswordCommandsError> for InvokeError {
    fn from(val: PasswordCommandsError) -> Self {
        let message = match val {
            PasswordCommandsError::Vault {
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
        };
        InvokeError::from(message)
    }
}

#[tauri::command]
pub async fn get_passwords(
    state: State<'_, Config>,
    vault_name: String,
    masterkey: String,
) -> Result<Vec<Password>, PasswordCommandsError> {
    let config = state.inner();
    let mut vault = Vault::new(config, vault_name)?;
    let passwords = vault.decrypt(&masterkey)?;
    vault.update_last_accessed()?;
    Ok(passwords.inner)
}

#[tauri::command]
pub async fn add_password(
    state: State<'_, Config>,
    vault_name: String,
    masterkey: String,
    notes: String,
) -> Result<Vec<Password>, PasswordCommandsError> {
    let config = state.inner();
    let vault = Vault::new(config, vault_name)?;
    let mut passwords = vault.decrypt(&masterkey)?;
    passwords.add_password(notes);
    vault.encrypt(&masterkey, &passwords)?;

    Ok(passwords.inner)
}

#[tauri::command]
pub async fn delete_password(
    state: State<'_, Config>,
    vault_name: String,
    masterkey: String,
    id: i32,
) -> Result<Vec<Password>, PasswordCommandsError> {
    let config = state.inner();
    let vault = Vault::new(config, vault_name)?;
    let mut passwords = vault.decrypt(&masterkey)?;
    passwords.delete_password(id);
    vault.encrypt(&masterkey, &passwords)?;

    Ok(passwords.inner)
}

#[tauri::command]
pub async fn update_password(
    state: State<'_, Config>,
    vault_name: String,
    masterkey: String,
    id: i32,
    password: String,
    notes: String,
) -> Result<Vec<Password>, PasswordCommandsError> {
    let config = state.inner();
    let vault = Vault::new(config, vault_name)?;
    let mut passwords = vault.decrypt(&masterkey)?;
    passwords.update_password(id, password, notes);
    vault.encrypt(&masterkey, &passwords)?;

    Ok(passwords.inner)
}
