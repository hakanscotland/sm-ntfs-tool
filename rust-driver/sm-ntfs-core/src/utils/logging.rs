//! Logging utilities

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize logging for the application
pub fn init_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sm_ntfs=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
