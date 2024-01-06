use crate::{
    message::{ConfigMsg, MSG_TX},
    store::ui::CoreStatus,
    CORE, CORE_SHUTDOWN,
};
use anyhow::{Context, Ok as AOk, Result};
use log::{error, info, warn};
use once_cell::sync::Lazy;
use std::{
    path::{Path, PathBuf},
    sync::atomic::Ordering,
};
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    async_runtime,
};
use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Debug)]
pub struct VCore {
    // Slidecare process
    pub child: Option<CommandChild>,
    // Core resource path
    asset_path: PathBuf,
}

/// detect the v2ray core version
pub fn core_version() -> Result<String> {
    let core = Command::new_sidecar("v2ray")?.args(["version"]).output()?;
    let stdout = core.stdout.split(' ').collect::<Vec<_>>();
    let stdout = stdout.get(1).unwrap_or(&"0.0");
    Ok(stdout.to_string())
}

#[derive(Debug, Copy, Clone)]
pub enum CoreMessage {
    Starting,
    Started,
    Stopping,
    Stopped,
}
pub static CORE_MSG: Lazy<(Sender<CoreMessage>, Receiver<CoreMessage>)> =
    Lazy::new(|| broadcast::channel(64));
pub static CORE_MSG_TX: Lazy<&Sender<CoreMessage>> = Lazy::new(|| &CORE_MSG.0);

fn start_core(path: &Path) -> Result<CommandChild> {
    // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
    CORE_MSG_TX.send(CoreMessage::Starting)?;
    let (mut rx, child) = Command::new_sidecar("v2ray")?
        .args(["run", "-c", &path.to_string_lossy()])
        .spawn()?;

    async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    if line.contains("started") {
                        CORE_MSG_TX.send(CoreMessage::Started)?;
                    }
                    info!("{line}");
                }
                CommandEvent::Stderr(line) => {
                    warn!("{line}");
                }
                CommandEvent::Terminated(line) => {
                    CORE_MSG_TX.send(CoreMessage::Starting)?;
                    if CORE_SHUTDOWN.load(Ordering::Relaxed) {
                        info!("Kill core succeed");
                        CORE_MSG_TX.send(CoreMessage::Stopped)?;
                    } else {
                        error!("{line:?}");
                        MSG_TX
                            .lock()
                            .await
                            .send(ConfigMsg::CoreStatus(CoreStatus::Stopped))
                            .await?;
                    }
                }
                _ => {
                    CORE_MSG_TX.send(CoreMessage::Stopped)?;
                    MSG_TX
                        .lock()
                        .await
                        .send(ConfigMsg::CoreStatus(CoreStatus::Stopped))
                        .await?;
                    error!("Core unknown error {event:?}");
                }
            }
        }
        AOk(())
    });

    Ok(child)
}

impl VCore {
    pub fn build() -> Self {
        Self {
            child: None,
            asset_path: PathBuf::new(),
        }
    }

    /// Init core add assets path and start core
    pub async fn init(&mut self, asset_path: &PathBuf) -> Result<()> {
        self.asset_path = PathBuf::from(asset_path);
        self.child = Some(start_core(&self.asset_path)?);
        Ok(())
    }

    /// Restart core and reload config
    pub async fn restart(&mut self) -> Result<()> {
        if let Some(child) = self.child.take() {
            CORE_SHUTDOWN.store(true, Ordering::Relaxed);
            child.kill()?;
        } else {
            warn!("core process not exist");
            return Ok(());
        };
        let child = start_core(&self.asset_path)?;
        self.child = Some(child);
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        CORE_SHUTDOWN.store(true, Ordering::Relaxed);
        if let Some(child) = self.child.take() {
            info!("Exiting core");
            child.kill()?;
        };
        Ok(())
    }
}

/// Exit Core and store the status
pub async fn exit_core() -> Result<()> {
    let mut core = CORE.lock().await;
    core.exit().with_context(|| "Kill core failed")?;
    Ok(())
}
