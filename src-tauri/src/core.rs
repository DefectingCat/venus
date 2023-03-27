use anyhow::Result;
use tauri::{api::process::{Command, CommandEvent} };

#[derive(Debug)]
pub struct VCore {}

impl VCore {
    pub fn build() -> Result<Self> {
        // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
        let (mut rx, mut child) = Command::new_sidecar("v2ray")
            .expect("failed to create `v2ray` binary command").args(["run" ])
            .spawn()
            .expect("Failed to spawn sidecar");

        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                dbg!(&event);
                if let CommandEvent::Stdout(line) = event {
                    child.write("get v2ray message".as_bytes()).unwrap();
                }
                
            }
        });

        Ok(Self {})
    }
}
