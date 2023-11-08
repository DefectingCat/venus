#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::{
    commands::{
        config::{get_config, update_config},
        core::select_node,
        node_speed,
        subs::{add_subscription, update_all_subs, update_sub},
        ui::toggle_main,
    },
    core::VCore,
    event::RUAEvents,
    logger::init_logger,
    message::{message_handler, ConfigMsg, MSG_TX},
    store::ui::CoreStatus,
    tray::tray_menu,
    utils::get_main_window,
};
use anyhow::{anyhow, Ok as AOk};
use config::VConfig;
use log::{error, info};
use once_cell::sync::Lazy;
use std::{
    env,
    error::Error,
    sync::atomic::{AtomicBool, Ordering},
};
use store::ui::UI;
use tauri::{
    async_runtime, App, AppHandle, Manager, RunEvent, SystemTray, SystemTrayEvent, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};
use tokio::sync::Mutex;

mod commands;
mod config;
mod core;
mod event;
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

    // App handler
    let handle_app = move |app: &mut App| -> Result<(), Box<dyn Error>> {
        #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);

        let resources_path = app
            .handle()
            .path_resolver()
            .resolve_resource("resources/")
            .ok_or(anyhow!("resource path is empty"))?;
        // Init config and core
        let window = get_main_window(app)?;
        // Start config and core
        async_runtime::spawn(async move {
            let mut config = CONFIG.lock().await;
            info!("Start init config");
            match config.init(&resources_path) {
                Ok(_) => info!("Config init sucess"),
                Err(err) => {
                    error!("Init config failed {}", err);
                    return AOk(());
                }
            }
            // Restore alll window status.
            if config.rua.save_windows {
                window.restore_state(StateFlags::all())?;
            }

            info!("Start core");
            let mut core = CORE.lock().await;
            // Set v2ray assert location with environment
            env::set_var("V2RAY_LOCATION_ASSET", &resources_path);

            let mut ui = UI.lock().await;
            match core.init(&config.core_path).await {
                Ok(_) => {
                    ui.core_status = CoreStatus::Started;
                    info!("Core started");
                }
                Err(err) => {
                    error!("Core start failed {err:?}");
                    CORE_SHUTDOWN.store(false, Ordering::Relaxed);
                    ui.core_status = CoreStatus::Stopped;
                }
            }
            AOk(())
        });

        let window = get_main_window(app)?;
        app.listen_global("ready", move |_e| {
            use RUAEvents::*;
            info!("Frontend ready");
            let window = window.get_window("main").unwrap();
            let task = async move {
                let config = CONFIG.lock().await;
                let ui = UI.lock().await;
                window.emit_all(UpdateRuaConfig.into(), &config.rua)?;
                window.emit_all(UpdateCoreConfig.into(), &config.core)?;
                window.emit_all(UpdateUI.into(), &*ui)?;
                info!("Reload config succeeded");
                AOk(())
            };
            async_runtime::spawn(task);
        });

        let window = get_main_window(app)?;
        // The config will use receiver here
        // when got a message, config will update and
        // emit a event to notify frontend to update global state
        message_handler(window)?;
        Ok(())
    };

    // Used to runner to manage core process
    // Runner handler
    let runner = move |app: &AppHandle, event: RunEvent| match event {
        RunEvent::Exit => {
            async_runtime::spawn(async move {
                let mut core = CORE.lock().await;
                CORE_SHUTDOWN.store(true, Ordering::Relaxed);
                core.exit().expect("Kill core failed");
            });
        }
        RunEvent::ExitRequested { api, .. } => {
            let _api = api;
        }
        RunEvent::WindowEvent { label, event, .. } => match event {
            WindowEvent::CloseRequested { api, .. } => {
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
            WindowEvent::Focused(is_focused) => {
                async_runtime::spawn(async move {
                    {
                        let mut ui = UI.lock().await;
                        ui.main_visible = is_focused;
                    }
                    if label == "main" {
                        MSG_TX.lock().await.send(ConfigMsg::EmitUI).await?;
                    }
                    AOk(())
                });
            }
            _ => {}
        },
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
            // common commands
            node_speed,
            // ui
            toggle_main
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
        .setup(handle_app)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(runner);
}
