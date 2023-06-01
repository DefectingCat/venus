use base64::{engine::general_purpose, Engine};
use log::debug;
use tauri::State;

use crate::{
    config::{ConfigState, Node, Subscription},
    utils::error::VResult,
};

async fn request_subs(name: &str, url: &str) -> VResult<Vec<Node>> {
    let result = reqwest::get(url).await?.text().await?;

    // Decode result to vmess://...
    let subscripition = general_purpose::STANDARD.decode(result)?;
    let subscripition = String::from_utf8_lossy(&subscripition).to_string();
    // Serizlize outbound nodes to json
    let subscripition = subscripition
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.replace("vmess://", "");
            let line = general_purpose::STANDARD.decode(line)?;
            let line = String::from_utf8_lossy(&line).to_string();
            let mut line = serde_json::from_str::<Node>(&line)?;
            line.subs = Some(name.to_string());
            let id = md5::compute(format!("{}-{}-{}", line.ps, line.add, line.port));
            line.id = format!("{:?}", id);
            Ok(line)
        })
        .collect::<VResult<Vec<_>>>()?;
    debug!("{subscripition:?}");
    Ok(subscripition)
}

#[tauri::command]
pub async fn add_subscription(
    name: String,
    url: String,
    config: State<'_, ConfigState>,
) -> VResult<()> {
    let mut config = config.lock().await;
    let mut subscription = request_subs(&name, &url).await?;

    // Write subscription and nodes to config file
    if let Some(nodes) = config.rua.nodes.as_mut() {
        nodes.append(&mut subscription);
    } else {
        config.rua.nodes = Some(subscription)
    };
    let sub = Subscription { name, url };
    if let Some(subscriptions) = config.rua.subscriptions.as_mut() {
        subscriptions.push(sub);
    } else {
        config.rua.subscriptions = Some(vec![sub])
    }
    config.write_rua()?;
    Ok(())
}