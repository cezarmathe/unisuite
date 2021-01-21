//! Watchman

mod asbot_client;
mod watcher;

use uslib::common::*;

use asbot_client::AsBotClient;

use blockz::prelude::*;

use watcher::RuleWatcher;

#[tokio::main]
async fn main() {
    slog::info!(uslib::LOGGER, "main: hello world\n");

    // initialization

    slog::debug!(uslib::LOGGER, "main: initializing asbot client\n");
    if let Err(e) = AsBotClient::init().await {
        slog::crit!(uslib::LOGGER, "main: initializing asbot client: {}\n", e);
        return;
    }
    slog::debug!(uslib::LOGGER, "main: initializing rule watcher\n");
    if let Err(e) = RuleWatcher::init().await {
        slog::crit!(uslib::LOGGER, "main: initializing rule watcher: {}\n", e);
        return;
    }

    // start

    slog::debug!(uslib::LOGGER, "main: starting rule watcher\n");
    if let Err(e) = RuleWatcher::use_mut_singleton(RuleWatcher::start).await {
        slog::crit!(uslib::LOGGER, "main: starting rule watcher: {}\n", e);
        return;
    }

    // wait for termination

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

    // graceful shutdown

    slog::debug!(uslib::LOGGER, "main: stopping rule watcher\n");
    if let Err(e) = RuleWatcher::use_mut_singleton(RuleWatcher::stop).await {
        slog::error!(uslib::LOGGER, "main: stopping rule watcher: {}\n", e);
        return;
    }

    // done

    slog::info!(uslib::LOGGER, "goodbye\n");
}
