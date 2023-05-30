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
    CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowEvent,
};

use crate::{
    commands::common::{add_subscription, get_rua_nodes, get_subscriptions},
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
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {}
            SystemTrayEvent::DoubleClick { .. } => {
                let windows = app.windows();
                for (_, window) in windows {
                    window.show().unwrap()
                }
            }
            SystemTrayEvent::MenuItemClick { tray_id, id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "quit" => {
                        todo!()
                    }
                    "hide" => {
                        let main_window = app.get_window("main").expect("Can not get main window");
                        main_window.hide().expect("Can not hide main window");
                        item_handle
                            .set_title("Show")
                            .expect("Can not set title to menu item");
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

// fn tray_config() {}
