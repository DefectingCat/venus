use log::info;
use tauri::State;
use tokio::time::Instant;

use crate::{config::ConfigState, message::MsgSender, utils::error::VResult};

pub mod config;
pub mod core;
pub mod subs;

// async fn calculate_speed() -> VResult<usize> {}
async fn speed_test(proxy: String, config: ConfigState) -> VResult<()> {
    let start = Instant::now();
    let proxy = reqwest::Proxy::http(proxy)?;
    let client = reqwest::Client::builder().proxy(proxy).build()?;
    let mut response = client
        .get("https://speed.hetzner.de/100MB.bin")
        .send()
        .await?;
    let latency = start.elapsed().as_millis();
    info!("Latency {}", latency);

    let download_start = Instant::now();
    let mut len = 0;
    while let Some(c) = response.chunk().await? {
        let time = download_start.elapsed().as_nanos() as f64 / 1_000_000_000 as f64;
        len += c.len();
        let bytes_per_second = (len as f64 / time).round();
    }
    Ok(())
}

#[tauri::command]
pub async fn node_speed(
    config: State<'_, ConfigState>,
    nodes: Vec<String>,
    tx: State<'_, MsgSender>,
) -> VResult<()> {
    let config = config.lock().await;
    let local_nodes = config
        .rua
        .subscriptions
        .iter()
        .fold(vec![], |prev, sub| [&prev[..], &sub.nodes[..]].concat());

    nodes.iter().for_each(|id| {
        let target = local_nodes
            .iter()
            .find(|n| n.node_id.as_ref().unwrap_or(&"".to_owned()) == id);
    });

    Ok(())
}
