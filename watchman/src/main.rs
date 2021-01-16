//! Watchman

mod asbot_client;
mod watcher;

use uslib::tokio as tokio;

#[tokio::main]
async fn main() {
    uslib::info!(uslib::LOGGER, "main: hello world\n");

    // initialization

    uslib::debug!(uslib::LOGGER, "main: initializing asbot client\n");
    if let Err(e) = asbot_client::AsBotClient::init().await {
        uslib::crit!(uslib::LOGGER, "main: initializing asbot client: {}\n", e);
        return;
    }
    uslib::debug!(uslib::LOGGER, "main: initializing rule watcher\n");
    if let Err(e) = watcher::RuleWatcher::init().await {
        uslib::crit!(uslib::LOGGER, "main: initializing rule watcher: {}\n", e);
        return;
    }

    // start

    {
        uslib::debug!(uslib::LOGGER, "main: starting rule watcher\n");
        if let Err(e) = watcher::RuleWatcher::start().await {
            uslib::crit!(uslib::LOGGER, "main: starting rule watcher: {}\n", e);
            return;
        }
    }

    // wait for termination

    uslib::debug!(uslib::LOGGER, "main: waiting for termination signal\n");
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .unwrap()
        .recv()
        .await
        .unwrap();
    uslib::debug!(
        uslib::LOGGER,
        "main: received termination signal, proceeding with graceful shutdown\n"
    );

    // graceful shutdown

    {
        uslib::debug!(uslib::LOGGER, "main: stopping rule watcher\n");
        if let Err(e) = watcher::RuleWatcher::stop().await {
            uslib::error!(uslib::LOGGER, "main: stopping rule watcher: {}\n", e);
            return;
        }
    }

    // done

    uslib::info!(uslib::LOGGER, "goodbye\n");
}
