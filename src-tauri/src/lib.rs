use self::config::Config;
use self::password_commands::{add_password, delete_password, get_passwords};
use self::vault_commands::{create_vault, delete_vault, list_vaults, rename_vault, update_vault};
use snafu::{ResultExt, Whatever};
use tauri::Manager;

mod cipher;
mod config;
mod password_commands;
mod passwords;
mod vault;
mod vault_commands;

#[snafu::report]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Whatever> {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                let log_plugin = tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build();
                app.handle().plugin(log_plugin)?;
            }

            app.manage(Config {
                app_data_dir: app.path().app_local_data_dir()?,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_vaults,
            create_vault,
            delete_vault,
            update_vault,
            rename_vault,
            get_passwords,
            add_password,
            delete_password
        ])
        .run(tauri::generate_context!())
        .whatever_context("error while running tauri application")?;

    Ok(())
}
