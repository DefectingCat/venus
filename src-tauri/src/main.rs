#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use config::VConfig;
use env_logger::Env;
use log::{error, info};
use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
use tauri::{
    async_runtime, App, AppHandle, Manager, RunEvent, SystemTrayEvent, Window, WindowEvent,
};
use tokio::sync::{self, mpsc::Receiver};
use utils::error::{VError, VResult};

use crate::{
    commands::{
        config::{get_core_config, get_rua_config},
        core::select_node,
        subs::{add_subscription, get_subscriptions, update_all_subs},
    },
    config::CoreStatus,
    core::VCore,
    message::{msg_build, ConfigMsg},
    tray::{handle_tray_click, new_tray},
};

mod commands;
mod config;
mod core;
mod message;
mod tray;
mod utils;

/// Determine the core is manual killed or it's got killed by not expected.
/// if manual killed will be true, otherwise false.
static CORE_SHUTDOWN: AtomicBool = AtomicBool::new(false);

fn main() {
    let tray = new_tray();

    let env = Env::default().filter_or("RUA_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    // Init message
    // Create a mpsc channel for config and other stuff,
    // when other stuff change state and need to update config
    // it will use tx send new state to config
    let (tx, rx) = msg_build();
    let tx = Arc::new(tx);
    // Init config.
    let config = Arc::new(async_runtime::Mutex::new(VConfig::new()));

    info!("starting up.");
    info!("V2rayR - {}", env!("CARGO_PKG_VERSION"));

    let core = Arc::new(Mutex::new(VCore::build(tx.clone())));

    let core_app = core.clone();
    let config_app = config.clone();
    // App handler
    let handle_app = move |app: &mut App| -> Result<(), Box<dyn Error>> {
        let resources_path = app.handle().path_resolver().resolve_resource("resources/");

        // Init config and core
        let init_config = config_app.clone();
        async_runtime::spawn(async move {
            let mut config = init_config.lock().await;
            match config.init(resources_path) {
                Ok(_) => {
                    info!("Config init sucess");
                }
                Err(err) => {
                    error!("Config init failed {err}");
                }
            }
        });
        let mut core = core_app.lock().expect("Can not lock core");
        info!("Start core");
        let resources_path = app.handle().path_resolver().resolve_resource("resources/");
        // Used to start core
        let config_core = config_app.clone();
        match core.init(resources_path) {
            Ok(_) => {
                async_runtime::spawn(async move {
                    let mut config = config_core.lock().await;
                    config.rua.core_status = CoreStatus::Started;
                    info!("Core started")
                });
            }
            Err(err) => {
                error!("Core start failed {err:?}");
                CORE_SHUTDOWN.store(false, Ordering::Relaxed);
                async_runtime::spawn(async move {
                    let mut config = config_core.lock().await;
                    config.rua.core_status = CoreStatus::Stopped;
                });
            }
        }

        app.listen_global("ready", move |_e| {
            info!("Got front ready event");
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
        match message_handler(main_window, rx, msg_config, msg_core) {
            Ok(_) => {}
            Err(err) => {
                error!("Handle message failed: {err}")
            }
        }
        Ok(())
    };

    // Used to runner to manage core process
    let core_runner = core.clone();
    // Runner handler
    let runner = move |app: &AppHandle, event: RunEvent| match event {
        RunEvent::Exit => {
            let mut core = core_runner.lock().expect("");
            CORE_SHUTDOWN.store(true, Ordering::Relaxed);
            core.exit().expect("Kill core failed")
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
        }
        _ => {}
    };

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {}
            SystemTrayEvent::DoubleClick { .. } => {
                let windows = app.windows();
                for (_, window) in windows {
                    window.show().unwrap()
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => handle_tray_click(app, id, &core),
            _ => {}
        })
        .manage(config)
        .manage(tx)
        .invoke_handler(tauri::generate_handler![
            add_subscription,
            get_subscriptions,
            update_all_subs,
            get_rua_config,
            get_core_config,
            select_node
        ])
        .setup(handle_app)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(runner);
}

fn message_handler(
    main_window: Window,
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
                    main_window.emit("rua://update-rua-config", &config.rua)?;
                }
                ConfigMsg::RestartCore => {
                    info!("Restarting core");
                    let mut config = msg_config.lock().await;
                    config.rua.core_status = CoreStatus::Restarting;
                    main_window.emit("rua://update-rua-config", &config.rua)?;
                    let mut core = msg_core.lock()?;
                    match core.restart() {
                        Ok(_) => {
                            config.rua.core_status = CoreStatus::Started;
                            main_window.emit("rua://update-rua-config", &config.rua)?;
                            main_window.emit("rua://update-core-config", &config.core)?;
                        }
                        Err(err) => {
                            error!("Core restart failed {err}");
                        }
                    }
                }
            }
        }
        Ok::<(), VError>(())
    };
    async_runtime::spawn(handler);
    Ok(())
}
