use anyhow::anyhow;
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

    let mut node = None;
    config.rua.subscriptions.iter().for_each(|sub| {
        node = sub
            .nodes
            .iter()
            .find(|n| n.node_id.as_ref().unwrap_or(&"".to_string()) == &node_id);
    });
    let node = node.ok_or(anyhow!("node {} not found", node_id))?;

    let outbounds = outbouds_builder(node)?;

    let core = config
        .core
        .as_mut()
        .ok_or(VError::EmptyError("core config is empty"))?;
    core.outbounds = outbounds;
    config.write_core()?;

    config.rua.current_id = node_id;
    config.write_rua()?;
    tx.send(crate::message::ConfigMsg::RestartCore).await?;
    Ok(())
}
