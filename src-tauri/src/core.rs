use std::{
    path::{Path, PathBuf},
    sync::{atomic::Ordering, Arc},
};

use log::{error, info, warn};
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    async_runtime,
};
use tokio::sync::Mutex;

use crate::{
    commands::speed_test,
    config::{outbouds_builder, ConfigState, CoreStatus},
    message::{ConfigMsg, MsgSender},
    utils::error::{VError, VResult},
    CORE_SHUTDOWN,
};

pub type AVCore = Arc<Mutex<VCore>>;

#[derive(Debug)]
pub struct VCore {
    // Slidecare process
    pub child: Option<CommandChild>,
    // Message sender
    pub tx: MsgSender,
    // Core resource path
    asset_path: PathBuf,
}

fn start_core(tx: MsgSender, path: &Path) -> VResult<CommandChild> {
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
    pub fn build(tx: MsgSender) -> Self {
        Self {
            child: None,
            tx,
            asset_path: PathBuf::new(),
        }
    }

    /// Init core add assets path and start core
    pub async fn init(&mut self, asset_path: &PathBuf) -> VResult<()> {
        self.asset_path = PathBuf::from(asset_path);
        self.child = Some(start_core(self.tx.clone(), &self.asset_path)?);
        Ok(())
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

    pub async fn speed_test(&mut self, node_ids: Vec<String>, config: ConfigState) -> VResult<()> {
        let loop_config = config.clone();

        let mut proxy = String::new();
        {
            let config = config.lock().await;
            let current_node = config
                .core
                .as_ref()
                .and_then(|core| core.outbounds.first().clone());
            let target_proxy = config
                .core
                .as_ref()
                .and_then(|core| core.inbounds.iter().find(|inbound| inbound.tag == "http"))
                .ok_or(VError::EmptyError("cannot find http inbound"))?;
            proxy = format!("http://{}:{}", target_proxy.listen, target_proxy.port);
        }

        for id in node_ids {
            let write_config = loop_config.clone();

            {
                let mut config = loop_config.lock().await;
                let config = &mut *config;
                let rua = &mut config.rua;

                let mut target = None;
                rua.subscriptions.iter_mut().for_each(|sub| {
                    target = sub
                        .nodes
                        .iter_mut()
                        .find(|n| n.node_id.as_ref().unwrap_or(&"".to_owned()) == &id);
                });
                let target = target.unwrap();

                let outbounds = outbouds_builder(target)?;
                let core = config
                    .core
                    .as_mut()
                    .ok_or(VError::EmptyError("core config is empty"))?;
                core.outbounds = outbounds;
                config.write_core()?;
            }

            self.restart().await?;
            speed_test(&proxy, write_config.clone(), &id, self.tx.clone()).await?;
        }

        Ok(())
    }
}
