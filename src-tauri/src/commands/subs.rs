use base64::{engine::general_purpose, Engine};
use log::{debug, info};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    config::{ConfigState, Node, Subscription},
    message::MsgSender,
    utils::error::{VError, VResult},
    NAME, VERSION,
};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    VMESS,
    VLESS,
    SS,
    SSR,
    TROJAN,
    TROJANGO,
    HTTPPROXY,
    HTTPSPROXY,
    SOCKS5,
    HTTP2,
    UNKNOWN,
}
impl From<&str> for NodeType {
    fn from(value: &str) -> Self {
        use NodeType::*;
        match value {
            "vmess" => VMESS,
            "vless" => VLESS,
            "ss" => SS,
            "ssr" => SSR,
            "trojan" => TROJAN,
            "trojan-go" => TROJANGO,
            "http-proxy" => HTTPPROXY,
            "https-proxy" => HTTPSPROXY,
            "socks5" => SOCKS5,
            "http2" => HTTP2,
            _ => UNKNOWN,
        }
    }
}
impl NodeType {
    pub fn as_str(&self) -> &str {
        use NodeType::*;
        match self {
            VMESS => "vmess",
            VLESS => "vless",
            SS => "ss",
            SSR => "ssr",
            TROJAN => "trojan",
            TROJANGO => "trojan-go",
            HTTPPROXY => "http-proxy",
            HTTPSPROXY => "https-proxy",
            SOCKS5 => "socks5",
            HTTP2 => "http2",
            UNKNOWN => "unknown",
        }
    }
}
impl Serialize for NodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for NodeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(NodeType::from(s.as_str()))
    }
}
// const NODE_PREFIX: [&str; 10] = [
//     "vmess",
//     "vless",
//     "ss",
//     "ssr",
//     "trojan",
//     "trojan-go",
//     "http-proxy",
//     "https-proxy",
//     "socks5",
//     "http2",
// ];

/// Send http request to download subscription info
async fn request_subs(name: &str, url: &str) -> VResult<Vec<Node>> {
    let client = reqwest::Client::new();
    let result = client
        .get(url)
        .header(USER_AGENT, format!("{}/{}", NAME, VERSION))
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
            let (node_type, link) = line
                .split_once("://")
                .ok_or(VError::EmptyError("Cannot serialize node link"))?;
            let link = general_purpose::STANDARD.decode(link)?;
            let link = String::from_utf8_lossy(&link).to_string();
            let mut node = serde_json::from_str::<Node>(&link)?;

            node.subs = Some(name.to_string());
            // Add unique id
            let id = md5::compute(format!("{}-{}-{}", node.ps, node.add, node.port));
            node.node_id = Some(format!("{:?}", id));
            node.raw_link = Some(line.to_owned());
            node.node_type = Some(NodeType::from(node_type));
            Ok(node)
        })
        .collect::<VResult<Vec<_>>>()?;
    debug!("{subscription:?}");
    Ok(subscription)
}

/// Add new subscription and write
/// to config file
#[tauri::command]
pub async fn add_subscription(
    name: String,
    url: String,
    config: State<'_, ConfigState>,
    tx: State<'_, MsgSender>,
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
    tx.send(crate::message::ConfigMsg::RestartCore).await?;
    Ok(())
}

/// Update all subscriptions in config file.
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
        .ok_or(VError::EmptyError("Subscriptions is empty"))?;
    for Subscription { name, url, nodes } in subs {
        let new_nodes = request_subs(name, url).await?;
        *nodes = Some(new_nodes);
    }
    config.write_rua()?;
    tx.send(crate::message::ConfigMsg::RestartCore).await?;
    info!("Update all subscriptions done");
    Ok(())
}

/// Update specific subscription with url
#[tauri::command]
pub async fn update_sub(
    config: State<'_, ConfigState>,
    tx: State<'_, MsgSender>,
    url: &str,
) -> VResult<()> {
    info!("Start update subscription {}", &url);
    let mut config = config.lock().await;
    let sub = config
        .rua
        .subscriptions
        .as_mut()
        .ok_or(VError::EmptyError("Subscriptions is empty"))?
        .iter_mut()
        .find(|s| s.url == url)
        .ok_or(VError::EmptyError("Cannot find target subscription"))?;
    let new_nodes = request_subs(&sub.name, &sub.url).await?;
    sub.nodes = Some(new_nodes);
    config.write_rua()?;
    tx.send(crate::message::ConfigMsg::RestartCore).await?;
    info!("Update subscription {} done", &url);
    Ok(())
}
