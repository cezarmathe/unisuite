//! Unisuite library.

#[allow(unused_imports)]
#[macro_use]
pub extern crate blockz;
pub use blockz::*;

use once_cell::sync::Lazy;

use slog::Drain;
use slog::Level;
use slog::Logger;
use slog_syslog::Facility;

// Common dependencies.
pub use anyhow;
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

/// Proper re-export of the async_trait crate.
pub mod async_trait {
    pub use tonic::async_trait;
}

/// Useful container for all required imports for using blockz.
pub mod blockz_prelude {
    pub use crate::anyhow;
    pub use crate::async_trait;
    pub use crate::blockz;
    pub use crate::blockz::singleton::Singleton;
    pub use crate::once_cell;
    pub use crate::tokio;
}

/// Protobuf definitions.
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
