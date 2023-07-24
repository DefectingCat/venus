use std::vec;

use tauri::State;

use crate::{
    config::{stream_settings_builder, ConfigState, CoreUser, Outbound, OutboundSettings, Vmess},
    message::MsgSender,
    utils::error::{VError, VResult},
};

pub async fn set_node(node_id: String, config: &ConfigState, tx: &MsgSender) -> VResult<()> {
    let mut config = config.lock().await;

    let nodes = config
        .rua
        .subscriptions
        .iter()
        .fold(vec![], |prev, sub| [&prev[..], &sub.nodes[..]].concat());
    let node = nodes
        .iter()
        .find(|node| node.node_id.as_ref().unwrap_or(&"".to_string()) == &node_id)
        .ok_or(VError::EmptyError("Cannot find target node"))?;

    let vmess = Vmess {
        address: node.add.clone(),
        port: node.port.parse()?,
        users: vec![CoreUser {
            id: node.id.clone(),
            alter_id: node.aid.parse()?,
            email: "rua@rua.rua".to_string(),
            security: "auto".to_string(),
        }],
    };
    let proxy = Outbound {
        tag: "proxy".to_string(),
        protocol: "vmess".to_string(),
        settings: OutboundSettings { vnext: vec![vmess] },
        stream_settings: Some(stream_settings_builder(node)?),
        proxy_setting: None,
        mux: None,
    };

    let freedom = Outbound {
        protocol: "freedom".to_owned(),
        settings: OutboundSettings { vnext: vec![] },
        tag: "direct".to_owned(),
        proxy_setting: None,
        stream_settings: None,
        mux: None,
    };
    let blackhole = Outbound {
        protocol: "blackhole".to_owned(),
        settings: OutboundSettings { vnext: vec![] },
        tag: "blocked".to_owned(),
        proxy_setting: None,
        stream_settings: None,
        mux: None,
    };

    let outbounds = vec![proxy, freedom, blackhole];

    let core = config
        .core
        .as_mut()
        .ok_or(VError::EmptyError("core config is empty"))?;
    core.outbounds = outbounds;
    config.write_core()?;
    tx.send(crate::message::ConfigMsg::RestartCore).await?;
    Ok(())
}

/// Active select node from frontend
#[tauri::command]
pub async fn select_node(
    node_id: String,
    config: State<'_, ConfigState>,
    tx: State<'_, MsgSender>,
) -> VResult<()> {
    let config = config.inner();
    let tx = tx.inner();
    set_node(node_id, config, tx).await
}
