use std::{sync::Arc, thread, time::Duration};

use log::{info, warn};
use tauri::{async_runtime, State};
use tokio::{
    sync::{Mutex, MutexGuard},
    time::Instant,
};

use crate::{
    config::{ConfigState, Node, VConfig},
    message::MsgSender,
    utils::error::{VError, VResult},
};

pub mod config;
pub mod core;
pub mod subs;

// async fn calculate_speed() -> VResult<usize> {}
// async fn speed_test(proxy: &str, config: &mut MutexGuard<'_, VConfig>) -> VResult<()> {
pub async fn speed_test(proxy: &str, node: &mut Node) -> VResult<()> {
    let start = Instant::now();
    let proxy = reqwest::Proxy::http(proxy)?;
    let client = reqwest::Client::builder().proxy(proxy).build()?;
    let mut response = client
        // .get("https://speed.hetzner.de/100MB.bin")
        .get("https://sabnzbd.org/tests/internetspeed/20MB.bin")
        .send()
        .await?;
    let latency = start.elapsed().as_millis();
    info!("Latency {}", latency);
    node.delay = Some(latency);

    // download length per chunk
    let len = Arc::new(Mutex::new(0 as usize));
    // current download speed per second
    let bytes_per_second = Arc::new(Mutex::new(0.0));
    // is download complete
    let done = Arc::new(Mutex::new(false));

    let total: Option<u64> = response.content_length();

    let check_len = len.clone();
    let bytes = bytes_per_second.clone();
    let check_done = done.clone();
    let node_host = node.host.clone();
    async_runtime::spawn(async move {
        loop {
            // update config to frontend per 500ms
            thread::sleep(Duration::from_millis(500));
            let check_len = check_len.lock().await;
            let bytes = bytes.lock().await;
            let check_done = check_done.lock().await;
            let percentage = if let Some(t) = total {
                (*check_len as f64) / (t as f64)
            } else {
                warn!("Content-length is empty");
                0.0
            };
            dbg!(&bytes, &check_len, &total, &percentage);
            info!(
                "Node {} download speed {}, {}",
                node_host,
                *bytes / 100_000 as f64,
                percentage
            );
            if *check_done {
                break;
            }
        }
    });

    let download_start = Instant::now();
    while let Some(c) = response.chunk().await? {
        let time = download_start.elapsed().as_nanos() as f64 / 1_000_000_000 as f64;
        let mut len = len.lock().await;
        let mut bytes_per_second = bytes_per_second.lock().await;
        *len += c.len();
        *bytes_per_second = (*len as f64 / time).round();
    }
    let mut done = done.lock().await;
    *done = true;

    Ok(())
}

#[tauri::command]
pub async fn node_speed(nodes: Vec<String>, tx: State<'_, MsgSender>) -> VResult<()> {
    tx.send(crate::message::ConfigMsg::NodeSpeedtest(nodes))
        .await?;
    Ok(())
}
