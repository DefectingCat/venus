#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::common::current_dir;
use config::VConfig;
use env_logger::Env;
use log::info;
use std::sync::{Arc, Mutex};

use crate::core::VCore;

mod commands;
mod config;
mod consts;
mod core;
mod utils;

fn main() {
    // Init config.
    let config = Arc::new(Mutex::new(VConfig::new()));
    let level = &config.lock().expect("can not lock config.").log_level;

    let env = Env::default().filter_or("RUA_LOG_LEVEL", level);
    env_logger::init_from_env(env);
    info!("starting up.");
    info!("V2rayR - {}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![current_dir])
        .setup(|_app| {
            info!("Start core");
            let _core = VCore::build().unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
