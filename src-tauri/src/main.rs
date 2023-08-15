#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use config::VConfig;
use log::{error, info};
use std::{
    env,
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tauri::{async_runtime, App, AppHandle, Manager, RunEvent, SystemTrayEvent, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};
use tokio::sync::Mutex;
use utils::error::VError;

use crate::{
    commands::{
        config::{get_config, update_config},
        core::select_node,
        node_speed,
        subs::{add_subscription, update_all_subs, update_sub},
    },
    config::CoreStatus,
    core::VCore,
    logger::init_logger,
    message::{message_handler, msg_build},
    tray::{handle_tray_click, new_tray},
    utils::get_main_window,
};

mod commands;
mod config;
mod core;
mod logger;
mod message;
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

fn main() {
    #[cfg(debug_assertions)]
    dbg!("test");

    let tray = new_tray();

    // Init message
    // Create a mpsc channel for config and other stuff,
    // when other stuff change state and need to update config
    // it will use tx send new state to config
    let (tx, rx) = msg_build();
    let tx = Arc::new(tx);

    // Init config.
    let config = Arc::new(Mutex::new(VConfig::new()));

    match init_logger(tx.clone()) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Logger init failed {e}");
        }
    }

    info!("Starting up.");
    info!("Venus - {}", VERSION);

    let core = Arc::new(Mutex::new(VCore::build(tx.clone())));

    let core_app = core.clone();
    let config_app = config.clone();
    // App handler
    let handle_app = move |app: &mut App| -> Result<(), Box<dyn Error>> {
        let resources_path = app
            .handle()
            .path_resolver()
            .resolve_resource("resources/")
            .ok_or(VError::ResourceError("resource path is empty"))?;
        // Init config and core
        let init_config = config_app.clone();
        let window = get_main_window(app)?;
        let core = core_app.clone();
        // Start config and core
        async_runtime::spawn(async move {
            let mut config = init_config.lock().await;
            info!("Start init config");
            config.init(&resources_path)?;
            info!("Config init sucess");
            // Restore alll window status.
            if config.rua.save_windows {
                window.restore_state(StateFlags::all())?;
            }

            info!("Start core");
            let mut core = core.lock().await;
            // Set v2ray assert location with environment
            env::set_var("V2RAY_LOCATION_ASSET", &resources_path);

            match core.init(&config.core_path).await {
                Ok(_) => {
                    config.rua.core_status = CoreStatus::Started;
                    info!("Core started");
                }
                Err(err) => {
                    error!("Core start failed {err:?}");
                    CORE_SHUTDOWN.store(false, Ordering::Relaxed);
                    config.rua.core_status = CoreStatus::Stopped;
                }
            }
            Ok::<(), VError>(())
        });

        let window = get_main_window(app)?;
        let event_config = config_app.clone();
        app.listen_global("ready", move |_e| {
            info!("Frontend ready");
            let window = window.get_window("main").unwrap();
            let event_config = event_config.clone();
            let task = async move {
                let config = event_config.lock().await;
                window.emit_all("rua://update-rua-config", &config.rua)?;
                window.emit_all("rua://update-core-config", &config.core)?;
                info!("Reload config succeeded");
                Ok::<(), VError>(())
            };
            async_runtime::spawn(task);
        });

        let msg_config = config_app.clone();
        // Receive message for core
        let msg_core = core_app.clone();
        let window = get_main_window(app)?;
        // The config will use receiver here
        // when got a message, config will update and
        // emit a event to notify frontend to update global state
        message_handler(window, rx, msg_config, msg_core)?;
        Ok(())
    };

    // Used to runner to manage core process
    let core_runner = core.clone();
    let config_runner = config.clone();
    // Runner handler
    let runner = move |app: &AppHandle, event: RunEvent| match event {
        RunEvent::Exit => {
            let core_runner = core_runner.clone();
            async_runtime::spawn(async move {
                let mut core = core_runner.lock().await;
                CORE_SHUTDOWN.store(true, Ordering::Relaxed);
                core.exit().expect("Kill core failed");
            });
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
            let tray_handle = app.tray_handle().get_item("hide");
            tray_handle
                .set_title("Show")
                .expect("Can not set tray title");

            let config = config_runner.clone();
            let app_handler = app.app_handle();
            async_runtime::spawn(async move {
                let config = config.lock().await;
                if config.rua.save_windows {
                    app_handler
                        .save_window_state(StateFlags::all())
                        .map_err(|_e| VError::WindowError("Save window status failed"))?;
                }
                Ok::<(), VError>(())
            });
        }
        _ => {}
    };

    let core_tray = core.clone();
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {}
            SystemTrayEvent::DoubleClick { .. } => {
                let windows = app.windows();
                let task = async move {
                    for (_, window) in windows {
                        window.show()?;
                        window.set_focus()?;
                    }
                    Ok::<(), VError>(())
                };
                async_runtime::spawn(task);
            }
            SystemTrayEvent::MenuItemClick { id, .. } => handle_tray_click(app, id, &core_tray),
            _ => {}
        })
        .manage(config)
        .manage(core)
        .manage(tx)
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
            // common commands
            node_speed
        ])
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
        .setup(handle_app)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(runner);
}
