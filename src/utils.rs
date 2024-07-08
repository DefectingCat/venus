use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};

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
