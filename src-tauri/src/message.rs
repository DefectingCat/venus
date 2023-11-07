use crate::{config::CoreStatus, event::RUAEvents, CONFIG, CORE};
use anyhow::{Ok as AOk, Result};
use log::{error, info};
use once_cell::sync::Lazy;
use tauri::{async_runtime, Manager, Window};
use tokio::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
    Mutex,
};

type VMessage = Lazy<(Mutex<Sender<ConfigMsg>>, Mutex<Receiver<ConfigMsg>>)>;
// Init message
// Create a mpsc channel for config and other stuff,
// when other stuff change state and need to update config
// it will use tx send new state to config
pub static MSG: VMessage = Lazy::new(|| {
    let msg = msg_build();
    (Mutex::new(msg.0), Mutex::new(msg.1))
});
pub static MSG_TX: Lazy<&Mutex<Sender<ConfigMsg>>> = Lazy::new(|| &MSG.0);

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
pub fn message_handler(window: Window) -> Result<()> {
    use RUAEvents::*;

    let handler = async move {
        while let Some(msg) = MSG.1.lock().await.recv().await {
            match msg {
                ConfigMsg::CoreStatus(status) => {
                    info!("Update core status {}", status.as_str());
                    let mut config = CONFIG.lock().await;
                    config.rua.core_status = status;
                    window.emit_all(UpdateRuaConfig.into(), &config.rua)?;
                }
                ConfigMsg::RestartCore => {
                    info!("Restarting core");
                    let mut core = CORE.lock().await;
                    match core.restart().await {
                        Ok(_) => {
                            let mut config = CONFIG.lock().await;
                            config.rua.core_status = CoreStatus::Started;
                            window.emit_all(UpdateRuaConfig.into(), &config.rua)?;
                            window.emit_all(UpdateCoreConfig.into(), &config.core)?;
                        }
                        Err(err) => {
                            error!("Core restart failed {err}");
                        }
                    }
                }
                ConfigMsg::EmitLog(log) => {
                    window.emit(EmitLog.into(), log)?;
                }
                ConfigMsg::EmitConfig => {
                    let config = CONFIG.lock().await;
                    window.emit_all(UpdateRuaConfig.into(), &config.rua)?;
                    window.emit_all(UpdateCoreConfig.into(), &config.core)?;
                }
            }
        }
        AOk(())
    };
    async_runtime::spawn(handler);
    Ok(())
}
