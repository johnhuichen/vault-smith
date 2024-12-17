use snafu::Whatever;
use tauri::State;

use crate::config::Config;
use crate::vault::{Vault, VaultError};
use std::fs;
use std::sync::Mutex;

#[tauri::command]
pub fn create_vault(state: tauri::State<Mutex<Config>>, name: String) -> Result<(), String> {
    let state = state.lock().unwrap();
    println!("{:?}, {:?}", state, name);
    Ok(())
    // Vault::validate_name(&name).map_err(|e| e.to_string())?;
    //
    // let vaults_dir = get_vaults_dir().map_err(|e| e.to_string())?;
    // let vault_path = vaults_dir.join(format!("{}.pwd", name));
    //
    // if vault_path.exists() {
    //     return Err(VaultError::VaultExists { name }.to_string());
    // }
    //
    // fs::write(&vault_path, "")
    //     .map_err(|source| VaultError::CreateFile { source })
    //     .map_err(|e| e.to_string())?;
    //
    // Ok(Vault {
    //     id: name.clone(),
    //     name,
    //     last_accessed: chrono::Local::now().format("%Y-%m-%d").to_string(),
    // })
}

#[tauri::command]
pub async fn list_vaults() -> Result<(), String> {
    Ok(())
    // pub async fn list_vaults() -> Result<Vec<Vault>, String> {
    // let vaults_dir = get_vaults_dir().map_err(|e| e.to_string())?;
    //
    // let mut vaults = Vec::new();
    // let entries = fs::read_dir(vaults_dir)
    //     .map_err(|source| VaultError::ReadDirectory { source })
    //     .map_err(|e| e.to_string())?;
    //
    // for entry in entries {
    //     let entry = entry
    //         .map_err(|source| VaultError::ReadDirectory { source })
    //         .map_err(|e| e.to_string())?;
    //     let file_name = entry.file_name();
    //     let file_name_str = file_name.to_string_lossy();
    //
    //     if let Some(vault) = Vault::from_file_name(&file_name_str) {
    //         vaults.push(vault);
    //     }
    // }
    //
    // // Sort by last accessed time, most recent first
    // vaults.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
    // Ok(vaults)
}

#[tauri::command]
pub async fn delete_vault(name: String) -> Result<(), String> {
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
pub async fn rename_vault(old_name: String, new_name: String) -> Result<(), String> {
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
