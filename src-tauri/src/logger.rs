use chrono::Local;
use std::io::Write;
use std::sync::Arc;
use tauri::async_runtime;

use env_logger::{Builder, Env};

use crate::{
    message::ConfigMsg,
    utils::error::{VError, VResult},
};
use tokio::sync::mpsc::Sender;

pub fn init_logger(tx: Arc<Sender<ConfigMsg>>) -> VResult<()> {
    let env = Env::default().filter_or("RUA_LOG_LEVEL", "info");
    let mut builder = Builder::from_env(env);
    let now = Local::now();

    builder
        .format(move |buf, record| {
            let formatted = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));
            let log = format!("{} - {} - {}", formatted, record.level(), record.args());

            let emit_log = log.clone();
            let tx = tx.clone();
            async_runtime::spawn(async move {
                tx.send(ConfigMsg::EmitLog(emit_log)).await?;
                Ok::<(), VError>(())
            });
            writeln!(buf, "{log}")
        })
        .init();
    // env_logger::init_from_env(env);
    Ok(())
}
