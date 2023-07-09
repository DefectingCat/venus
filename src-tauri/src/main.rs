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
        Arc, Mutex,
    },
};
use tauri::{
    async_runtime, App, AppHandle, Manager, RunEvent, SystemTrayEvent, Window, WindowEvent,
};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};
use tokio::sync::{self, mpsc::Receiver};
use utils::error::{VError, VResult};

use crate::{
    commands::{
        config::{get_core_config, get_rua_config, update_config},
        core::select_node,
        subs::{add_subscription, update_all_subs, update_sub},
    },
    config::CoreStatus,
    core::VCore,
    logger::init_logger,
    message::{msg_build, ConfigMsg},
    tray::{handle_tray_click, new_tray},
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
static VERSION: &str = env!("CARGO_PKG_VERSION");
static NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let tray = new_tray();

    // Init message
    // Create a mpsc channel for config and other stuff,
    // when other stuff change state and need to update config
    // it will use tx send new state to config
    let (tx, rx) = msg_build();
    let tx = Arc::new(tx);
    // Init config.
    let config = Arc::new(async_runtime::Mutex::new(VConfig::new()));

    match init_logger(tx.clone(), config.clone()) {
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
        let window = app
            .get_window("main")
            .ok_or(VError::EmptyError("Can not get main window"))?;
        let core = core_app.clone();
        // Start config and core
        async_runtime::spawn(async move {
            let mut config = init_config.lock().await;
            config.init(&resources_path)?;
            info!("Config init sucess");
            // Restore alll window status.
            if config.rua.save_windows {
                window.restore_state(StateFlags::all())?;
            }

            info!("Start core");
            let mut core = core.lock().expect("Can not lock core");
            // Set v2ray assert location with environment
            env::set_var("V2RAY_LOCATION_ASSET", &resources_path);
            match core.init(&config.core_path) {
                Ok(_) => {
                    config.rua.core_status = CoreStatus::Started;
                    info!("Core started")
                }
                Err(err) => {
                    error!("Core start failed {err:?}");
                    CORE_SHUTDOWN.store(false, Ordering::Relaxed);
                    config.rua.core_status = CoreStatus::Stopped;
                }
            }
            Ok::<(), VError>(())
        });

        let window = app
            .get_window("main")
            .ok_or(VError::EmptyError("Can not get main window"))?;
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
        let main_window = app
            .get_window("main")
            .ok_or(VError::EmptyError("Can not get main window"))?;
        // The config will use receiver here
        // when got a message, config will update and
        // emit a event to notify frontend to update global state
        message_handler(main_window, rx, msg_config, msg_core)?;
        Ok(())
    };

    // Used to runner to manage core process
    let core_runner = core.clone();
    let config_runner = config.clone();
    // Runner handler
    let runner = move |app: &AppHandle, event: RunEvent| match event {
        RunEvent::Exit => {
            let mut core = core_runner.lock().expect("");
            CORE_SHUTDOWN.store(true, Ordering::Relaxed);
            core.exit().expect("Kill core failed");
        }
        RunEvent::ExitRequested { api, .. } => {
            let _api = api;
        }
        RunEvent::WindowEvent { label, event, .. } => {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    let win = app.get_window(label.as_str()).expect("Cannot get window");
                    win.hide().expect("Cannot hide window");
                    api.prevent_close();
                    let tray_handle = app.tray_handle().get_item("hide");
                    tray_handle
                        .set_title("Show")
                        .expect("Can not set tray title");
                }
                _ => {
                    let config = config_runner.clone();
                    let app_handler = app.app_handle();
                    async_runtime::spawn(async move {
                        let config = config.lock().await;
                        if config.rua.save_windows {
                            app_handler
                                .save_window_state(StateFlags::all())
                                .map_err(|_e| VError::EmptyError("Save window status failed"))?;
                        }
                        Ok::<(), VError>(())
                    });
                }
            };
        }
        _ => {}
    };

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
            SystemTrayEvent::MenuItemClick { id, .. } => handle_tray_click(app, id, &core),
            _ => {}
        })
        .manage(config)
        .manage(tx)
        .invoke_handler(tauri::generate_handler![
            // subs
            add_subscription,
            update_all_subs,
            update_sub,
            // configs
            get_rua_config,
            get_core_config,
            update_config,
            // core
            select_node,
            // common commands
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            info!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .setup(handle_app)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(runner);
}

fn message_handler(
    window: Window,
    mut rx: Receiver<ConfigMsg>,
    msg_config: Arc<sync::Mutex<VConfig>>,
    msg_core: Arc<Mutex<VCore>>,
) -> VResult<()> {
    let handler = async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                ConfigMsg::CoreStatue(status) => {
                    info!("Update core status {}", status.as_str());
                    let mut config = msg_config.lock().await;
                    config.rua.core_status = status;
                    window.emit_all("rua://update-rua-config", &config.rua)?;
                }
                ConfigMsg::RestartCore => {
                    info!("Restarting core");
                    let mut config = msg_config.lock().await;
                    let mut core = msg_core.lock()?;
                    match core.restart() {
                        Ok(_) => {
                            config.rua.core_status = CoreStatus::Started;
                            window.emit_all("rua://update-rua-config", &config.rua)?;
                            window.emit_all("rua://update-core-config", &config.core)?;
                        }
                        Err(err) => {
                            error!("Core restart failed {err}");
                        }
                    }
                }
                ConfigMsg::EmitLog(log) => {
                    window.emit("rua://emit-log", log)?;
                }
            }
        }
        Ok::<(), VError>(())
    };
    async_runtime::spawn(handler);
    Ok(())
}
