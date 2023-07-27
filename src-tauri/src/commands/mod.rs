use std::{sync::Arc, thread, time::Duration};

use log::{info, warn};
use tauri::{async_runtime, State};
use tokio::{
    sync::{Mutex, MutexGuard},
    time::Instant,
};

use crate::{
    config::{ConfigState, VConfig},
    message::{BroadcastState, MsgSender},
    utils::error::{VError, VResult},
};

use self::core::set_node;

pub mod config;
pub mod core;
pub mod subs;

// async fn calculate_speed() -> VResult<usize> {}
async fn speed_test(proxy: &str, config: &mut MutexGuard<'_, VConfig>) -> VResult<()> {
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
    let len = Arc::new(Mutex::new(0 as usize));
    let bytes_per_second = Arc::new(Mutex::new(0.0));
    let done = Arc::new(Mutex::new(false));

    let total: Option<u64> = response.content_length();

    let check_len = len.clone();
    let bytes = bytes_per_second.clone();
    let check_done = done.clone();
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
        // dbg!(bytes_per_second);
    }
    let mut done = done.lock().await;
    *done = true;

    Ok(())
}

#[tauri::command]
pub async fn node_speed(
    config: State<'_, ConfigState>,
    nodes: Vec<String>,
    tx: State<'_, MsgSender>,
) -> VResult<()> {
    let mut config = config.lock().await;
    let local_nodes = config
        .rua
        .subscriptions
        .iter()
        .fold(vec![], |prev, sub| [&prev[..], &sub.nodes[..]].concat());

    let current_node = config
        .core
        .as_ref()
        .and_then(|core| core.outbounds.first().clone());
    let proxy = config
        .core
        .as_ref()
        .and_then(|core| core.inbounds.iter().find(|inbound| inbound.tag == "http"))
        .ok_or(VError::EmptyError("cannot find http inbound"))?;
    let proxy = format!("http://{}:{}", proxy.listen, proxy.port);

    for id in nodes {
        let target = local_nodes
            .iter()
            .find(|n| n.node_id.as_ref().unwrap_or(&"".to_owned()) == &id)
            .unwrap();
        let node_id = target.node_id.as_ref().unwrap().as_str();
        set_node(node_id, &mut config, tx.clone()).await?;

        // while let Ok(status) = b_rx.lock().await.recv().await {
        //     dbg!(&status);
        // speed_test(&proxy.as_str(), &mut config).await?;
        // }
    }

    Ok(())
}
