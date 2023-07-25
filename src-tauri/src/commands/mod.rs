use log::info;
use tauri::State;
use tokio::{sync::MutexGuard, time::Instant};

use crate::{
    config::{ConfigState, VConfig},
    message::MsgSender,
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
        dbg!(bytes_per_second);
    }
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
        speed_test(&proxy.as_str(), &mut config).await?;
    }

    Ok(())
}
