#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::common::current_dir;
use config::VConfig;
use env_logger::Env;
use log::{error, info};
use std::sync::{Arc, Mutex};
use tauri::Manager;

use crate::{
    commands::common::{add_subscription, get_config},
    core::VCore,
};

mod commands;
mod config;
mod consts;
mod core;
mod utils;

fn main() {
    // Init config.
    let config = Arc::new(Mutex::new(VConfig::new()));

    let env = Env::default().filter_or("RUA_LOG_LEVEL", "info");
    env_logger::init_from_env(env);
    info!("starting up.");
    info!("V2rayR - {}", env!("CARGO_PKG_VERSION"));

    info!("Start core");
    let core = match VCore::build(config.clone()) {
        Ok(core) => Some(core),
        Err(err) => {
            error!("Core start failed {err:?}");
            None
        }
    };

    let config_state = config.clone();
    tauri::Builder::default()
        .manage(config_state)
        .invoke_handler(tauri::generate_handler![
            current_dir,
            add_subscription,
            get_config
        ])
        .setup(move |app| {
            config
                .lock()
                .expect("can not lock config")
                .init(&app.handle())
                .expect("can not init core config");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
