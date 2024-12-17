use tauri::State;

use crate::config::Config;
use crate::vault::{Vault, VaultError};

#[tauri::command]
pub fn create_vault(
    state: State<Config>,
    name: String,
    masterkey: String,
) -> Result<Vault, VaultError> {
    let config = state.inner();
    let vault = Vault::new(name, masterkey, config)?;
    Ok(vault)
}

#[tauri::command]
pub fn list_vaults(state: State<Config>) -> Result<Vec<Vault>, VaultError> {
    let config = state.inner();
    Vault::list(config)
}

#[tauri::command]
pub fn delete_vault(name: String) -> Result<(), String> {
    Ok(())
    // let vaults_dir = get_vaults_dir().map_err(|e| e.to_string())?;
    // let vault_path = vaults_dir.join(format!("{}.pwd", name));
    // let metadata_path = Vault::get_metadata_path(&vault_path);
    //
    // // Delete both vault and metadata files
    // if vault_path.exists() {
    //     fs::remove_file(&vault_path)
    //         .map_err(|source| VaultError::DeleteFile { source })
    //         .map_err(|e| e.to_string())?;
    // }
    //
    // if metadata_path.exists() {
    //     fs::remove_file(&metadata_path)
    //         .map_err(|source| VaultError::DeleteFile { source })
    //         .map_err(|e| e.to_string())?;
    // }
    //
    // Ok(())
}

#[tauri::command]
pub fn rename_vault(old_name: String, new_name: String) -> Result<(), String> {
    Ok(())
    // pub async fn rename_vault(old_name: String, new_name: String) -> Result<Vault, String> {
    // // Validate new name
    // Vault::validate_name(&new_name).map_err(|e| e.to_string())?;
    //
    // let vaults_dir = get_vaults_dir().map_err(|e| e.to_string())?;
    // let old_path = vaults_dir.join(format!("{}.pwd", old_name));
    // let new_path = vaults_dir.join(format!("{}.pwd", new_name));
    //
    // // Check if source vault exists
    // if !old_path.exists() {
    //     return Err(VaultError::VaultNotFound { name: old_name }.to_string());
    // }
    //
    // // Check if target name already exists
    // if new_path.exists() {
    //     return Err(VaultError::RenameTargetExists { name: new_name }.to_string());
    // }
    //
    // // Rename the file
    // fs::rename(&old_path, &new_path)
    //     .map_err(|source| VaultError::RenameFile { source })
    //     .map_err(|e| e.to_string())?;
    //
    // Ok(Vault {
    //     id: new_name.clone(),
    //     name: new_name,
    //     last_accessed: chrono::Local::now().format("%Y-%m-%d").to_string(),
    // })
}
