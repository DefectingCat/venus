use std::{
    path::{Path, PathBuf},
    sync::{atomic::Ordering, Arc, Mutex},
};

use log::{error, info, warn};
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    async_runtime,
};
use tokio::sync::mpsc::Sender;

use crate::{
    config::CoreStatus,
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
    asset_path: PathBuf,
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
                        tx.send(ConfigMsg::CoreStatue(CoreStatus::Stopped)).await?;
                    }
                }
                _ => {
                    tx.send(ConfigMsg::CoreStatue(CoreStatus::Stopped)).await?;
                    error!("Core unknown error {event:?}");
                }
            }
        }
        Ok::<(), VError>(())
    });

    Ok(child)
}

impl VCore {
    pub fn build(tx: Arc<Sender<ConfigMsg>>) -> Self {
        Self {
            child: None,
            tx,
            asset_path: PathBuf::new(),
        }
    }

    /// Init core add assets path and start core
    pub fn init(&mut self, asset_path: &Path) -> VResult<()> {
        self.asset_path = PathBuf::from(asset_path);
        self.child = Some(start_core(self.tx.clone(), &self.asset_path)?);
        Ok(())
    }

    /// Restart core and reload config
    pub fn restart(&mut self) -> VResult<()> {
        if let Some(child) = self.child.take() {
            CORE_SHUTDOWN.store(true, Ordering::Relaxed);
            child.kill()?;
        } else {
            warn!("core process not exist");
            return Ok(());
        };
        let child = start_core(self.tx.clone(), &self.asset_path)?;
        self.child = Some(child);
        Ok(())
    }

    pub fn exit(&mut self) -> VResult<()> {
        if let Some(child) = self.child.take() {
            info!("Exiting core");
            child.kill()?;
        };
        Ok(())
    }
}
