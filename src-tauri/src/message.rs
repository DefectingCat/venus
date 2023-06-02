use tokio::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

pub enum ConfigMsgType {
    CoreStatue,
}
pub struct ConfigMsg {
    pub msg_typs: ConfigMsgType,
    pub data: String,
}

pub fn msg_build() -> (Sender<ConfigMsg>, Receiver<ConfigMsg>) {
    mpsc::channel::<ConfigMsg>(128)
}