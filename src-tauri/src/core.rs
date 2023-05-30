use std::env;

use log::{error, info, warn};
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    async_runtime,
};

use crate::utils::error::VResult;

#[derive(Debug)]
pub struct VCore {
    // Slidecare process
    pub child: Option<CommandChild>,
}

fn start_core() -> VResult<CommandChild> {
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
                    info!("{line}")
                }
                CommandEvent::Stderr(line) => {
                    warn!("{line}")
                }
                _ => {
                    error!("Core unknown error {event:?}")
                }
            }
        }
    });

    Ok(child)
}

impl VCore {
    pub fn build() -> VResult<Self> {
        // Set v2ray assert location with environment
        env::set_var("V2RAY_LOCATION_ASSET", "resources");
        let child = start_core()?;

        Ok(Self { child: Some(child) })
    }

    /// Restart core and reload config
    pub fn restart(&mut self) -> VResult<()> {
        if let Some(child) = self.child.take() {
            if let Err(err) = child.kill() {
                error!("{err}")
            }
        } else {
            warn!("core process not exist");
            return Ok(());
        };
        let child = start_core()?;
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
