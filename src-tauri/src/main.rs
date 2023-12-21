#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::{
    commands::{
        config::{get_config, update_config},
        core::{restart_core, select_node},
        node_speed,
        subs::{add_subscription, update_all_subs, update_sub},
        ui::{exit_app, toggle_window},
    },
    core::{exit_core, VCore},
    init::setup_app,
    logger::init_logger,
    message::{ConfigMsg, MSG_TX},
    tray::tray_menu,
};
use anyhow::Ok as AOk;
use config::VConfig;
use log::{error, info};
use once_cell::sync::Lazy;
use std::{env, sync::atomic::AtomicBool};
use store::ui::UI;
use tauri::{
    async_runtime, AppHandle, Manager, RunEvent, SystemTray, SystemTrayEvent, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
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

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

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

    // Used to runner to manage core process
    // Runner handler
    let runner = move |app: &AppHandle, event: RunEvent| match event {
        RunEvent::Exit => {
            async_runtime::spawn(async move { exit_core().await });
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

            let app_handler = app.app_handle();
            async_runtime::spawn(async move {
                if label == "main" {
                    {
                        let mut ui = UI.lock().await;
                        ui.main_visible = false;
                    }
                    MSG_TX.lock().await.send(ConfigMsg::EmitUI).await?;
                }
                let config = CONFIG.lock().await;
                if config.rua.save_windows {
                    if let Err(err) = app_handler.save_window_state(StateFlags::all()) {
                        error!("Save window status failed {}", err)
                    };
                }
                AOk(())
            });
        }
        _ => {}
    };

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
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            info!("{}, {argv:?}, {cwd}", app.package_info().name);
            match app.emit_all("single-instance", Payload { args: argv, cwd }) {
                Ok(_) => {
                    let windows = app.windows();
                    windows.iter().for_each(|(_, win)| {
                        win.show().expect("Cannot show window");
                        win.set_focus().expect("Cannot set focus on window");
                    })
                }
                Err(err) => {
                    error!("{err}");
                }
            };
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .on_window_event(|event| {
            if let tauri::WindowEvent::Focused(is_focused) = event.event() {
                let name = event.window().label();
                if !is_focused && name == "menu" {
                    event.window().hide().unwrap();
                }
            }
        })
        .setup(setup_app)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(runner);
}
