#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::common::current_dir;
use config::VConfig;
use env_logger::Env;
use log::{error, info};
use std::sync::{Arc, Mutex};
use tauri::{
    async_runtime, CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowEvent,
};

use crate::{
    commands::{
        common::{get_rua_nodes, get_subscriptions},
        subs::add_subscription,
    },
    config::CoreStatus,
    core::VCore,
};

mod commands;
mod config;
mod consts;
mod core;
mod utils;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    // Init config.
    let config = Arc::new(tauri::async_runtime::Mutex::new(VConfig::new()));

    let env = Env::default().filter_or("RUA_LOG_LEVEL", "info");
    env_logger::init_from_env(env);
    info!("starting up.");
    info!("V2rayR - {}", env!("CARGO_PKG_VERSION"));

    info!("Start core");
    let core = match VCore::build() {
        Ok(core) => {
            let config = config.clone();
            async_runtime::spawn(async move {
                let mut config = config.lock().await;
                config.core_status = CoreStatus::Started;
            });
            Arc::new(Mutex::new(Some(core)))
        }
        Err(err) => {
            error!("Core start failed {err:?}");
            Arc::new(Mutex::new(None))
        }
    };

    let config_state = config.clone();
    let tray_core = core.clone();
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
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "quit" => {
                        let mut core = tray_core.lock().expect("");
                        if let Some(core) = core.as_mut() {
                            core.exit().expect("")
                        }
                        app.exit(0);
                    }
                    "hide" => {
                        let main_window = app.get_window("main").expect("Can not get main window");
                        let main_visible = main_window
                            .is_visible()
                            .expect("Failed to detect window visible");
                        if main_visible {
                            main_window.hide().expect("Can not hide main window");
                            item_handle
                                .set_title("Show")
                                .expect("Can not set title to menu item");
                        } else {
                            main_window.show().expect("Can not show main window");
                            item_handle
                                .set_title("Hide")
                                .expect("Can not set title tray title");
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .manage(config_state)
        .invoke_handler(tauri::generate_handler![
            current_dir,
            add_subscription,
            get_rua_nodes,
            get_subscriptions,
        ])
        .setup(move |app| {
            let resolver = app.handle().path_resolver();
            let core_path = resolver
                .resolve_resource("resources/config.json")
                .expect("can not found config file");
            let rua_path = resolver
                .resolve_resource("resources/config.toml")
                .expect("can not found rua config file");
            async_runtime::spawn(async move {
                let mut config = config.lock().await;
                config
                    .init(core_path, rua_path)
                    .expect("can not init core config");
            });

            app.listen_global("ready", |_e| {
                info!("Got front ready event");
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app, event| match event {
            RunEvent::Exit => {
                let mut core = core.lock().expect("");
                if let Some(core) = core.as_mut() {
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
                let tray_handle = app.tray_handle().get_item("hide");
                tray_handle
                    .set_title("Show")
                    .expect("Can not set tray title");
            }
            _ => {}
        });
}

// fn tray_config() {}
