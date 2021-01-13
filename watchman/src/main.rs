#[macro_use]
extern crate anyhow;

mod asbot_client;
mod watcher;

#[tokio::main]
async fn main() {
    uslib::info!(uslib::LOGGER, "hello world\n");

    // initialization
    if let Err(e) = asbot_client::AsBotClient::init().await {
        uslib::crit!(uslib::LOGGER, "initialization: {}\n", e);
        return;
    }
    if let Err(e) = watcher::RuleWatcher::init().await {
        uslib::crit!(uslib::LOGGER, "initialization: {}\n", e);
        return;
    }

    // starting
    {
        if let Err(e) = watcher::RuleWatcher::start().await {
            uslib::crit!(uslib::LOGGER, "starting: {}\n", e);
            return;
        }
    }

    // waiting for termination signal
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .unwrap()
        .recv()
        .await
        .unwrap();

    // graceful shutdown
    {
        if let Err(e) = watcher::RuleWatcher::stop().await {
            uslib::error!(uslib::LOGGER, "stopping: {}\n", e);
            return;
        }
    }

    uslib::info!(uslib::LOGGER, "goodbye\n");
}
