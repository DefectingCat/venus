use std::env;

use anyhow::Result;
use log::info;
use tauri::api::process::{Command, CommandEvent};

#[derive(Debug)]
pub struct VCore {}

impl VCore {
    pub fn build() -> Result<Self> {
        // Set v2ray assert location with environment
        env::set_var("V2RAY_LOCATION_ASSET", "resources");
        // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
        let (mut rx, mut child) = Command::new_sidecar("v2ray")
            .expect("failed to create `v2ray` binary command").args(["run", "-c", "resources/config.json" ])
            .spawn()
            .expect("Failed to spawn sidecar");

        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                // dbg!(&event);
                if let CommandEvent::Stdout(line) = event {
                    info!("{line}");
                    child.write("get v2ray message".as_bytes()).unwrap();
                }
                
            }
        });

        Ok(Self {})
    }
}
