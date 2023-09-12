use std::sync::atomic::Ordering;

use anyhow::{anyhow, Result};
use log::error;
use tauri::{
    async_runtime, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayMenu,
    SystemTrayMenuItem, SystemTrayMenuItemHandle, WindowBuilder, WindowUrl,
};

use crate::{core::AVCore, utils::toggle_windows, CORE_SHUTDOWN};

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

/// Handle system tray window create
pub fn tray_menu(app: &AppHandle) {
    let app_handle = app.app_handle();
    async_runtime::spawn(async move {
        match handle_tray_menu(&app_handle) {
            Ok(_) => {}
            Err(err) => {
                error!("Create system tray menu failed {}", err)
            }
        }
    });
}
/// Create new system tray window
pub fn handle_tray_menu(app: &AppHandle) -> Result<()> {
    use tauri_plugin_positioner::{Position, WindowExt};

    let menu = {
        let window = app.get_window("menu");
        if let Some(win) = window {
            win
        } else {
            WindowBuilder::new(app, "menu", WindowUrl::App("system-tray".into())).build()?
        }
    };
    menu.set_always_on_top(true)?;
    menu.set_decorations(false)?;
    menu.move_window(Position::TrayCenter)?;
    Ok(())
    // let show_handle = app.tray_handle().get_item("hide");
    // match handle_visible(app, &show_handle, Some(true)) {
    //     Ok(_) => {}
    //     Err(err) => {
    //         error!("handle windows visible failed {}", err)
    //     }
    // }
}

/// Handle system tray menu item right-click.
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
        "hide" => match handle_visible(app, &item_handle, None) {
            Ok(_) => {}
            Err(err) => {
                error!("handle windows visible failed {}", err)
            }
        },
        _ => {}
    }
}

// For right click "hide" menu
fn handle_visible(
    app: &AppHandle,
    item_handle: &SystemTrayMenuItemHandle,
    overide: Option<bool>,
) -> Result<()> {
    let windows = app.windows();
    let main_window = app
        .get_window("main")
        .ok_or(anyhow!("cannot get main window".to_owned()))?;

    let main_visible = main_window.is_visible()?;
    let show = if let Some(s) = overide {
        item_handle.set_title("Hide")?;
        !s
    } else {
        if main_visible {
            item_handle.set_title("Show")?;
        } else {
            item_handle.set_title("Hide")?;
        }
        main_visible
    };
    toggle_windows(windows, !show)?;

    Ok(())
}
