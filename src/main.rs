use dotenvy::dotenv;
use error::AppResult;
use tracing::info;
use utils::init_logger;

mod config;
mod consts;
mod error;
mod utils;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    init_logger();

    info!("Hello, world!");
    Ok(())
}
