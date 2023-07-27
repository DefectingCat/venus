use std::{
    path::Path,
    sync::{atomic::Ordering, Arc},
};

use log::{error, info, warn};
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    async_runtime,
};
use tokio::sync::{mpsc::Sender, Mutex};

use crate::{
    config::{ConfigState, CoreStatus},
    message::ConfigMsg,
    utils::error::{VError, VResult},
    CORE_SHUTDOWN,
};

pub type AVCore = Arc<Mutex<VCore>>;

#[derive(Debug)]
pub struct VCore {
    // Slidecare process
    pub child: Option<CommandChild>,
    // Message sender
    pub tx: Arc<Sender<ConfigMsg>>,
    // Core resource path
    config: Option<ConfigState>,
}

fn start_core(tx: Arc<Sender<ConfigMsg>>, path: &Path) -> VResult<CommandChild> {
    // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
    let (mut rx, child) = Command::new_sidecar("v2ray")
        .expect("Failed to create `v2ray` binary command")
        .args(["run", "-c", &path.to_string_lossy()])
        .spawn()
        .expect("Failed to spawn sidecar");

    async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    info!("{line}");
                }
                CommandEvent::Stderr(line) => {
                    warn!("{line}");
                }
                CommandEvent::Terminated(line) => {
                    if CORE_SHUTDOWN.load(Ordering::Relaxed) {
                        info!("Kill core succeed");
                    } else {
                        error!("{line:?}");
                        tx.send(ConfigMsg::CoreStatus(CoreStatus::Stopped)).await?;
                    }
                }
                _ => {
                    tx.send(ConfigMsg::CoreStatus(CoreStatus::Stopped)).await?;
                    error!("Core unknown error {event:?}");
                }
            }
        }
        Ok::<(), VError>(())
    });

    Ok(child)
}

impl VCore {
    /// tx: restart core message
    pub fn build(tx: Arc<Sender<ConfigMsg>>) -> Self {
        Self {
            child: None,
            tx,
            config: None,
        }
    }

    /// Init core add assets path and start core
    pub async fn init(&mut self, config: ConfigState) {
        self.config = Some(config.clone());
        let mut config = config.lock().await;
        // self.child = Some(start_core(self.tx.clone(), &config.core_path)?);
        match start_core(self.tx.clone(), &config.core_path) {
            Ok(child) => {
                config.rua.core_status = CoreStatus::Started;
                info!("Core started");
                self.child = Some(child);
            }
            Err(err) => {
                error!("Core start failed {err:?}");
                CORE_SHUTDOWN.store(false, Ordering::Relaxed);
                config.rua.core_status = CoreStatus::Stopped;
            }
        }
    }

    /// Restart core and reload config
    pub async fn restart(&mut self) -> VResult<()> {
        if let Some(child) = self.child.take() {
            CORE_SHUTDOWN.store(true, Ordering::Relaxed);
            child.kill()?;
        } else {
            warn!("core process not exist");
            return Ok(());
        };
        let mut config = self
            .config
            .as_mut()
            .ok_or(VError::EmptyError("()"))?
            .lock()
            .await;
        // let child = start_core(self.tx.clone(), &config.core_path)?;
        match start_core(self.tx.clone(), &config.core_path) {
            Ok(child) => {
                config.rua.core_status = CoreStatus::Started;
                self.child = Some(child);
            }
            Err(err) => {
                error!("Core restart failed {err}");
            }
        }
        Ok(())
    }

    pub fn exit(&mut self) -> VResult<()> {
        if let Some(child) = self.child.take() {
            info!("Exiting core");
            child.kill()?;
        };
        Ok(())
    }

    pub fn speed_test(&mut self) -> VResult<()> {
        Ok(())
    }
}
