//! File watcher.

use crate::asbot_client::CLIENT as ASBOT_CLIENT;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use hotwatch::{Event, Hotwatch};

use uslib::parking_lot::Mutex;
use uslib::proto::NotifyRequest;

/// Rule watcher.
pub static RULE_WATCHER: uslib::OnceCell<Mutex<RuleWatcher>> = uslib::OnceCell::new();

/// A wrapper around a scraper rule.
#[derive(Debug)]
pub struct ScraperRule(pub String);

impl ScraperRule {
    /// Create a new ScraperRule.
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl TryFrom<&Path> for ScraperRule {
    type Error = uslib::Error;

    fn try_from(src: &Path) -> uslib::Result<ScraperRule> {
        uslib::trace!(uslib::LOGGER, "scraper rule: try from path\n");
        if !src.starts_with("/var/usscraper/data") {
            uslib::bail!("scraper rule: try from path: bad path prefix");
        }
        uslib::trace!(
            uslib::LOGGER,
            "scraper rule: try from path: path prefix ok\n"
        );
        if !src.ends_with("data.json") {
            uslib::bail!("scraper rule: try from path: bad suffix");
        }
        uslib::trace!(
            uslib::LOGGER,
            "scraper rule: try from path: path suffix ok\n"
        );
        let components: Vec<&str> = src
            .components()
            .filter(|c| {
                if let Component::Normal(_) = c {
                    true
                } else {
                    false
                }
            })
            .map(|c| c.as_os_str().to_str().unwrap_or(""))
            .collect();
        if components.len() != 5 {
            uslib::bail!("scraper rule: try from path: path does not have expected size");
        }
        uslib::trace!(
            uslib::LOGGER,
            "scraper rule: try from path: path length ok\n"
        );
        if components[3] == "" {
            uslib::bail!("scraper rule: try from path: rule name is not valid utf-8\n");
        }
        uslib::trace!(
            uslib::LOGGER,
            "scraper rule: try from path: path encoding ok: {}\n",
            components[3]
        );
        Ok(ScraperRule(components[3].to_string()))
    }
}

impl Into<PathBuf> for &ScraperRule {
    fn into(self) -> PathBuf {
        // 80 = 20(/var/usscraper/data/) + 50(rule name length) + 10(/data.json)
        let mut rule_data_path = PathBuf::with_capacity(80);
        rule_data_path.push("/var/usscraper/data");
        rule_data_path.push(&self.0);
        rule_data_path.push("data.json");
        uslib::trace!(
            uslib::LOGGER,
            "scraper rule: into pathbuf: {}\n",
            rule_data_path.display()
        );
        rule_data_path
    }
}

/// Configuration for the Rule Watcher.
struct RuleWatcherConfig {
    rules: Vec<ScraperRule>,
}

impl RuleWatcherConfig {
    /// Load the rule watcher configuration.
    pub async fn load() -> uslib::Result<Self> {
        uslib::debug!(uslib::LOGGER, "rule watcher config: load\n");
        let rules_string: String;
        match std::env::var("WATCHMAN_RULES") {
            Ok(value) => rules_string = value,
            Err(e) => uslib::bail!(
                "rule watcher config: load: WATCHMAN_RULES not found: {}\n",
                e
            ),
        }
        uslib::trace!(
            uslib::LOGGER,
            "rule watcher config: load: environment variable ok\n"
        );
        let rules: Vec<ScraperRule> = rules_string
            .split(",")
            .map(|s| ScraperRule::new(s.to_string()))
            .collect();
        uslib::trace!(
            uslib::LOGGER,
            "rule watcher config: load: rules ok: {:?}\n",
            rules
        );
        Ok(Self { rules })
    }
}

/// The rule watcher.
pub struct RuleWatcher {
    config: RuleWatcherConfig,
    hw: RefCell<Hotwatch>,
}

impl RuleWatcher {
    /// Initialize the rule watcher.
    pub async fn init() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "rule watcher: init\n");
        let config = RuleWatcherConfig::load().await?;

        uslib::trace!(uslib::LOGGER, "rule watcher: init: setting up singleton\n");
        if RULE_WATCHER
            .set(Mutex::new(RuleWatcher {
                config,
                hw: RefCell::new(Hotwatch::new()?),
            }))
            .is_err()
        {
            uslib::bail!("rule watcher: failed to init: already initialized\n");
        };
        uslib::trace!(uslib::LOGGER, "rule watcher: init: singleton ok\n");

        uslib::trace!(uslib::LOGGER, "rule watcher: init: ok\n");
        Ok(())
    }

    /// Start the rule watcher.
    ///
    /// This will start watching all rules.
    pub async fn start() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "rule watcher: start\n");
        let rule_watcher = RULE_WATCHER.get().unwrap().lock();
        let mut hw = rule_watcher.hw.borrow_mut();
        for rule in rule_watcher.config.rules.as_slice() {
            uslib::trace!(
                uslib::LOGGER,
                "rule watcher: start: watching rule {:?}\n",
                rule
            );
            let path: &PathBuf = &rule.into();
            hw.watch(path, Self::handle_event)?
        }
        uslib::trace!(uslib::LOGGER, "rule watcher: start ok\n");
        Ok(())
    }

    /// Stop the rule watcher.
    ///
    /// This will stop watching all rules.
    pub async fn stop() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "rule watcher: stop\n");
        let rule_watcher = RULE_WATCHER.get().unwrap().lock();
        let mut hw = rule_watcher.hw.borrow_mut();
        for rule in rule_watcher.config.rules.as_slice() {
            uslib::trace!(
                uslib::LOGGER,
                "rule watcher: stop: watching rule {:?}\n",
                rule
            );
            let path: &PathBuf = &rule.into();
            hw.unwatch(path)?
        }
        uslib::trace!(uslib::LOGGER, "rule watcher: stop ok\n");
        Ok(())
    }

    #[tokio::main]
    async fn handle_event(event: Event) {
        uslib::trace!(uslib::LOGGER, "rule watcher: handle event\n");
        let mut rule: Option<ScraperRule> = None;
        match event {
            Event::Create(path) => match ScraperRule::try_from(path.as_path()) {
                Ok(val) => rule = Some(val),
                Err(e) => uslib::warn!(
                    uslib::LOGGER,
                    "rule watcher: handle event: failed to get rule from path: {}\n",
                    e
                ),
            },
            Event::Write(path) => match ScraperRule::try_from(path.as_path()) {
                Ok(val) => rule = Some(val),
                Err(e) => uslib::warn!(
                    uslib::LOGGER,
                    "rule watcher: handle event: failed to get rule from path: {}\n",
                    e
                ),
            },
            Event::Error(err, path) => {
                uslib::warn!(
                    uslib::LOGGER,
                    "rule watcher: handle event: error event received: {}, path {:?}\n",
                    err,
                    path
                );
            }
            _ => {
                uslib::debug!(
                    uslib::LOGGER,
                    "rule watcher: handle event: received other event: {:?}\n",
                    event
                );
            }
        }
        if let Some(val) = rule {
            uslib::info!(
                uslib::LOGGER,
                "rule watcher: handle event: received meaningful event: {}\n",
                val.0.as_str()
            );
            let mut client = ASBOT_CLIENT.get().unwrap().lock();
            let mevents = &mut client.mevents_client;
            if let Err(e) = mevents.notify(NotifyRequest {}).await {
                uslib::error!(uslib::LOGGER, "rule watcher: handle event: {}", e);
            }
        }
    }
}
