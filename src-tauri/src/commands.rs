use tauri::State;

use crate::config::Config;
use crate::passwords::{Password, Passwords};
use crate::vault::{Vault, VaultError};

#[tauri::command]
pub fn create_vault(
    state: State<Config>,
    name: String,
    masterkey: String,
) -> Result<Vault, VaultError> {
    let config = state.inner();
    let vault = Vault::new(config, &name, &masterkey)?;
    Ok(vault)
}

#[tauri::command]
pub fn list_vaults(state: State<Config>) -> Result<Vec<Vault>, VaultError> {
    let config = state.inner();
    Vault::list(config)
}

#[tauri::command]
pub fn delete_vault(state: State<Config>, name: String) -> Result<(), VaultError> {
    let config = state.inner();
    Vault::delete(config, &name)
}

#[tauri::command]
pub fn update_vault(
    state: State<Config>,
    name: String,
    old_masterkey: String,
    new_masterkey: String,
) -> Result<(), VaultError> {
    let config = state.inner();
    Vault::update(config, &name, &old_masterkey, &new_masterkey)
}

#[tauri::command]
pub fn rename_vault(
    state: State<Config>,
    name: String,
    new_name: String,
) -> Result<Vault, VaultError> {
    let config = state.inner();
    Vault::rename(config, &name, &new_name)
}

#[tauri::command]
pub fn add_password(
    state: State<Config>,
    name: String,
    masterkey: String,
    password: String,
    notes: String,
) -> Result<(), VaultError> {
    Ok(())
    // Implementation to add new password
}

#[tauri::command]
pub fn delete_password(
    state: State<Config>,
    name: String,
    masterkey: String,
    index: usize,
) -> Result<(), VaultError> {
    Ok(())
    // Implementation to delete password at index
}

#[tauri::command]
pub fn get_passwords(
    state: State<Config>,
    name: String,
    masterkey: String,
) -> Result<Vec<Password>, VaultError> {
    Ok(Vec::new())
}
