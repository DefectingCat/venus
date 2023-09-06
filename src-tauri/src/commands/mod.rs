use anyhow::{Ok as AOk, Result};
use std::{sync::Arc, thread, time::Duration};

use log::{info, warn};
use tauri::{async_runtime, State, Window};
use tokio::{sync::Mutex, time::Instant};

use crate::{
    config::ConfigState,
    core::AVCore,
    event::RUAEvents,
    message::{ConfigMsg, MsgSender},
    utils::error::VResult,
};

pub mod config;
pub mod core;
pub mod subs;

pub async fn speed_test(
    proxy: &str,
    config: ConfigState,
    node_id: String,
    tx: MsgSender,
) -> Result<()> {
    let start = Instant::now();
    let proxy = reqwest::Proxy::http(proxy)?;
    let client = reqwest::Client::builder().proxy(proxy).build()?;
    let c_config = config.lock().await;
    let mut response = client.get(&c_config.rua.settings.speed_url).send().await?;
    let latency = start.elapsed().as_millis();
    info!("Latency {}", latency);
    drop(c_config);

    // download length per chunk
    let len = Arc::new(Mutex::new(0_usize));
    // current download speed per second
    let bytes_per_second = Arc::new(Mutex::new(0.0));
    // is download complete
    let done = Arc::new(Mutex::new(false));

    let total: Option<u64> = response.content_length();

    let check_len = len.clone();
    let bytes = bytes_per_second.clone();
    let check_done = done.clone();
    let write_config = config.clone();
    let a_tx = tx.clone();
    async_runtime::spawn(async move {
        let config = config.clone();
        loop {
            let mut config = config.lock().await;
            let mut node = None;
            config.rua.subscriptions.iter_mut().for_each(|sub| {
                node = sub
                    .nodes
                    .iter_mut()
                    .find(|n| n.node_id.as_ref().unwrap_or(&"".to_owned()) == &node_id);
            });
            let node = match node {
                Some(n) => n,
                None => break,
            };
            node.delay = Some(latency as u64);
            // update config to frontend per 500ms
            thread::sleep(Duration::from_millis(500));
            let check_len = check_len.lock().await;
            let bytes = bytes.lock().await;
            let speed = *bytes / 1_000_000_f64;
            let speed = format!("{:.2}", speed).parse().unwrap_or(speed);
            node.speed = Some(speed);
            let percentage = if let Some(t) = total {
                let p = (*check_len as f64) / (t as f64) * 100.0;
                p.round() as u8
            } else {
                warn!("Content-length is empty");
                0
            };
            info!(
                "Node {} download speed {} MB/s, {}%",
                node.host, speed, percentage
            );
            drop(config);

            a_tx.send(ConfigMsg::EmitConfig).await?;
            let check_done = check_done.lock().await;
            if *check_done {
                break;
            }
        }
        AOk(())
    });

    let download_start = Instant::now();
    tx.send(ConfigMsg::EmitConfig).await?;
    while let Ok(Some(c)) = response.chunk().await {
        // milliseconds
        let time = download_start.elapsed().as_nanos() as f64 / 1_000_000_000_f64;
        let mut len = len.lock().await;
        let mut bytes_per_second = bytes_per_second.lock().await;
        *len += c.len();
        *bytes_per_second = *len as f64 / time;
    }
    let mut done = done.lock().await;
    *done = true;
    let mut config = write_config.lock().await;
    config.write_rua()?;
    tx.send(ConfigMsg::EmitConfig).await?;
    Ok(())
}

#[tauri::command]
pub async fn node_speed(
    nodes: Vec<String>,
    config: State<'_, ConfigState>,
    core: State<'_, AVCore>,
    window: Window,
) -> VResult<()> {
    let ev = RUAEvents::SpeedTest;
    window.emit(ev.as_str(), true)?;

    let core = core.inner().clone();
    let config = config.inner().clone();
    async_runtime::spawn(async move {
        let mut core = core.lock().await;
        core.speed_test(nodes, config.clone())
            .await
            .expect("Speed test failed");
        window.emit(ev.as_str(), false).unwrap();
    });
    Ok(())
}
