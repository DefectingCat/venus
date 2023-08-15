use std::sync::atomic::Ordering;

use log::error;
use tauri::{
    async_runtime, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayMenu,
    SystemTrayMenuItem, SystemTrayMenuItemHandle,
};

use crate::{
    core::AVCore,
    utils::{
        error::{VError, VResult},
        toggle_windows,
    },
    CORE_SHUTDOWN,
};

/// Build new system tray.
pub fn new_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

/// Handle system tray menu item click.
pub fn handle_tray_click(app: &AppHandle, id: String, core: &AVCore) {
    let item_handle = app.tray_handle().get_item(&id);
    match id.as_str() {
        "quit" => {
            let core = core.clone();
            let app = app.app_handle();
            async_runtime::spawn(async move {
                let mut core = core.lock().await;
                CORE_SHUTDOWN.store(true, Ordering::Relaxed);
                core.exit().expect("Kill core failed");
                app.exit(0);
            });
        }
        "hide" => match hide_windows(app, &item_handle) {
            Ok(_) => {}
            Err(err) => {
                error!("hide windows failed {}", err)
            }
        },
        _ => {}
    }
}

fn hide_windows(app: &AppHandle, item_handle: &SystemTrayMenuItemHandle) -> VResult<()> {
    use VError::CommonError;

    let windows = app.windows();
    let main_window = app
        .get_window("main")
        .ok_or(CommonError("cannot get main window".to_owned()))?;
    let main_visible = main_window.is_visible()?;
    toggle_windows(windows, !main_visible)?;
    if main_visible {
        item_handle.set_title("Show")?;
    } else {
        item_handle.set_title("Hide")?;
    }
    Ok(())
}
