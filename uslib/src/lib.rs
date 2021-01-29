//! Unisuite library.

/// Proper re-export of the async_trait crate.
pub mod async_trait {
    pub use tonic::async_trait;
}

/// Common dependencies, to be imported as:
///
/// ```rust
/// #[allow(unused_imports)]
/// pub use uslib::common::*;
///
/// at the top of every source file.
/// ```
pub mod common {
    pub use crate::async_trait;
    pub use crate::proto;
    pub use crate::types;
    pub use anyhow;
    pub use blockz;
    pub use blockz::blockz_derive::*;
    pub use chrono;
    pub use config;
    pub use envy;
    pub use once_cell;
    pub use reqwest;
    pub use slog;
    pub use tokio;
    pub use tonic;
    pub use url;
}

/// Models.
pub mod model;

/// Protobuf definitions.
pub mod proto {
    tonic::include_proto!("com.cezarmathe.unisuite");
}

/// Common types.
pub mod types;

use once_cell::sync::Lazy;

use slog::Drain;
use slog::Level;
use slog::Logger;
use slog_syslog::Facility;

/// Root logger.
///
/// Can be used either as-is, or used in another Lazy context for building a child logger.
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
