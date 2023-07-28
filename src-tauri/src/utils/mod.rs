use log::{error, info};
use std::sync::Arc;
use tauri::{async_runtime, App, Manager, Window};
use tokio::sync::{mpsc::Receiver, Mutex};

use crate::{config::VConfig, core::VCore, message::ConfigMsg};

use self::error::{VError, VResult};

pub mod error;

/// Handle core message and emit log to frontend
pub fn message_handler(
    window: Window,
    mut rx: Receiver<ConfigMsg>,
    msg_config: Arc<Mutex<VConfig>>,
    msg_core: Arc<Mutex<VCore>>,
) -> VResult<()> {
    let handler = async move {
        while let Some(msg) = rx.recv().await {
            let mut core = msg_core.lock().await;
            match msg {
                ConfigMsg::CoreStatus(status) => {
                    info!("Update core status {}", status.as_str());
                    let mut config = msg_config.lock().await;
                    config.rua.core_status = status;
                    window.emit_all("rua://update-rua-config", &config.rua)?;
                }
                ConfigMsg::RestartCore => {
                    info!("Restarting core");
                    match core.restart().await {
                        Ok(_) => {
                            let config = msg_config.lock().await;
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
                ConfigMsg::NodeSpeedtest(node_ids) => {
                    core.speed_test(node_ids, msg_config.clone()).await?;
                }
                ConfigMsg::EmitConfig => {
                    dbg!("try");
                    let config = msg_config.lock().await;
                    window.emit_all("rua://update-rua-config", &config.rua)?;
                    window.emit_all("rua://update-core-config", &config.core)?;
                }
            }
        }
        Ok::<(), VError>(())
    };
    async_runtime::spawn(handler);
    Ok(())
}

/// Get main window by app
pub fn get_main_window(app: &App) -> VResult<Window> {
    let window = app
        .get_window("main")
        .ok_or(VError::WindowError("Can not get main window"))?;
    Ok(window)
}
