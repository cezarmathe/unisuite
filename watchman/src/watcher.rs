//! File watcher.

use std::cell::RefCell;
use std::convert::TryFrom;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use hotwatch::{Event, Hotwatch};

use uslib::parking_lot::Mutex;
use uslib::LOGGER;

/// Rule watcher.
pub static RULE_WATCHER: uslib::OnceCell<Mutex<RuleWatcher>> = uslib::OnceCell::new();

/// A wrapper around a scraper rule.
#[derive(Debug)]
pub struct ScraperRule(pub String);

impl ScraperRule {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl TryFrom<&Path> for ScraperRule {
    type Error = anyhow::Error;

    fn try_from(src: &Path) -> anyhow::Result<ScraperRule> {
        if !src.starts_with("/var/usscraper/data") {
            bail!("scraper rule: bad path prefix");
        }
        if !src.ends_with("data.json") {
            bail!("scraper rule: bad suffix");
        }
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
            bail!("scraper rule: path does not have expected size");
        }
        if components[3] == "" {
            bail!("scraper rule: rule name is not valid utf-8");
        }
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
        rule_data_path
    }
}

/// Configuration for the Rule Watcher.
struct RuleWatcherConfig {
    rules: Vec<ScraperRule>,
}

impl RuleWatcherConfig {
    /// Load the rule watcher configuration.
    pub async fn load() -> anyhow::Result<Self> {
        let rules_string: String;
        match std::env::var("WATCHMAN_RULES") {
            Ok(value) => rules_string = value,
            Err(e) => bail!("rule watcher config: failed to load: {}\n", e),
        }

        let rules: Vec<ScraperRule> = rules_string
            .split(",")
            .map(|s| ScraperRule::new(s.to_string()))
            .collect();

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
    pub async fn init() -> anyhow::Result<()> {
        let config = RuleWatcherConfig::load().await?;

        if RULE_WATCHER
            .set(Mutex::new(RuleWatcher {
                config,
                hw: RefCell::new(Hotwatch::new()?),
            }))
            .is_err()
        {
            bail!("rule watcher: failed to init: already initialized\n");
        };

        Ok(())
    }

    /// Start the rule watcher.
    ///
    /// This will start watching all rules.
    pub async fn start() -> anyhow::Result<()> {
        let rule_watcher = RULE_WATCHER.get().unwrap().lock();
        let mut hw = rule_watcher.hw.borrow_mut();
        for rule in rule_watcher.config.rules.as_slice() {
            uslib::info!(LOGGER, "watching rule {:?}\n", rule);
            let path: &PathBuf = &rule.into();
            hw.watch(path, Self::handle_event)?
        }
        Ok(())
    }

    /// Stop the rule watcher.
    ///
    /// This will stop watching all rules.
    pub async fn stop() -> anyhow::Result<()> {
        let rule_watcher = RULE_WATCHER.get().unwrap().lock();
        let mut hw = rule_watcher.hw.borrow_mut();
        for rule in rule_watcher.config.rules.as_slice() {
            uslib::info!(LOGGER, "watching rule {:?}\n", rule);
            let path: &PathBuf = &rule.into();
            hw.unwatch(path)?
        }
        Ok(())
    }

    fn handle_event(event: Event) {
        let mut rule: Option<ScraperRule> = None;
        match event {
            Event::Create(path) => match ScraperRule::try_from(path.as_path()) {
                Ok(val) => rule = Some(val),
                Err(e) => uslib::warn!(
                    LOGGER,
                    "rule watcher: failed to get rule from path: {}\n",
                    e
                ),
            },
            Event::Write(path) => match ScraperRule::try_from(path.as_path()) {
                Ok(val) => rule = Some(val),
                Err(e) => uslib::warn!(
                    LOGGER,
                    "rule watcher: failed to get rule from path: {}\n",
                    e
                ),
            },
            Event::Error(err, path) => {
                uslib::error!(
                    LOGGER,
                    "rule watcher: error event received: {}, path {:?}\n",
                    err,
                    path
                );
            }
            _ => {
                uslib::debug!(LOGGER, "rule watcher: received other event: {:?}\n", event);
            }
        }
        if let Some(val) = rule {
            // let watcher_event = WatcherEvent::new(val);
            // uslib::info!(LOGGER, "rule watcher: watcher event: {:?}\n", watcher_event);
        }
    }
}
