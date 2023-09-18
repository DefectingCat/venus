use chrono::Local;
use std::io::Write;
use tauri::async_runtime;

use env_logger::{Builder, Env};

use crate::message::{ConfigMsg, MSG_TX};
use crate::LOGGING;
use anyhow::{Ok, Result};

pub fn init_logger() -> Result<()> {
    let env = Env::default().filter_or("RUA_LOG_LEVEL", "info");
    let mut builder = Builder::from_env(env);

    builder
        .format(move |buf, record| {
            let now = Local::now();
            let formatted = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));
            let log = format!("{} - {} - {}", formatted, record.level(), record.args());

            let emit_log = log.clone();
            async_runtime::spawn(async move {
                if LOGGING.load(std::sync::atomic::Ordering::Relaxed) {
                    MSG_TX.send(ConfigMsg::EmitLog(emit_log)).await?;
                }
                Ok(())
            });
            writeln!(buf, "{log}")
        })
        .init();
    // env_logger::init_from_env(env);
    Ok(())
}
