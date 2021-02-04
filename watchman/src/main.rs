//! Watchman

mod asbot_client;
mod server;
mod services;
mod watcher;

use uslib::common::*;

use std::sync::Arc;

use asbot_client::AsBotClient;
use asbot_client::AsBotClientConfig;

use blockz::prelude::*;

use watcher::RuleWatcher;
use watcher::RuleWatcherConfig;

const ENV_PREFIX: &str = "WATCHMAN_";

#[tokio::main]
async fn main() {
    slog::info!(uslib::LOGGER, "main: hello world\n");

    // initialization

    slog::debug!(uslib::LOGGER, "main: initializing asbot client\n");
    let asbot_config = match AsBotClientConfig::load(Some(ENV_PREFIX.to_string())).await {
        Ok(value) => value,
        Err(e) => {
            slog::crit!(
                uslib::LOGGER,
                "main: initializing asbot client: config: {}\n",
                e
            );
            return;
        }
    };
    if let Err(e) = AsBotClient::init(asbot_config).await {
        slog::crit!(uslib::LOGGER, "main: initializing asbot client: {}\n", e);
        return;
    }

    slog::debug!(uslib::LOGGER, "main: initializing rule watcher\n");
    let rule_watcher_config: Arc<_> =
        match RuleWatcherConfig::load(Some(ENV_PREFIX.to_string())).await {
            Ok(value) => Arc::new(value),
            Err(e) => {
                slog::crit!(uslib::LOGGER, "main: initializing rule watcher: {}\n", e);
                return;
            }
        };
    if let Err(e) = RuleWatcher::init().await {
        slog::crit!(uslib::LOGGER, "main: initializing rule watcher: {}\n", e);
        return;
    }

    // start

    slog::debug!(uslib::LOGGER, "main: starting rule watcher\n");
    if let Err(e) =
        RuleWatcher::use_mut_singleton_with_arg(RuleWatcher::start, rule_watcher_config.clone())
            .await
    {
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
    if let Err(e) =
        RuleWatcher::use_mut_singleton_with_arg(RuleWatcher::stop, rule_watcher_config).await
    {
        slog::error!(uslib::LOGGER, "main: stopping rule watcher: {}\n", e);
        return;
    }

    // done

    slog::info!(uslib::LOGGER, "goodbye\n");
}
