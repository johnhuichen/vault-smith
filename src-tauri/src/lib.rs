use self::commands::{create_vault, delete_vault, list_vaults, rename_vault};
use self::config::Config;
use snafu::{ResultExt, Whatever};
use tauri::{Builder, Manager};

mod cipher;
mod commands;
mod config;
mod passwords;
mod vault;

#[snafu::report]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Whatever> {
    Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                let log_plugin = tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build();
                app.handle().plugin(log_plugin)?;
            }

            app.manage(Config {
                app_data_dir: app.path().app_data_dir()?,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_vaults,
            create_vault,
            delete_vault,
            rename_vault
        ])
        .run(tauri::generate_context!())
        .whatever_context("error while running tauri application")?;

    Ok(())
}
