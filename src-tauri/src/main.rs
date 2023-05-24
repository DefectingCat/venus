#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::common::current_dir;
use config::VConfig;
use env_logger::Env;
use log::info;
use std::sync::{Arc, Mutex};

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
    let core = VCore::build().unwrap();

    let env = Env::default().filter_or("RUA_LOG_LEVEL", "info");
    env_logger::init_from_env(env);
    info!("starting up.");
    info!("V2rayR - {}", env!("CARGO_PKG_VERSION"));

    let config_state = config.clone();
    tauri::Builder::default()
        .manage(config)
        .invoke_handler(tauri::generate_handler![
            current_dir,
            add_subscription,
            get_config
        ])
        .setup(move |app| {
            info!("Start core");
            config_state
                .lock()
                .expect("can not lock config")
                .reload_core(app.handle())
                .expect("can not init core config");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
