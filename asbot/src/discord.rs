//! Adam Smith bot component.

use uslib::common::*;

use std::sync::Arc;

use blockz::prelude::*;

use serde::Deserialize;

use serenity::http::Http;
use serenity::model::webhook::Webhook;

/// Configuration for the discord.
#[derive(Configuration, Debug, Deserialize)]
struct DiscordConfig {
    // discord token,
    token: String,
    // moodle webhook id
    moodle_webhook_id: u64,
    // moodle webhook token
    moodle_webhook_token: String,
}

#[derive(Debug, Singleton)]
pub struct Discord {
    config: DiscordConfig,
    http: Http,
    moodle_webhook: Webhook,
    // http_client: HttpClient,
}

impl Discord {
    /// Initialize the AsBotClient.
    pub async fn init() -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "discord: init\n");
        let config = DiscordConfig::load().await?;

        slog::trace!(uslib::LOGGER, "discord: init: setting up http client\n");
        let http = Http::new(
            Arc::new(reqwest::Client::builder().trust_dns(true).build()?),
            config.token.as_str(),
        );
        slog::trace!(uslib::LOGGER, "discord: init: setting up moodle webhook\n");
        let moodle_webhook: Webhook;
        match http
            .get_webhook_with_token(
                config.moodle_webhook_id,
                config.moodle_webhook_token.as_str(),
            )
            .await
        {
            Ok(value) => moodle_webhook = value,
            Err(e) => anyhow::bail!("discord: init: setting up moodle webhook: {:?}\n", e),
        }

        slog::trace!(uslib::LOGGER, "discord: init: setting up singleton\n");
        let discord = Self {
            config,
            http,
            moodle_webhook,
        };
        if let Err(e) = Self::init_singleton(discord) {
            anyhow::bail!("discord: init: {}\n", e);
        };
        slog::trace!(uslib::LOGGER, "discord: init: singleton ok\n");

        slog::trace!(uslib::LOGGER, "discord: init: ok\n");
        Ok(())
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "discord: start\n");
        slog::trace!(uslib::LOGGER, "discord: start ok\n");
        Ok(())
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "discord: stop\n");
        slog::trace!(uslib::LOGGER, "discord: stop ok\n");
        Ok(())
    }

    pub async fn execute_moodle_webhook(&self, msg: String) -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "discord: execute moodle webhook\n");

        if let Err(e) = self
            .moodle_webhook
            .execute(&self.http, false, |w| {
                w.content(msg);
                w
            })
            .await
        {
            anyhow::bail!("discord: execute moodle webhook: {}\n", e);
        }

        slog::trace!(uslib::LOGGER, "discord: execute moodle webhook: ok\n");
        Ok(())
    }
}
