use tokio::signal;
use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};

pub mod jwt;
pub mod password;
pub mod validator;

/// Initializes the logger for tracing.
///
/// This function sets up the necessary layers for tracing using the `tracing_subscriber`
/// crate. It configures the formatting layer and environment filter based on the value
/// of the `LIMOS_LOG` environment variable (defaulting to "info" if not set).
///
/// # Example
///
/// ```rust
/// use utils::init_logger;
///
/// fn test() {
///     init_logger();
/// }
/// ```
pub fn init_logger() {
    let formatting_layer = fmt::layer()
        // .pretty()
        .with_thread_ids(false)
        .with_target(false)
        .with_writer(std::io::stdout);

    let env_layer = EnvFilter::try_from_env("LIMOS_LOG").unwrap_or_else(|_| "info".into());

    registry().with(env_layer).with(formatting_layer).init();
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
