use std::sync::Arc;

use log::{error, info};
use tauri::{async_runtime, Manager, Window};
use tokio::sync::Mutex;

use crate::{
    config::{CoreStatus, VConfig},
    core::VCore,
    utils::error::{VError, VResult},
};

use tokio::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

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

pub type MsgSender = Arc<Sender<ConfigMsg>>;
pub type MsgReceiver = Receiver<ConfigMsg>;

pub fn msg_build() -> (Sender<ConfigMsg>, Receiver<ConfigMsg>) {
    mpsc::channel::<ConfigMsg>(128)
}

/// Handle core message and emit log to frontend
pub fn message_handler(
    window: Window,
    mut rx: MsgReceiver,
    msg_config: Arc<Mutex<VConfig>>,
    msg_core: Arc<Mutex<VCore>>,
) -> VResult<()> {
    let handler = async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                ConfigMsg::CoreStatus(status) => {
                    info!("Update core status {}", status.as_str());
                    let mut config = msg_config.lock().await;
                    config.rua.core_status = status;
                    window.emit_all("rua://update-rua-config", &config.rua)?;
                }
                ConfigMsg::RestartCore => {
                    info!("Restarting core");
                    let mut core = msg_core.lock().await;
                    match core.restart().await {
                        Ok(_) => {
                            let mut config = msg_config.lock().await;
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
