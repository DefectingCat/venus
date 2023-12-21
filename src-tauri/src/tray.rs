use anyhow::Result;
use log::error;
use tauri::{AppHandle, LogicalSize, Manager, Size, WindowBuilder, WindowUrl};

// Build new system tray.
// pub fn new_tray() -> SystemTray {
//     let quit = CustomMenuItem::new("quit".to_string(), "Quit");
//     let hide = CustomMenuItem::new("hide".to_string(), "Hide");
//     let tray_menu = SystemTrayMenu::new()
//         .add_item(hide)
//         .add_native_item(SystemTrayMenuItem::Separator)
//         .add_item(quit);
//     SystemTray::new().with_menu(tray_menu)
// }

/// Handle system tray window create
pub fn tray_menu(app: &AppHandle) {
    let app_handle = app.app_handle();
    std::thread::spawn(move || match handle_tray_menu(&app_handle) {
        Ok(_) => {}
        Err(err) => {
            error!("Create system tray menu failed {}", err)
        }
    });
}
/// Create new system tray window
pub fn handle_tray_menu(app: &AppHandle) -> Result<()> {
    use tauri_plugin_positioner::{Position, WindowExt};

    // get or create a new menu window
    let (menu, is_build) = {
        let window = app.get_window("menu");
        if let Some(win) = window {
            (win, false)
        } else {
            let win = WindowBuilder::new(app, "menu", WindowUrl::App("system-tray".into()))
                .decorations(false)
                .transparent(true)
                .build()?;
            win.set_size(Size::Logical(LogicalSize {
                width: 300.0,
                height: 600.0,
            }))?;
            (win, true)
        }
    };

    if menu.is_visible()? && !is_build {
        menu.hide()?;
    } else {
        menu.move_window(Position::TrayCenter)?;
        menu.show()?;
        menu.set_focus()?;
        menu.set_always_on_top(true)?;
    }
    Ok(())
}

// Handle system tray menu item right-click.
// pub fn handle_tray_click(app: &AppHandle, id: String) {
//     let item_handle = app.tray_handle().get_item(&id);
//     match id.as_str() {
//         "quit" => {
//             let app = app.app_handle();
//             async_runtime::spawn(async move {
//                 let mut core = CORE.lock().await;
//                 CORE_SHUTDOWN.store(true, Ordering::Relaxed);
//                 core.exit().expect("Kill core failed");
//                 app.exit(0);
//             });
//         }
//         "hide" => match handle_visible(app, &item_handle, None) {
//             Ok(_) => {}
//             Err(err) => {
//                 error!("handle windows visible failed {}", err)
//             }
//         },
//         _ => {}
//     }
// }
// For right click "hide" menu
// fn handle_visible(
//     app: &AppHandle,
//     item_handle: &SystemTrayMenuItemHandle,
//     overide: Option<bool>,
// ) -> Result<()> {
//     let windows = app.windows();
//     let main_window = app
//         .get_window("main")
//         .ok_or(anyhow!("cannot get main window".to_owned()))?;

//     let main_visible = main_window.is_visible()?;
//     let show = if let Some(s) = overide {
//         item_handle.set_title("Hide")?;
//         !s
//     } else {
//         if main_visible {
//             item_handle.set_title("Show")?;
//         } else {
//             item_handle.set_title("Hide")?;
//         }
//         main_visible
//     };
//     toggle_windows(windows, !show)?;
//     Ok(())
// }
