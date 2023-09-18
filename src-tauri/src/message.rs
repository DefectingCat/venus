use anyhow::{Ok as AOk, Result};
use log::{error, info};
use once_cell::sync::Lazy;
use tauri::{async_runtime, Manager, Window};

use crate::{config::CoreStatus, CONFIG, CORE};

use tokio::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

// Init message
// Create a mpsc channel for config and other stuff,
// when other stuff change state and need to update config
// it will use tx send new state to config
pub static mut MSG: Lazy<(Sender<ConfigMsg>, Receiver<ConfigMsg>)> = Lazy::new(msg_build);
// Message channel sender
pub static MSG_TX: Lazy<&Sender<ConfigMsg>> = Lazy::new(|| unsafe { &MSG.0 });
// Message channel receiver
// pub static MSG_RX: Lazy<&Receiver<ConfigMsg>> = Lazy::new(|| unsafe { &MSG.1 });

#[derive(Debug)]
pub enum ConfigMsg {
    CoreStatus(CoreStatus),
    RestartCore,
    EmitLog(String),
    EmitConfig,
}
// pub struct ConfigMsg {
//     pub msg: ConfigMsgType,
// }

// pub type MsgSender = Arc<Sender<ConfigMsg>>;
// pub type MsgReceiver = Receiver<ConfigMsg>;

/// Build message channel.
pub fn msg_build() -> (Sender<ConfigMsg>, Receiver<ConfigMsg>) {
    mpsc::channel::<ConfigMsg>(128)
}

/// Handle core message and emit log to frontend
pub unsafe fn message_handler(window: Window) -> Result<()> {
    let handler = async move {
        while let Some(msg) = MSG.1.recv().await {
            match msg {
                ConfigMsg::CoreStatus(status) => {
                    info!("Update core status {}", status.as_str());
                    let mut config = CONFIG.lock().await;
                    config.rua.core_status = status;
                    window.emit_all("rua://update-rua-config", &config.rua)?;
                }
                ConfigMsg::RestartCore => {
                    info!("Restarting core");
                    let mut core = CORE.lock().await;
                    match core.restart().await {
                        Ok(_) => {
                            let mut config = CONFIG.lock().await;
                            config.rua.core_status = CoreStatus::Started;
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
                ConfigMsg::EmitConfig => {
                    let config = CONFIG.lock().await;
                    window.emit_all("rua://update-rua-config", &config.rua)?;
                    window.emit_all("rua://update-core-config", &config.core)?;
                }
            }
        }
        AOk(())
    };
    async_runtime::spawn(handler);
    Ok(())
}
