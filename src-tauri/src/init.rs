use crate::{
    core::core_version, event::RUAEvents, message::message_handler, store::ui::CoreStatus,
    utils::get_main_window, CONFIG, CORE, CORE_SHUTDOWN, UI,
};
use anyhow::{anyhow, Ok as AOk, Result};
use log::{error, info};
use std::{env, error::Error, path::PathBuf, sync::atomic::Ordering, thread};
use tauri::{async_runtime, App, Manager, Window};
use tauri_plugin_window_state::{StateFlags, WindowExt};

/// Setup app and init configs. used for `tauri::Builder::default().setup()`
///
/// ## Arguments
///
/// `app`: current app instance
pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {
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
        init_core_and_config(&resources_path, &window).await?;
        AOk(())
    });

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
async fn init_core_and_config(resources_path: &PathBuf, window: &Window) -> Result<()> {
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
