use tauri::State;

use crate::{config::ConfigState, message::MsgSender, utils::error::VResult};

#[tauri::command]
pub async fn select_node(config: State<'_, ConfigState>, tx: State<'_, MsgSender>) -> VResult<()> {
    dbg!(&tx);
    Ok(())
}