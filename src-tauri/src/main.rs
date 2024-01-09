#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::{
    commands::{
        config::{get_config, read_config_file, update_config},
        core::{restart_core, select_node},
        node_speed,
        subs::{add_subscription, update_all_subs, update_sub},
        ui::{exit_app, toggle_window},
    },
    core::VCore,
    init::{app_runtime, setup_app, single_instance_init, window_event_handler},
    logger::init_logger,
    tray::tray_menu,
};
use config::VConfig;
use log::info;
use once_cell::sync::Lazy;
use std::{env, sync::atomic::AtomicBool};
use store::ui::UI;
use tauri::{SystemTray, SystemTrayEvent};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::Mutex;

mod commands;
mod config;
mod core;
mod event;
mod init;
mod logger;
mod message;
mod store;
mod tray;
mod utils;

/// Determine the core is manual killed or it's got killed by not expected.
/// if manual killed will be true, otherwise false.
static CORE_SHUTDOWN: AtomicBool = AtomicBool::new(false);
/// Is logging to frontend
static LOGGING: AtomicBool = AtomicBool::new(false);
/// info from package
static VERSION: &str = env!("CARGO_PKG_VERSION");
static NAME: &str = env!("CARGO_PKG_NAME");

/// Control v2ray-core
pub static CORE: Lazy<Mutex<VCore>> = Lazy::new(|| Mutex::new(VCore::build()));
/// Global config and v2ray-core config
pub static CONFIG: Lazy<Mutex<VConfig>> = Lazy::new(|| Mutex::new(VConfig::new()));
/// Global UI state
pub static UI: Lazy<Mutex<UI>> = Lazy::new(|| Mutex::new(UI::default()));

fn main() {
    #[cfg(debug_assertions)]
    use utils::debug_process;
    #[cfg(debug_assertions)]
    debug_process();

    match init_logger() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Logger init failed {e}");
        }
    }
    info!("Starting up.");
    info!("Venus - {}", VERSION);

    tauri::Builder::default()
        .system_tray(SystemTray::new())
        .on_system_tray_event(move |app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick { .. } => tray_menu(app),
                SystemTrayEvent::RightClick { .. } => tray_menu(app),
                // SystemTrayEvent::DoubleClick { .. } => {}
                // SystemTrayEvent::MenuItemClick { id, .. } => handle_tray_click(app, id),
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            // subs
            add_subscription,
            update_all_subs,
            update_sub,
            // configs
            get_config,
            update_config,
            read_config_file,
            // core
            select_node,
            restart_core,
            // common commands
            node_speed,
            // ui
            toggle_window,
            exit_app
        ])
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(single_instance_init))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .on_window_event(window_event_handler)
        .setup(setup_app)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(app_runtime);
}
