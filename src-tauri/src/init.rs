use crate::{
    commands::subs::{timer_update, update_all_subs_core},
    config::SubsAutoUpdate,
    core::{core_version, exit_core},
    event::RUAEvents,
    message::{message_handler, MSG_TX},
    store::ui::CoreStatus,
    utils::get_main_window,
    CONFIG, CORE, CORE_SHUTDOWN, UI,
};
use anyhow::{anyhow, Ok as AOk, Result};
use log::{error, info};
use std::{env, error::Error, path::PathBuf, sync::atomic::Ordering, thread};
use tauri::{
    async_runtime, App, AppHandle, GlobalWindowEvent, Manager, RunEvent, Window, WindowEvent,
};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

/// Setup app and init configs. used for `tauri::Builder::default().setup()`
///
/// ## Arguments
///
/// `app`: current app instance
pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {
    let resources_path = app
        .handle()
        .path_resolver()
        .resolve_resource("resources/")
        .ok_or(anyhow!("resource path is empty"))?;
    // Init config and core
    let window = get_main_window(app)?;
    // Start config and core
    async_runtime::spawn(async move {
        let _ = init_core_and_config(&resources_path, window)
            .await
            .map_err(|e| error!("init core and config failed {e}"));
        let _ = after_app_setup()
            .await
            .map_err(|e| error!("after app setup failed {e}"));
        AOk(())
    });

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    let window = get_main_window(app)?;
    app.listen_global("ready", move |_e| {
        use RUAEvents::*;
        let window = window.get_window("main").expect("");
        let task = async move {
            let config = CONFIG.lock().await;
            let ui = UI.lock().await;
            window.emit_all(UpdateRuaConfig.into(), &config.rua)?;
            window.emit_all(UpdateCoreConfig.into(), &config.core)?;
            window.emit_all(UpdateUI.into(), &*ui)?;
            info!("Reload config succeeded");
            AOk(())
        };
        info!("Frontend ready");
        async_runtime::spawn(task);
    });

    let window = get_main_window(app)?;
    // The config will use receiver here
    // when got a message, config will update and
    // emit a event to notify frontend to update global state
    thread::spawn(move || {
        message_handler(window)?;
        AOk(())
    });
    Ok(())
}

/// Init v2ray core and config.
///
/// ## Arguments
///
/// `resources_path`: the config store path
/// `window`: the tauri window manager
///
/// ## Lock
///
/// This function will lock the core and config until function end
async fn init_core_and_config(resources_path: &PathBuf, window: Window) -> Result<()> {
    let mut config = CONFIG.lock().await;
    info!("Start init config");
    match config.init(resources_path) {
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
    env::set_var("V2RAY_LOCATION_ASSET", resources_path);

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
    ui.core_version = core_version()?;
    AOk(())
}

/// After app initialized
async fn after_app_setup() -> Result<()> {
    let mut config = CONFIG.lock().await;
    info!("Start init config");

    match config.rua.settings.update_subs {
        Some(SubsAutoUpdate::Startup) => {
            update_all_subs_core(&mut config).await?;
            MSG_TX
                .lock()
                .await
                .send(crate::message::ConfigMsg::RestartCore)
                .await?;
        }
        Some(SubsAutoUpdate::Time) => {
            let duration = config.rua.settings.update_time;
            timer_update(duration).await;
        }
        Some(SubsAutoUpdate::Off) => {}
        None => {}
    };
    Ok(())
}

/// App runtime handler. used for `tauri::Builder::default().run()`
/// for handle runtime events
///
/// ## Arguments
///
/// `app`: current tauri app handler
/// `event`: tauri runtime events
pub fn app_runtime(app: &AppHandle, event: RunEvent) {
    match event {
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
            let window_task = async move {
                let config = CONFIG.lock().await;
                if config.rua.save_windows {
                    if let Err(err) = app_handler.save_window_state(StateFlags::all()) {
                        error!("Save window status failed {}", err)
                    };
                }
                AOk(())
            };
            async_runtime::spawn(window_task);
        }
        _ => {}
    }
}

/// Hanlde rumtime global tauri window event
/// used for `tauri::Builder::default().on_window_event()`
pub fn window_event_handler(event: GlobalWindowEvent) {
    if let tauri::WindowEvent::Focused(is_focused) = event.event() {
        let name = event.window().label();
        if !is_focused && name == "menu" {
            if let Err(err) = event.window().hide() {
                error!("hide window failed {}", err)
            }
        }
    };
}

#[derive(Clone, serde::Serialize)]
struct SingleInstancePayload {
    args: Vec<String>,
    cwd: String,
}

/// Init tauri_plugin_single_instance plugin
pub fn single_instance_init(app: &AppHandle, argv: Vec<String>, cwd: String) {
    info!("{}, {argv:?}, {cwd}", app.package_info().name);
    match app.emit_all("single-instance", SingleInstancePayload { args: argv, cwd }) {
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
}
