//! Adam Smith bot client.

use uslib::blockz_prelude::*;
use uslib::proto::moodle_events_client::MoodleEventsClient;
use uslib::proto::NotifyRequest;

/// Configuration for the AsBotClient.
#[derive(Debug)]
struct AsBotClientConfig {
    address: String,
}

impl AsBotClientConfig {
    /// Load the AsBot client configuration.
    pub async fn load() -> uslib::Result<Self> {
        uslib::debug!(uslib::LOGGER, "asbot client config: load\n");
        let address: String;
        match std::env::var("WATCHMAN_ASBOT_ADDRESS") {
            Ok(value) => address = value.to_string(),
            Err(e) => uslib::bail!(
                "asbot client config: load: WATCHMAN_ASBOT_ADDRESS not found: {}\n",
                e
            ),
        }
        let config = Self { address };
        uslib::trace!(uslib::LOGGER, "asbot client config: load: {:?}\n", config);
        Ok(config)
    }
}

/// Adam Smith bot client.
#[derive(Debug, Singleton)]
pub struct AsBotClient {
    config: AsBotClientConfig,
    mevents_client: MoodleEventsClient<uslib::tonic::transport::channel::Channel>,
}

impl AsBotClient {
    /// Initialize the AsBotClient.
    pub async fn init() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "asbot client: init\n");
        let config = AsBotClientConfig::load().await?;
        let mevents_client: MoodleEventsClient<uslib::tonic::transport::channel::Channel>;

        uslib::debug!(
            uslib::LOGGER,
            "asbot client: init: connecting to {}\n",
            config.address.as_str()
        );
        match MoodleEventsClient::connect(config.address.clone()).await {
            Ok(value) => mevents_client = value,
            Err(e) => uslib::bail!("asbot client: init: {}\n", e),
        }
        uslib::trace!(uslib::LOGGER, "asbot client: init: connection ok\n");

        uslib::trace!(uslib::LOGGER, "asbot client: init: setting up singleton\n");
        let asbot_client = Self {
            config,
            mevents_client,
        };
        if let Err(e) = Self::init_singleton(asbot_client) {
            uslib::bail!("asbot client: init: {}\n", e);
        };
        uslib::trace!(uslib::LOGGER, "asbot client: init: singleton ok\n");

        uslib::trace!(uslib::LOGGER, "asbot client: init: ok\n");
        Ok(())
    }

    /// Send a notify event to the Adam Smith bot.
    pub async fn notify(&mut self, rule: String) -> anyhow::Result<()> {
        if let Err(e) = self.mevents_client.notify(NotifyRequest { rule }).await {
            uslib::bail!("asbot client: notify: {}", e);
        }
        Ok(())
    }
}
