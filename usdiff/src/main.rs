//! Unisuite diff(er).

use uslib::common::*;

#[tokio::main]
async fn main() {
    slog::info!(uslib::LOGGER, "main: hello world\n");

    slog::debug!(uslib::LOGGER, "main: waiting for termination signal\n");
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .unwrap()
        .recv()
        .await
        .unwrap();
    slog::debug!(
        uslib::LOGGER,
        "main: received termination signal, proceeding with graceful shutdown\n"
    );

    slog::info!(uslib::LOGGER, "goodbye\n");
}
