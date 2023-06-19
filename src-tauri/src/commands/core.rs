use tauri::State;

use crate::{
    config::{ConfigState, CoreUser, Outbound, OutboundSettings, Vmess},
    message::MsgSender,
    utils::error::{VError, VResult},
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

    let node = node.ok_or(VError::EmptyError("node is empty"))?;

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
        settings: OutboundSettings {
            vnext: Some(vec![vmess]),
        },
        proxy_setting: None,
        mux: None,
    };
    let mut outbounds = vec![proxy];

    let core = config
        .core
        .as_mut()
        .ok_or(VError::EmptyError("core config is empty"))?;
    outbounds.append(&mut core.outbounds);
    core.outbounds = outbounds;
    config.write_core()?;
    tx.send(crate::message::ConfigMsg::RestartCore).await?;

    Ok(())
}
