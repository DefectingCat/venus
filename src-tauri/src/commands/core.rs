use crate::{
    config::proxy_builder,
    message::{ConfigMsg, MSG_TX},
    utils::error::VResult,
    CONFIG, CORE,
};
use anyhow::anyhow;

/// Active select node from frontend
#[tauri::command]
pub async fn select_node(node_id: String) -> VResult<()> {
    let mut config = CONFIG.lock().await;
    let config = &mut *config;
    let rua = &mut config.rua;
    let core = &mut config.core;

    let mut node = None;
    rua.subscriptions.iter().for_each(|sub| {
        node = sub
            .nodes
            .iter()
            .find(|n| n.node_id.as_ref().unwrap_or(&"".to_string()) == &node_id);
    });
    let node = node.ok_or(anyhow!("node {} not found", node_id))?;

    let core = core
        .as_mut()
        .ok_or(anyhow!("cannont found config config"))?;
    let proxy_outbound = core
        .outbounds
        .iter()
        .position(|outbound| outbound.tag == "proxy")
        .ok_or(anyhow!("cannot found proxy outbound"))?;
    core.outbounds[proxy_outbound] = proxy_builder(node, "proxy".into())?;
    config.write_core()?;

    config.rua.current_id = node_id;
    config.write_rua()?;
    MSG_TX.lock().await.send(ConfigMsg::RestartCore).await?;
    Ok(())
}

#[tauri::command]
pub async fn restart_core() -> VResult<()> {
    CORE.lock().await.restart().await?;
    Ok(())
}
