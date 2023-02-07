#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::common::current_dir;
use config::VConfig;
use env_logger::Env;
use log::info;
use std::sync::{Arc, Mutex};
use utils::manager::{download_latest, HttpClient};

mod commands;
mod config;
mod consts;
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
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                let http_client = HttpClient::new()
                    .expect("error while create http client.")
                    .client;
                download_latest(&http_client)
                    .await
                    .expect("error while download file");
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
