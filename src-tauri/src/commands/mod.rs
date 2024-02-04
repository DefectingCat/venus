use crate::{
    config::{change_connectivity, find_node, proxy_builder, Rule},
    core::{CoreMessage, CORE_MSG_TX},
    event::{RUAEvents, SpeedTestPayload},
    message::{ConfigMsg, MSG_TX},
    utils::{
        consts::SPEED_URL,
        error::{VError, VResult},
    },
    CONFIG,
};
use anyhow::{anyhow, Result};
use log::{error, info, warn};
use tauri::Window;
use tokio::time::{sleep, Duration, Instant};
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
    let mut config = CONFIG.lock().await;
    let mut response = client.get(&config.rua.settings.speed_url).send().await?;
    let latency = start.elapsed().as_millis();
    info!("Latency {}ms", latency);

    // download length per chunk
    let mut len = 0_usize;
    let total = response.content_length().unwrap_or_else(|| {
        warn!("Content-length is empty");
        0
    });

    let download_start = Instant::now();
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
    while let Ok(Some(c)) = response.chunk().await {
        // milliseconds
        let time = download_start.elapsed().as_nanos() as f64 / 1_000_000_000_f64;
        len += c.len();
        let bytes_per_second = len as f64 / time;

        let speed = bytes_per_second / 1_000_000_f64;
        let speed = format!("{:.2}", speed).parse().unwrap_or(speed);
        node.speed = Some(speed);
        let percentage = {
            let p = (len as f64) / (total as f64) * 100.0;
            p.round() as u8
        };
        info!(
            "Node {} download speed {} MB/s, {}%",
            node.add, speed, percentage
        );
        // MSG_TX.lock().await.send(ConfigMsg::EmitConfig).await?;
    }
    config.write_rua()?;
    drop(config);
    MSG_TX.lock().await.send(ConfigMsg::EmitConfig).await?;
    Ok(())
}

/// Test selected node speed
///
/// ## Arguments
///
/// `node_id`: selected node id
/// `window`: tauri window
#[tauri::command]
pub async fn node_speed(node_id: String, window: Window) -> VResult<()> {
    let mut origin_config = CONFIG.lock().await;
    let config = &mut *origin_config;
    let rua = &config.rua;
    let core = &mut config.core;
    let core = core.as_mut().ok_or(anyhow!("cannot found config config"))?;

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
    drop(origin_config);

    // prepare to test speed
    let mut config = CONFIG.lock().await;
    let core = &config
        .core
        .as_mut()
        .ok_or(anyhow!("cannot found config config"))?;
    let target_proxy = core
        .inbounds
        .iter()
        .find(|inbound| inbound.tag == "socks")
        .ok_or(anyhow!("cannot find socks inbound"))?;
    let proxy = format!("socks5://{}:{}", target_proxy.listen, target_proxy.port);
    drop(config);

    let mut rx = CORE_MSG_TX.subscribe();
    MSG_TX.lock().await.send(ConfigMsg::RestartCore).await?;
    // test speed and change loading state
    // TODO tokio select
    let ev = RUAEvents::SpeedTest;
    let mut payload = SpeedTestPayload {
        id: &node_id,
        loading: true,
    };
    let test_node_speed = async {
        while let Ok(msg) = rx.recv().await {
            if let CoreMessage::Started = msg {
                window.emit(ev.as_str(), &payload)?;
                return match speed_test(&proxy, node_id.clone()).await {
                    Ok(_) => {
                        change_connectivity(&node_id, true).await?;
                        payload.loading = false;
                        window.emit(ev.as_str(), &payload)?;
                        Ok(())
                    }
                    Err(err) => {
                        let err = format!("Speed test failed {}", err);
                        error!("{err}");
                        change_connectivity(&node_id, false).await?;
                        Err(VError::CommonError(anyhow!(err)))
                    }
                };
            } else {
                continue;
            }
        }
        Ok(())
    };
    tokio::select! {
        val = test_node_speed => {
            val?;
        }
        _ = sleep(Duration::from_millis(20000)) => {
            change_connectivity(&node_id, false).await?;
            payload.loading = false;
            window.emit(ev.as_str(), &payload)?;
            return Err(VError::CommonError(anyhow!("Speed test timeout")))
        }
    }
    Ok(())
}
