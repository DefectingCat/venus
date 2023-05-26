#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::common::current_dir;
use config::VConfig;
use env_logger::Env;
use log::{error, info};
use std::sync::{Arc, Mutex};
use tauri::{Manager, RunEvent, WindowEvent};

use crate::{
    commands::common::{add_subscription, get_config, get_rua_nodes, get_subscriptions},
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
            get_rua_nodes,
            get_subscriptions,
            get_config
        ])
        .setup(move |app| {
            let mut config = config.lock().expect("can not lock config");
            config
                .init(&app.handle())
                .expect("can not init core config");

            app.listen_global("ready", |_e| {
                info!("Got front ready event");
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app, event| match event {
            RunEvent::Exit => {
                if let Some(mut core) = core.take() {
                    core.exit().expect("")
                }
            }
            RunEvent::ExitRequested { api, .. } => {
                let _api = api;
            }
            RunEvent::WindowEvent {
                label,
                event: WindowEvent::CloseRequested { api, .. },
                ..
            } => {
                let win = app.get_window(label.as_str()).expect("Cannot get window");
                win.hide().expect("Cannot hide window");
                api.prevent_close();
            }
            _ => {}
        });
}
