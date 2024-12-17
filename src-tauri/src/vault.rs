use borsh::BorshSerialize;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use snafu::{Location, ResultExt, Snafu};
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::PathBuf;

use crate::cipher::{Cipher, CipherError};
use crate::passwords::Passwords;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    pub id: String,
    pub last_accessed: String,
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

    #[snafu(display("Vault with name '{}' already exists", name))]
    VaultExists { name: String },

    #[snafu(display("Vault '{}' does not exist", name))]
    VaultNotFound { name: String },

    #[snafu(display("Invalid vault name: {}", reason))]
    InvalidVaultName { reason: String },

    #[snafu(display("Cannot rename to '{}': vault already exists", name))]
    RenameTargetExists { name: String },

    #[snafu(display("Master key must be at least {} characters long", min_length))]
    MasterKeyTooShort { min_length: usize },

    #[snafu(display("Cipher Error"))]
    Cipher { source: CipherError },

    #[snafu(display("Failed to get app directory"))]
    AppDirAccess,
}

impl Vault {
    // fn get_vaults_dir() -> Result<PathBuf, VaultError> {
    //     let maybe_app_dir = app_data_dir(&tauri::Config::default());
    //
    //     match maybe_app_dir {
    //         Some(app_dir) => {
    //             let vaults_dir = app_dir.join("pawn-vaults");
    //             std::fs::create_dir_all(&vaults_dir)?;
    //
    //             Ok(vaults_dir)
    //         }
    //         None => Err(VaultError::AppDirAccess),
    //     }
    // }
    //
    // pub fn get_metadata_path(vault_path: &PathBuf) -> PathBuf {
    //     vault_path.with_extension("meta")
    // }
    //
    // pub fn validate_master_key(master_key: &str) -> Result<(), VaultError> {
    //     const MIN_LENGTH: usize = 12;
    //
    //     if master_key.len() < MIN_LENGTH {
    //         return Err(VaultError::MasterKeyTooShort {
    //             min_length: MIN_LENGTH,
    //         });
    //     }
    //
    //     Ok(())
    // }
    //
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
    // pub fn create_encrypted_vault(
    //     name: &str,
    //     master_key: &str,
    //     vault_path: &PathBuf,
    // ) -> Result<(), VaultError> {
    //     let now = Local::now();
    //
    //     // Create metadata
    //     let metadata = VaultMetadata {
    //         created_at: now,
    //         last_accessed: now,
    //     };
    //
    //     // Save metadata to separate file
    //     let metadata_path = Self::get_metadata_path(vault_path);
    //     let metadata = serde_json::to_string(&metadata).context(SerializeSnafu)?;
    //     fs::write(&metadata_path, metadata)?;
    //
    //     // Create empty encrypted vault content
    //     let cipher = Cipher::new(master_key.to_string());
    //     let file = File::create(vault_path)?;
    //     let mut writer = BufWriter::new(file);
    //     cipher
    //         .dump(Passwords::empty().try_to_vec()?, &mut writer)
    //         .context(CipherSnafu)?;
    //
    //     Ok(())
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
