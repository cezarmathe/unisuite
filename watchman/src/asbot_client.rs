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
    pub async fn load() -> anyhow::Result<Self> {
        let address: String;
        match std::env::var("WATCHMAN_ASBOT_ADDRESS") {
            Ok(value) => address = value.to_string(),
            Err(e) => bail!("asbot client config: failed to load: {}\n", e),
        }
        Ok(Self { address })
    }
}

/// Adam Smith bot client.
pub struct AsBotClient {
    config: AsBotClientConfig,
    pub mevents_client: MoodleEventsClient<tonic::transport::channel::Channel>,
}

impl AsBotClient {
    /// Initialize the AsBotClient.
    pub async fn init() -> anyhow::Result<()> {
        let config = AsBotClientConfig::load().await?;
        let mevents_client: MoodleEventsClient<tonic::transport::channel::Channel>;

        uslib::info!(
            uslib::LOGGER,
            "asbot client: connecting to {}\n",
            config.address.as_str()
        );
        match MoodleEventsClient::connect(config.address.clone()).await {
            Ok(value) => mevents_client = value,
            Err(e) => bail!("asbot client: failed to init: {}\n", e),
        }

        if CLIENT
            .set(Mutex::new(AsBotClient {
                config,
                mevents_client,
            }))
            .is_err()
        {
            bail!("asbot client: failed to init: already initialized\n");
        };

        Ok(())
    }
}
