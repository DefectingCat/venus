use std::{sync::Arc, thread, time::Duration};

use log::{info, warn};
use tauri::{async_runtime, State};
use tokio::{sync::Mutex, time::Instant};

use crate::{
    config::ConfigState,
    core::AVCore,
    message::{ConfigMsg, MsgSender},
    utils::error::{VError, VResult},
};

pub mod config;
pub mod core;
pub mod subs;

// async fn calculate_speed() -> VResult<usize> {}
// async fn speed_test(proxy: &str, config: &mut MutexGuard<'_, VConfig>) -> VResult<()> {
pub async fn speed_test(
    proxy: &str,
    config: ConfigState,
    node_id: String,
    tx: MsgSender,
) -> VResult<()> {
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
            let speed = *bytes / 100_000_f64;
            node.speed = Some(speed);
            let percentage = if let Some(t) = total {
                (*check_len as f64) / (t as f64)
            } else {
                warn!("Content-length is empty");
                0.0
            };
            dbg!(&bytes, &check_len, &total, &percentage);
            info!(
                "Node {} download speed {}, {}",
                node.host,
                *bytes / 100_000_f64,
                percentage
            );
            drop(config);

            a_tx.send(ConfigMsg::EmitConfig)
                .await
                .map_err(|err| {
                    VError::CommonError(format!("Send emit-config message failed {}", err))
                })
                .unwrap();
            let check_done = check_done.lock().await;
            if *check_done {
                break;
            }
        }
    });

    let download_start = Instant::now();
    tx.send(ConfigMsg::EmitConfig).await?;
    while let Some(c) = response.chunk().await? {
        // milliseconds
        let time = download_start.elapsed().as_nanos() as f64 / 1_000_000_000_f64;
        let mut len = len.lock().await;
        let mut bytes_per_second = bytes_per_second.lock().await;
        *len += c.len();
        *bytes_per_second = (*len as f64 / time).round();
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
) -> VResult<()> {
    let core = core.inner().clone();
    let config = config.inner().clone();
    async_runtime::spawn(async move {
        let mut core = core.lock().await;
        core.speed_test(nodes, config.clone())
            .await
            .expect("Speed test failed");
    });
    Ok(())
}
