//! Adam Smith bot component.

use uslib::parking_lot::Mutex;

/// discord bot.
pub static DISCORD_BOT: uslib::OnceCell<Mutex<DiscordBot>> = uslib::OnceCell::new();

/// Configuration for the discord bot.
#[derive(Debug)]
struct BotConfig {}

impl BotConfig {
    /// Load the discord bot configuration.
    pub async fn load() -> uslib::Result<Self> {
        uslib::debug!(uslib::LOGGER, "discord bot config: load\n");
        let config = Self {};
        uslib::trace!(uslib::LOGGER, "discord bot config: load: {:?}\n", config);
        Ok(config)
    }
}

#[derive(Debug)]
pub struct DiscordBot {}

impl DiscordBot {
    /// Initialize the AsBotClient.
    pub async fn init() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "discord bot: init\n");
        let config = BotConfig::load().await?;

        uslib::trace!(uslib::LOGGER, "discord bot: init: setting up singleton\n");
        if DISCORD_BOT
            .set(Mutex::new(DiscordBot {}))
            .is_err()
        {
            uslib::bail!("discord bot: init: already initialized\n");
        };
        uslib::trace!(uslib::LOGGER, "discord bot: init: singleton ok\n");

        uslib::trace!(uslib::LOGGER, "discord bot: init: ok\n");
        Ok(())
    }

    pub async fn start() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "discord bot: start\n");
        let server = DISCORD_BOT.get().unwrap().lock();
        uslib::trace!(uslib::LOGGER, "discord bot: start ok\n");
        Ok(())
    }

    pub async fn stop() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "discord bot: stop\n");
        let server = DISCORD_BOT.get().unwrap().lock();
        uslib::trace!(uslib::LOGGER, "discord bot: stop ok\n");
        Ok(())
    }
}
