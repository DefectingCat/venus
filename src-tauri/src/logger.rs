use chrono::Local;
use std::io::Write;
use tauri::async_runtime;

use env_logger::{Builder, Env};

use crate::config::ConfigState;
use crate::message::MsgSender;
use crate::{
    message::ConfigMsg,
    utils::error::{VError, VResult},
};

pub fn init_logger(tx: MsgSender, config: ConfigState) -> VResult<()> {
    let env = Env::default().filter_or("RUA_LOG_LEVEL", "info");
    let mut builder = Builder::from_env(env);

    builder
        .format(move |buf, record| {
            let now = Local::now();
            let formatted = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));
            let log = format!("{} - {} - {}", formatted, record.level(), record.args());

            let emit_log = log.clone();
            let tx = tx.clone();
            let config = config.clone();
            async_runtime::spawn(async move {
                let config = config.lock().await;
                if config.rua.logging {
                    tx.send(ConfigMsg::EmitLog(emit_log)).await?;
                }
                Ok::<(), VError>(())
            });
            writeln!(buf, "{log}")
        })
        .init();
    // env_logger::init_from_env(env);
    Ok(())
}
