use base64::{engine::general_purpose, Engine};
use log::{debug, info};
use reqwest::header::USER_AGENT;
use tauri::State;

use crate::{
    config::{ConfigState, Node, Subscription},
    message::MsgSender,
    utils::error::{VError, VResult},
};

#[tauri::command]
pub async fn get_subscriptions(state: State<'_, ConfigState>) -> VResult<String> {
    let config = state.lock().await;
    let subs = serde_json::to_string(&config.rua.subscriptions)?;
    Ok(subs)
}

/// Send http request to download subscription info
async fn request_subs(name: &str, url: &str) -> VResult<Vec<Node>> {
    let client = reqwest::Client::new();
    let result = client
        .get(url)
        .header(USER_AGENT, format!("V2rayR/{}", env!("CARGO_PKG_VERSION")))
        .send()
        .await?
        .text()
        .await?;

    // Decode result to vmess://...
    let subscription = general_purpose::STANDARD.decode(result)?;
    let subscription = String::from_utf8_lossy(&subscription).to_string();
    // Serizlize outbound nodes to json
    let subscription = subscription
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.replace("vmess://", "");
            let line = general_purpose::STANDARD.decode(line)?;
            let line = String::from_utf8_lossy(&line).to_string();
            let mut line = serde_json::from_str::<Node>(&line)?;

            line.subs = Some(name.to_string());
            // Add unique id
            let id = md5::compute(format!("{}-{}-{}", line.ps, line.add, line.port));
            line.node_id = Some(format!("{:?}", id));
            Ok(line)
        })
        .collect::<VResult<Vec<_>>>()?;
    debug!("{subscription:?}");
    Ok(subscription)
}

#[tauri::command]
pub async fn add_subscription(
    name: String,
    url: String,
    config: State<'_, ConfigState>,
) -> VResult<()> {
    let mut config = config.lock().await;
    let nodes = request_subs(&name, &url).await?;

    // Write subscription and nodes to config file
    let sub = Subscription {
        name,
        url,
        nodes: Some(nodes),
    };
    if let Some(subscriptions) = config.rua.subscriptions.as_mut() {
        subscriptions.push(sub);
    } else {
        config.rua.subscriptions = Some(vec![sub])
    }
    config.write_rua()?;
    Ok(())
}

#[tauri::command]
pub async fn update_all_subs(
    config: State<'_, ConfigState>,
    tx: State<'_, MsgSender>,
) -> VResult<()> {
    info!("Starting update all subscriptions");
    let mut config = config.lock().await;
    let subs = config
        .rua
        .subscriptions
        .as_mut()
        .ok_or(VError::EmptyError("Rua config is empty"))?;
    for Subscription { name, url, nodes } in subs {
        let new_nodes = request_subs(name, url).await?;
        *nodes = Some(new_nodes);
    }
    config.write_rua()?;
    tx.send(crate::message::ConfigMsg::RestartCore).await?;
    info!("Update all subscriptions done");
    Ok(())
}
