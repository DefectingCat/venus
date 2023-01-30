#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::utils::manager::{download_latest, HttpClient};
use anyhow::Result;

mod utils;
mod version;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn test(filename: String) {
    dbg!(filename);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test])
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                let http_client = HttpClient::new().unwrap().client;
                download_latest(&http_client).await.expect("Download file");
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
