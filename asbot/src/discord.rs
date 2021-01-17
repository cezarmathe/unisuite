//! Adam Smith bot component.

use std::str::FromStr;
use std::sync::Arc;

use serenity::http::Http;
use serenity::model::webhook::Webhook;

use uslib::tokio;

use tokio::sync::Mutex;

/// discord.
pub static DISCORD: uslib::OnceCell<Mutex<Discord>> = uslib::OnceCell::new();

/// Configuration for the discord.
#[derive(Debug)]
struct DiscordConfig {
    // discord token,
    token: String,
    // moodle webhook id
    moodle_webhook_id: u64,
    // moodle webhook token
    moodle_webhook_token: String,
}

impl DiscordConfig {
    /// Load the discord configuration.
    pub async fn load() -> uslib::Result<Self> {
        uslib::debug!(uslib::LOGGER, "discord config: load\n");
        let token: String;
        match std::env::var("ASBOT_DISCORD_TOKEN") {
            Ok(value) => token = value,
            Err(e) => uslib::bail!(
                "discord config: load: ASBOT_DISCORD_TOKEN not found: {}\n",
                e
            ),
        }
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
            token,
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
    http: Http,
    moodle_webhook: Webhook,
    // http_client: HttpClient,
}

impl Discord {
    /// Initialize the AsBotClient.
    pub async fn init() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "discord: init\n");
        let config = DiscordConfig::load().await?;

        uslib::trace!(uslib::LOGGER, "discord: init: setting up http client\n");
        let http = Http::new(
            Arc::new(uslib::reqwest::Client::builder().trust_dns(true).build()?),
            config.token.as_str(),
        );
        uslib::trace!(uslib::LOGGER, "discord: init: setting up moodle webhook\n");
        let moodle_webhook: Webhook;
        match http
            .get_webhook_with_token(
                config.moodle_webhook_id,
                config.moodle_webhook_token.as_str(),
            )
            .await
        {
            Ok(value) => moodle_webhook = value,
            Err(e) => uslib::bail!("discord: init: setting up moodle webhook: {:?}\n", e),
        }

        uslib::trace!(uslib::LOGGER, "discord: init: setting up singleton\n");
        if DISCORD
            .set(Mutex::new(Discord {
                config,
                http,
                moodle_webhook,
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

    pub async fn execute_moodle_webhook(&self, msg: String) -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "discord: execute moodle webhook\n");

        if let Err(e) = self
            .moodle_webhook
            .execute(&self.http, false, |w| {
                w.content(msg);
                w
            })
            .await
        {
            uslib::bail!("discord: execute moodle webhook: {}\n", e);
        }

        uslib::trace!(uslib::LOGGER, "discord: execute moodle webhook: ok\n");
        Ok(())
    }
}
