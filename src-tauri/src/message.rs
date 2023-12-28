use crate::{event::RUAEvents, store::ui::CoreStatus, CONFIG, CORE, UI};
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
    /// change core status
    CoreStatus(CoreStatus),
    /// restart core and notifiy frontend update ui
    RestartCore,
    /// emit single line log to frontend
    EmitLog(String),
    /// emit core and rua config to frontend
    EmitConfig,
    // emit whole ui to fronted
    // EmitUI,
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
                    let mut ui = UI.lock().await;
                    ui.core_status = status;
                    window.emit_all(UpdateUI.into(), &*ui)?;
                }
                ConfigMsg::RestartCore => {
                    info!("Restarting core");
                    let mut core = CORE.lock().await;
                    let mut ui = UI.lock().await;
                    ui.core_status = CoreStatus::Restarting;
                    match core.restart().await {
                        Ok(_) => {
                            let config = CONFIG.lock().await;
                            ui.core_status = CoreStatus::Started;
                            window.emit_all(UpdateUI.into(), &*ui)?;
                            window.emit_all(UpdateCoreConfig.into(), &config.core)?;
                            window.emit_all(UpdateRuaConfig.into(), &config.rua)?;
                        }
                        Err(err) => {
                            error!("Core restart failed {err}");
                            let config = CONFIG.lock().await;
                            ui.core_status = CoreStatus::Stopped;
                            window.emit_all(UpdateUI.into(), &*ui)?;
                            window.emit_all(UpdateCoreConfig.into(), &config.core)?;
                            window.emit_all(UpdateRuaConfig.into(), &config.rua)?;
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
                } /* ConfigMsg::EmitUI => {
                      let ui = UI.lock().await;
                      window.emit_all(UpdateUI.into(), &*ui)?;
                  } */
            }
        }
        AOk(())
    };
    async_runtime::spawn(handler);
    Ok(())
}
