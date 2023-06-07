use std::{
    env,
    sync::{Arc, Mutex},
};

use log::{error, info, warn};
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    async_runtime,
};
use tokio::sync::mpsc::Sender;

use crate::{config::CoreStatus, message::ConfigMsg, utils::error::VResult};

pub type AVCore = Arc<Mutex<Option<VCore>>>;

#[derive(Debug)]
pub struct VCore {
    // Slidecare process
    pub child: Option<CommandChild>,
    // Message sender
    pub tx: Arc<Sender<ConfigMsg>>,
}

fn start_core(tx: Arc<Sender<ConfigMsg>>) -> VResult<CommandChild> {
    // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
    let (mut rx, child) = Command::new_sidecar("v2ray")
        .expect("Failed to create `v2ray` binary command")
        .args(["run", "-c", "resources/config.json"])
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
                    warn!("{line:?}");
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
    pub fn build(tx: Arc<Sender<ConfigMsg>>) -> VResult<Self> {
        // Set v2ray assert location with environment
        env::set_var("V2RAY_LOCATION_ASSET", "resources");
        let child = start_core(tx.clone())?;

        Ok(Self {
            child: Some(child),
            tx,
        })
    }

    /// Restart core and reload config
    pub fn restart(&mut self) -> VResult<()> {
        if let Some(child) = self.child.take() {
            child.kill()?;
        } else {
            warn!("core process not exist");
            return Ok(());
        };
        let child = start_core(self.tx.clone())?;
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
