#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::common::current_dir;
use config::VConfig;
use env_logger::Env;
use log::{error, info};
use std::sync::{Arc, Mutex};
use tauri::{Manager, RunEvent};

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
    let mut core = match VCore::build(config.clone()) {
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
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |_app_handle, event| match event {
            RunEvent::Exit => {}
            RunEvent::ExitRequested { api, .. } => {
                if let Some(mut core) = core.take() {
                    core.exit().expect("")
                }
            }
            _ => {}
        });
}
