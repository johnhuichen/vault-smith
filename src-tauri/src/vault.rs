use borsh::{BorshDeserialize, BorshSerialize};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use snafu::{Location, Snafu};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use tauri::ipc::InvokeError;

use crate::cipher::{Cipher, CipherError};
use crate::config::Config;
use crate::passwords::Passwords;

#[derive(Debug, Snafu)]
pub enum VaultMetadataError {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultMetadata {
    pub file_path: PathBuf,
    pub created_at: DateTime<Local>,
    pub last_accessed: DateTime<Local>,
}

impl VaultMetadata {
    pub fn new(config: &Config, vault_name: &str) -> Result<Self, VaultMetadataError> {
        let file_path = Self::file_path(config, vault_name);

        if file_path.exists() {
            let file_path = Self::file_path(config, vault_name);
            let file = File::open(&file_path)?;
            let mut metadata: VaultMetadata = serde_json::from_reader(file)?;
            metadata.file_path = file_path;

            Ok(metadata)
        } else {
            let now = Local::now();
            Ok(Self {
                file_path,
                created_at: now,
                last_accessed: now,
            })
        }
    }

    pub fn save_to_file(&self) -> Result<(), VaultMetadataError> {
        let metadata_str = serde_json::to_string(self)?;
        fs::write(&self.file_path, metadata_str)?;
        Ok(())
    }

    pub fn delete_file(&self) -> Result<(), VaultMetadataError> {
        if self.file_path.exists() {
            fs::remove_file(&self.file_path)?;
        }
        Ok(())
    }

    fn file_path(config: &Config, vault_name: &str) -> PathBuf {
        config.app_data_dir.join(format!("{}.meta", vault_name))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    pub file_path: PathBuf,
    pub name: String,
    pub metadata: VaultMetadata,
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

    #[snafu(display("Cipher Error"))]
    #[snafu(context(false))]
    Cipher {
        source: CipherError,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Vault metadata error"))]
    #[snafu(context(false))]
    VaultMeta {
        source: VaultMetadataError,
        #[snafu(implicit)]
        location: Location,
    },
}

impl From<VaultError> for InvokeError {
    fn from(val: VaultError) -> Self {
        InvokeError::from(val.to_string())
    }
}

impl Vault {
    pub fn new(config: &Config, name: String) -> Result<Self, VaultError> {
        let file_path = Self::file_path(config, &name);
        let metadata = VaultMetadata::new(config, &name)?;
        Ok(Self {
            file_path,
            metadata,
            name,
        })
    }

    pub fn update_masterkey(
        &self,
        old_masterkey: &str,
        new_masterkey: &str,
    ) -> Result<(), VaultError> {
        let old_cipher = Cipher::new(old_masterkey);
        let file = File::open(&self.file_path)?;
        let mut reader = BufReader::new(file);
        let encoded = old_cipher.parse(&mut reader)?;
        let passwords = Passwords::try_from_slice(&encoded)?;

        self.save_to_file(new_masterkey, passwords)?;

        Ok(())
    }

    pub fn save_to_file(&self, masterkey: &str, passwords: Passwords) -> Result<(), VaultError> {
        let cipher = Cipher::new(masterkey);
        let file = File::create(&self.file_path)?;
        let mut writer = BufWriter::new(file);
        cipher.dump(passwords.try_to_vec()?, &mut writer)?;

        self.metadata.save_to_file()?;

        Ok(())
    }

    pub fn delete_file(&self) -> Result<(), VaultError> {
        if self.file_path.exists() {
            fs::remove_file(&self.file_path)?;
        }
        self.metadata.delete_file()?;
        Ok(())
    }

    fn file_path(config: &Config, name: &str) -> PathBuf {
        config.app_data_dir.join(format!("{}.pwd", name))
    }
}
