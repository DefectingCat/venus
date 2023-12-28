use crate::{
    config::{find_node, proxy_builder},
    message::{ConfigMsg, MSG_TX},
    utils::error::VResult,
    CONFIG,
};
use anyhow::anyhow;

/// Active select node from frontend
#[tauri::command]
pub async fn select_node(node_id: String) -> VResult<()> {
    let mut config = CONFIG.lock().await;
    let config = &mut *config;
    let rua = &mut config.rua;
    let core = &mut config.core;

    let node = find_node(&node_id, rua)?;
    let core = core
        .as_mut()
        .ok_or(anyhow!("cannont found config config"))?;
    let proxy_outbound = core
        .outbounds
        .iter()
        .position(|outbound| outbound.tag == "proxy");
    if let Some(index) = proxy_outbound {
        core.outbounds[index] = proxy_builder(node, "proxy".into())?;
    } else {
        core.outbounds
            .insert(0, proxy_builder(node, "proxy".into())?)
    }
    config.write_core()?;
    config.rua.current_id = node_id;
    config.write_rua()?;
    MSG_TX.lock().await.send(ConfigMsg::RestartCore).await?;
    Ok(())
}

#[tauri::command]
pub async fn restart_core() -> VResult<()> {
    MSG_TX.lock().await.send(ConfigMsg::RestartCore).await?;
    Ok(())
}
