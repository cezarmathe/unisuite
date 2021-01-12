//! File watcher.

use crate::config::ScraperRule;

use std::convert::TryFrom;
use std::path::PathBuf;

use hotwatch::{Event, Hotwatch};

use uslib::LOGGER;

/// An event raised by the watcher.
/// We only really care if the file changed, at this point.
#[derive(Debug)]
pub struct WatcherEvent {
    rule: ScraperRule,
    cg_detected: std::time::Instant,
}

impl WatcherEvent {
    /// Create a new WatcherEvent.
    pub fn new(rule: ScraperRule) -> Self {
        Self {
            rule,
            cg_detected: std::time::Instant::now(),
        }
    }
}

pub struct RuleWatcher {
    rules: Vec<ScraperRule>,
    hw: Hotwatch,
}

impl RuleWatcher {
    /// Create a new rule watcher.
    pub fn new(rules: Vec<ScraperRule>) -> anyhow::Result<Self> {
        Ok(Self {
            rules,
            hw: Hotwatch::new()?,
        })
    }

    /// Start the rule watcher.
    ///
    /// This will start watching all rules.
    pub async fn start(&mut self) -> anyhow::Result<()> {
        for rule in self.rules.as_slice() {
            uslib::info!(LOGGER, "watching rule {:?}\n", rule);
            let path: &PathBuf = &rule.into();
            self.hw.watch(path, Self::handle_event)?
        }

        Ok(())
    }

    /// Stop the rule watcher.
    ///
    /// This will stop watching all rules.
    pub async fn stop(&mut self) -> anyhow::Result<()> {
        for rule in self.rules.as_slice() {
            uslib::info!(LOGGER, "unwatching rule {:?}\n", rule);
            let path: &PathBuf = &rule.into();
            self.hw.unwatch(path)?
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
            let watcher_event = WatcherEvent::new(val);
            uslib::info!(LOGGER, "rule watcher: watcher event: {:?}\n", watcher_event);
        }
    }
}
