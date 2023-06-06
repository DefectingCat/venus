use std::sync::Arc;

use tokio::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

use crate::config::CoreStatus;

#[derive(Debug)]
pub enum ConfigMsg {
    CoreStatue(CoreStatus),
}
// pub struct ConfigMsg {
//     pub msg: ConfigMsgType,
// }

pub type MsgSender = Arc<Sender<ConfigMsg>>;

pub fn msg_build() -> (Sender<ConfigMsg>, Receiver<ConfigMsg>) {
    mpsc::channel::<ConfigMsg>(128)
}