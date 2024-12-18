use tauri::State;

use crate::config::Config;
use crate::passwords::Password;
use crate::vault::Vault;
use crate::vault_controller::{VaultController, VaultControllerError};

#[tauri::command]
pub async fn create_vault(
    state: State<'_, Config>,
    name: String,
    masterkey: String,
    confirm_masterkey: String,
) -> Result<Vault, VaultControllerError> {
    let config = state.inner();
    let vault = VaultController::create_vault(config, &name, &masterkey, &confirm_masterkey)?;
    Ok(vault)
}

#[tauri::command]
pub async fn list_vaults(state: State<'_, Config>) -> Result<Vec<Vault>, VaultControllerError> {
    let config = state.inner();
    VaultController::list(config)
}

#[tauri::command]
pub async fn delete_vault(state: State<'_, Config>, name: String) -> Result<(), VaultControllerError> {
    let config = state.inner();
    VaultController::delete(config, &name)
}

#[tauri::command]
pub async fn update_vault(
    state: State<'_, Config>,
    name: String,
    old_masterkey: String,
    new_masterkey: String,
    confirm_new_masterkey: String,
) -> Result<(), VaultControllerError> {
    let config = state.inner();
    VaultController::update_masterkey(
        config,
        &name,
        &old_masterkey,
        &new_masterkey,
        &confirm_new_masterkey,
    )
}

#[tauri::command]
pub async fn rename_vault(
    state: State<'_, Config>,
    name: String,
    new_name: String,
) -> Result<Vault, VaultControllerError> {
    let config = state.inner();

    VaultController::rename(config, &name, &new_name)
}

#[tauri::command]
pub async fn get_passwords(
    state: State<'_, Config>,
    name: String,
    masterkey: String,
) -> Result<Vec<Password>, VaultControllerError> {
    Ok(Vec::new())
}

#[tauri::command]
pub async fn add_password(
    state: State<'_, Config>,
    name: String,
    masterkey: String,
    password: String,
    notes: String,
) -> Result<(), VaultControllerError> {
    Ok(())
    // Implementation to add new password
}

#[tauri::command]
pub async fn delete_password(
    state: State<'_, Config>,
    name: String,
    masterkey: String,
    index: usize,
) -> Result<(), VaultControllerError> {
    Ok(())
    // Implementation to delete password at index
}
