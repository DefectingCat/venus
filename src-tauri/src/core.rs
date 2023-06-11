use std::{
    env,
    path::PathBuf,
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
    asset_path: String,
}

fn start_core(tx: Arc<Sender<ConfigMsg>>, path: &str) -> VResult<CommandChild> {
    // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
    let (mut rx, child) = Command::new_sidecar("v2ray")
        .expect("Failed to create `v2ray` binary command")
        .args(["run", "-c", &format!("{path}/config.json")])
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
                        warn!("Kill core {line:?}");
                    } else {
                        error!("{line:?}");
                        tx.send(ConfigMsg::CoreStatue(CoreStatus::Stopped))
                            .await
                            .expect("Cannot send config msg");
                    }
                }
                _ => {
                    tx.send(ConfigMsg::CoreStatue(CoreStatus::Stopped))
                        .await
                        .expect("Cannot send config msg");
                    error!("Core unknown error {event:?}");
                }
            }
        }
    });

    Ok(child)
}

impl VCore {
    pub fn build(tx: Arc<Sender<ConfigMsg>>) -> Self {
        Self {
            child: None,
            tx,
            asset_path: String::new(),
        }
    }

    /// Init core add assets path and start core
    pub fn init(&mut self, asset_path: Option<PathBuf>) -> VResult<()> {
        let asset_path = asset_path.ok_or(VError::ResourceError("resource path is empty"))?;
        let path = asset_path
            .to_str()
            .ok_or(VError::ResourceError("resource path is empty"))?;
        // Set v2ray assert location with environment
        env::set_var("V2RAY_LOCATION_ASSET", path);
        self.child = Some(start_core(self.tx.clone(), path)?);
        self.asset_path = path.into();
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
