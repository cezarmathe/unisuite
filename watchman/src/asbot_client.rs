//! Adam Smith bot client.

use uslib::common::*;

use blockz::prelude::*;

use proto::moodle_events_client::MoodleEventsClient;
use proto::NotifyRequest;

use serde::Deserialize;

use types::Url;

/// Configuration for the AsBotClient.
#[derive(Configuration, Debug, Deserialize)]
pub struct AsBotClientConfig {
    asbot_address: Url,
}

/// Adam Smith bot client.
#[derive(Debug, Singleton)]
pub struct AsBotClient {
    mevents_client: MoodleEventsClient<tonic::transport::channel::Channel>,
}

impl AsBotClient {
    /// Initialize the AsBotClient.
    pub async fn init(config: AsBotClientConfig) -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "asbot client: init\n");
        let mevents_client: MoodleEventsClient<tonic::transport::channel::Channel>;

        slog::debug!(
            uslib::LOGGER,
            "asbot client: init: connecting to {}\n",
            *config.asbot_address
        );
        let addr: String = config.asbot_address.as_ref().into();
        match MoodleEventsClient::connect(addr).await {
            Ok(value) => mevents_client = value,
            Err(e) => anyhow::bail!("asbot client: init: {}\n", e),
        }
        slog::trace!(uslib::LOGGER, "asbot client: init: connection ok\n");

        slog::trace!(uslib::LOGGER, "asbot client: init: setting up singleton\n");
        let asbot_client = Self {
            mevents_client,
        };
        if let Err(e) = Self::init_singleton(asbot_client) {
            anyhow::bail!("asbot client: init: {}\n", e);
        };
        slog::trace!(uslib::LOGGER, "asbot client: init: singleton ok\n");

        slog::trace!(uslib::LOGGER, "asbot client: init: ok\n");
        Ok(())
    }
}

impl AsBotClient {
    /// Send a notify event to the Adam Smith bot.
    pub async fn notify(&mut self, rule: String) -> anyhow::Result<()> {
        if let Err(e) = self.mevents_client.notify(NotifyRequest { rule }).await {
            anyhow::bail!("asbot client: notify: {}", e);
        }
        Ok(())
    }
}
