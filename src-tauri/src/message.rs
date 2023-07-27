use std::sync::Arc;

use tokio::sync::{
    broadcast, mpsc,
    mpsc::{Receiver, Sender},
    Mutex,
};

use crate::config::CoreStatus;

#[derive(Debug)]
pub enum ConfigMsg {
    CoreStatus(CoreStatus),
    RestartCore,
    EmitLog(String),
}
// pub struct ConfigMsg {
//     pub msg: ConfigMsgType,
// }

pub type MsgSender = Arc<Sender<ConfigMsg>>;

pub fn msg_build() -> (Sender<ConfigMsg>, Receiver<ConfigMsg>) {
    mpsc::channel::<ConfigMsg>(128)
}

pub type BroadcastState = Mutex<broadcast::Receiver<CoreStatus>>;
pub fn broad_build() -> (
    broadcast::Sender<CoreStatus>,
    broadcast::Receiver<CoreStatus>,
) {
    broadcast::channel(16)
}
