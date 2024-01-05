use crate::{
    config::{change_connectivity, find_node, proxy_builder, Rule},
    event::{RUAEvents, SpeedTestPayload},
    message::{ConfigMsg, MSG_TX},
    utils::{consts::SPEED_URL, error::VResult},
    CONFIG,
};
use anyhow::{anyhow, Ok as AOk, Result};
use log::{error, info, warn};
use std::{sync::Arc, thread, time::Duration};
use tauri::{async_runtime, Window};
use tokio::{
    sync::Mutex,
    time::{sleep, Instant},
};
use url::Url;

pub mod config;
pub mod core;
pub mod subs;
pub mod ui;

pub async fn speed_test(proxy: &str, node_id: String) -> Result<()> {
    let start = Instant::now();
    let http = reqwest::Proxy::http(proxy)?;
    let https = reqwest::Proxy::https(proxy)?;
    let client = reqwest::Client::builder()
        .proxy(http)
        .proxy(https)
        .build()?;
    let c_config = CONFIG.lock().await;
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

    async_runtime::spawn(async move {
        loop {
            let mut config = CONFIG.lock().await;
            let rua = &mut config.rua;
            let mut node = None;
            rua.subscriptions.iter_mut().for_each(|sub| {
                node = sub
                    .nodes
                    .iter_mut()
                    .find(|n| n.node_id.as_ref().unwrap_or(&"".to_string()) == &node_id);
            });
            let node = node.ok_or(anyhow!("node {} not found", node_id))?;
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
                node.add, speed, percentage
            );

            drop(config);
            MSG_TX.lock().await.send(ConfigMsg::EmitConfig).await?;
            let check_done = check_done.lock().await;
            if *check_done {
                break;
            }
        }
        AOk(())
    });

    let download_start = Instant::now();
    MSG_TX.lock().await.send(ConfigMsg::EmitConfig).await?;
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
    let mut config = CONFIG.lock().await;
    config.write_rua()?;
    drop(config);
    MSG_TX.lock().await.send(ConfigMsg::EmitConfig).await?;
    Ok(())
}

/// Test selected node speed
///
/// ## Argments
///
/// `node_id`: selected node id
/// `window`: tauri window
#[tauri::command]
pub async fn node_speed(node_id: String, window: Window) -> VResult<()> {
    let mut orgin_config = CONFIG.lock().await;
    let config = &mut *orgin_config;
    let rua = &config.rua;
    let core = &mut config.core;
    let core = core
        .as_mut()
        .ok_or(anyhow!("cannont found config config"))?;

    // Change speed outbound
    let node = find_node(&node_id, rua)?;
    let speed_outbound = core
        .outbounds
        .iter()
        .position(|outbound| outbound.tag == "speed");
    if let Some(index) = speed_outbound {
        core.outbounds[index] = proxy_builder(node, "speed".into())?;
    } else {
        core.outbounds.push(proxy_builder(node, "speed".into())?);
    }
    // add speed outbound routing rule
    let rule_index = core
        .routing
        .rules
        .iter()
        .position(|rule| rule.outbound_tag == "speed")
        .unwrap_or_else(|| {
            let mut speed_rule = Rule::new("speed".into());
            speed_rule.domain = Some(vec![rua.settings.speed_url.clone()]);
            core.routing.rules.push(speed_rule);
            core.routing.rules.len() - 1
        });
    let speed_rule = &mut core.routing.rules[rule_index];
    let domain_rule = Url::parse(SPEED_URL).map_err(|err| anyhow!("{}", err))?;
    let domain_rule = domain_rule.host().ok_or(anyhow!(""))?;
    match &mut speed_rule.domain {
        None => {
            speed_rule.domain = Some(vec![domain_rule.to_string()]);
        }
        Some(domain) => {
            let domain_string = domain_rule.to_string();
            if domain[0] != domain_string {
                domain[0] = domain_string;
            }
        }
    }

    config.write_core()?;
    drop(orgin_config);
    MSG_TX.lock().await.send(ConfigMsg::RestartCore).await?;

    // prepare to test speed
    let mut config = CONFIG.lock().await;
    let core = &config
        .core
        .as_mut()
        .ok_or(anyhow!("cannont found config config"))?;
    let target_proxy = core
        .inbounds
        .iter()
        .find(|inbound| inbound.tag == "socks")
        .ok_or(anyhow!("cannot find socks inbound"))?;
    let proxy = format!("socks5://{}:{}", target_proxy.listen, target_proxy.port);
    drop(config);

    // TODO core restart notification
    sleep(Duration::from_secs(1)).await;
    // test speed and change loading state
    let ev = RUAEvents::SpeedTest;
    let mut payload = SpeedTestPayload {
        id: &node_id,
        loading: true,
    };
    window.emit(ev.as_str(), &payload)?;
    match speed_test(&proxy, node_id.clone()).await {
        Ok(_) => {
            change_connectivity(&node_id, true).await?;
        }
        Err(err) => {
            error!("Speed test failed {}", err);
            change_connectivity(&node_id, false).await?;
        }
    }
    payload.loading = false;
    window.emit(ev.as_str(), &payload)?;
    Ok(())
}
