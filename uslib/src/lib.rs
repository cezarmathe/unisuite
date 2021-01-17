//! Unisuite library.

use once_cell::sync::Lazy;

use slog::Drain;
use slog::Level;
use slog::Logger;
use slog_syslog::Facility;

// Common dependencies.
pub use once_cell;
pub use reqwest;
pub use slog;
pub use tokio;
pub use tonic;

// Useful re-exports.
pub use anyhow::bail;
pub use anyhow::Error;
pub use anyhow::Result;
pub use once_cell::sync::OnceCell;
pub use slog::crit;
pub use slog::debug;
pub use slog::error;
pub use slog::info;
pub use slog::trace;
pub use slog::warn;
pub use tonic::async_trait;

pub mod proto {
    tonic::include_proto!("com.cezarmathe.unisuite");
}

/// Root logger.
pub static LOGGER: Lazy<Logger> = Lazy::new(|| {
    use std::net::SocketAddr;
    use std::net::ToSocketAddrs;
    use std::str::FromStr;

    let server = std::env::var("SYSLOG")
        .expect("Expected SYSLOG environment variable pointing to syslog server.")
        .to_socket_addrs()
        .expect("Failed to get syslog server address.")
        .collect::<Vec<SocketAddr>>()
        .get(0)
        .unwrap()
        .clone();

    let level = Level::from_str(
        std::env::var("LOG_LEVEL")
            .expect("Expected LOG_LEVEL environment variable.")
            .as_str(),
    )
    .expect("Failed to parse log level from environment variable.");

    let syslog = slog_syslog::SyslogBuilder::new()
        .facility(Facility::LOG_LOCAL0)
        .level(level)
        .tcp(
            server,
            hostname::get()
                .expect("Failed to get hostname.")
                .into_string()
                .unwrap(),
        )
        .start()
        .expect("Failed to start syslog client.");

    Logger::root(syslog.fuse(), slog::o!())
});
