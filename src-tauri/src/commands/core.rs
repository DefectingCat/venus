use tauri::State;

use crate::{
    config::{ConfigState, CoreUser, Outbound, OutboundSettings, Vmess},
    message::MsgSender,
    utils::error::VResult,
};

#[tauri::command]
pub async fn select_node(
    sub_name: String,
    node_id: String,
    config: State<'_, ConfigState>,
    tx: State<'_, MsgSender>,
) -> VResult<()> {
    let mut config = config.lock().await;

    let node = config.rua.subscriptions.as_ref().and_then(|subs| {
        let sub = subs.iter().find(|sub| sub.name == sub_name);
        sub.and_then(|s| {
            s.nodes.as_ref().and_then(|nodes| {
                nodes
                    .iter()
                    .find(|node| node.node_id.as_ref().unwrap_or(&"".to_string()) == &node_id)
            })
        })
    });

    if let Some(node) = node.as_ref() {
        let vmess = Vmess {
            address: node.add.clone(),
            port: node.port.clone(),
            users: vec![CoreUser {
                id: node.id.clone(),
                alter_id: node.aid.clone(),
                email: "rua@rua.rua".to_string(),
                security: "auto".to_string(),
            }],
        };
        let proxy = Outbound {
            tag: "proxy".to_string(),
            protocol: "vmess".to_string(),
            settings: OutboundSettings {
                vnext: Some(vec![vmess]),
            },
            proxy_setting: None,
            mux: None,
        };
        let mut outbounds = vec![proxy];

        if let Some(mut core) = config.core.as_mut() {
            outbounds.append(&mut core.outbounds);
            core.outbounds = outbounds;
            config.write_core()?;
            tx.send(crate::message::ConfigMsg::RestartCore).await?;
        };
    }

    Ok(())
}