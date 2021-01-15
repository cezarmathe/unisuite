//! Adam Smith bot client.

use uslib::parking_lot::Mutex;
use uslib::proto::moodle_events_client::MoodleEventsClient;

/// Adam Smith bot client.
pub static CLIENT: uslib::OnceCell<Mutex<AsBotClient>> = uslib::OnceCell::new();

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
pub struct AsBotClient {
    config: AsBotClientConfig,
    pub mevents_client: MoodleEventsClient<uslib::tonic::transport::channel::Channel>,
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
        if CLIENT
            .set(Mutex::new(AsBotClient {
                config,
                mevents_client,
            }))
            .is_err()
        {
            uslib::bail!("asbot client: init: already initialized\n");
        };
        uslib::trace!(uslib::LOGGER, "asbot client: init: singleton ok\n");

        uslib::trace!(uslib::LOGGER, "asbot client: init: ok\n");
        Ok(())
    }
}
