use std::vec;

use tauri::State;

use crate::{
    config::{outbouds_builder, ConfigState},
    message::MsgSender,
    utils::error::{VError, VResult},
};

/// Active select node from frontend
#[tauri::command]
pub async fn select_node(
    node_id: String,
    config: State<'_, ConfigState>,
    tx: State<'_, MsgSender>,
) -> VResult<()> {
    let mut config = config.lock().await;
    // set_node(node_id.as_str(), &mut config, tx).await

    let nodes = config
        .rua
        .subscriptions
        .iter()
        .fold(vec![], |prev, sub| [&prev[..], &sub.nodes[..]].concat());
    let node = nodes
        .iter()
        .find(|node| node.node_id.as_ref().unwrap_or(&"".to_string()) == &node_id)
        .ok_or(VError::EmptyError("Cannot find target node"))?;

    let outbounds = outbouds_builder(node)?;

    let core = config
        .core
        .as_mut()
        .ok_or(VError::EmptyError("core config is empty"))?;
    core.outbounds = outbounds;
    config.write_core()?;
    tx.send(crate::message::ConfigMsg::RestartCore).await?;
    Ok(())
}
