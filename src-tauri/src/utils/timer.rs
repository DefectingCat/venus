use anyhow::{anyhow, bail};
use std::time::Duration;
use tauri::async_runtime::{self, JoinHandle};
use tokio::time::sleep;

/// A timer
pub struct Timer {
    pub duration: Duration,
    pub job: fn(),
    handler: Option<JoinHandle<()>>,
}

impl Timer {
    pub fn new(time: u64, job: fn()) -> Self {
        let duration = Duration::from_secs(time);
        Self {
            duration,
            job,
            handler: None,
        }
    }

    /// Start a new async thread by `async_runtime::spwn()`
    pub fn start(&mut self) -> anyhow::Result<()> {
        if self.handler.is_some() {
            bail!("already has a job, terminate first");
        }

        let job = self.job;
        let duration = self.duration;
        let handler = async_runtime::spawn(async move {
            loop {
                sleep(duration).await;
                (job)();
            }
        });
        self.handler = Some(handler);
        Ok(())
    }

    pub fn terminate(&mut self) -> anyhow::Result<()> {
        let handler = self.handler.take();
        handler.ok_or(anyhow!(""))?.abort();
        Ok(())
    }
}
