//! Adam Smith bot component.

use std::collections::HashMap;
use std::str::FromStr;

use reqwest::Client as HttpClient;

use uslib::parking_lot::Mutex;

/// discord.
pub static DISCORD: uslib::OnceCell<Mutex<Discord>> = uslib::OnceCell::new();

/// Configuration for the discord.
#[derive(Debug)]
struct DiscordConfig {
    // moodle webhook id
    moodle_webhook_id: u64,
    // moodle webhook token
    moodle_webhook_token: String,
}

impl DiscordConfig {
    /// Load the discord configuration.
    pub async fn load() -> uslib::Result<Self> {
        uslib::debug!(uslib::LOGGER, "discord config: load\n");
        let moodle_webhook_id: u64;
        match std::env::var("ASBOT_DISCORD_WEBHOOK_ID") {
            Ok(value) => moodle_webhook_id = u64::from_str(value.as_str())?,
            Err(e) => uslib::bail!(
                "discord config: load: ASBOT_DISCORD_WEBHOOK_ID not found: {}\n",
                e
            ),
        }
        let moodle_webhook_token: String;
        match std::env::var("ASBOT_DISCORD_WEBHOOK_TOKEN") {
            Ok(value) => moodle_webhook_token = value,
            Err(e) => uslib::bail!(
                "discord config: load: ASBOT_DISCORD_WEBHOOK_TOKEN not found: {}\n",
                e
            ),
        }
        let config = Self {
            moodle_webhook_id,
            moodle_webhook_token,
        };
        uslib::trace!(uslib::LOGGER, "discord config: load: {:?}\n", config);
        Ok(config)
    }
}

#[derive(Debug)]
pub struct Discord {
    config: DiscordConfig,
    http_client: HttpClient,
}

impl Discord {
    /// Initialize the AsBotClient.
    pub async fn init() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "discord: init\n");
        let config = DiscordConfig::load().await?;
        let http_client = HttpClient::new();

        uslib::trace!(uslib::LOGGER, "discord: init: setting up singleton\n");
        if DISCORD
            .set(Mutex::new(Discord {
                config,
                http_client,
            }))
            .is_err()
        {
            uslib::bail!("discord: init: already initialized\n");
        };
        uslib::trace!(uslib::LOGGER, "discord: init: singleton ok\n");

        uslib::trace!(uslib::LOGGER, "discord: init: ok\n");
        Ok(())
    }

    pub async fn start() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "discord: start\n");
        let server = DISCORD.get().unwrap().lock();
        uslib::trace!(uslib::LOGGER, "discord: start ok\n");
        Ok(())
    }

    pub async fn stop() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "discord: stop\n");
        let server = DISCORD.get().unwrap().lock();
        uslib::trace!(uslib::LOGGER, "discord: stop ok\n");
        Ok(())
    }

    #[tokio::main]
    pub async fn send_moodle_update(&self, msg: String) -> uslib::Result<()> {
        let json = {
            let mut json = HashMap::new();
            json.insert("content", msg);
            json
        };
        let req = self.http_client.post(format!("https://discord.com/api/v8/webhooks/{}/{}", self.config.moodle_webhook_id, self.config.moodle_webhook_token).as_str());
        if let Err(e) = req.json(&json).send().await {
            uslib::bail!("webhook: send: {}", e);
        }
        Ok(())
    }
}
