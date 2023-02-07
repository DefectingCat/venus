#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use commands::common::current_dir;
use config::VConfig;
use utils::manager::{download_latest, HttpClient};

mod commands;
mod config;
mod consts;
mod utils;

fn main() {
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

fn init() {
    let config = VConfig::new();
}
