#[macro_use]
extern crate anyhow;

mod config;
mod watcher;

use watcher::RuleWatcher;

use std::sync::Mutex;

use uslib::LOGGER;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rules = std::env::var("USSCRAPER_RULES").unwrap();
    let rules_vec: Vec<config::ScraperRule> = rules
        .split(",")
        .map(|s| config::ScraperRule::new(s.to_string()))
        .collect();

    uslib::info!(LOGGER, "hello world\n");

    let rule_watcher_mutex = Mutex::new(RuleWatcher::new(rules_vec)?);
    {
        let mut rule_watcher = rule_watcher_mutex.lock().unwrap();
        rule_watcher.start().await?;
    }

    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?
        .recv()
        .await
        .unwrap();

    {
        let mut rule_watcher = rule_watcher_mutex.lock().unwrap();
        rule_watcher.stop().await?;
    }

    uslib::info!(LOGGER, "goodbye\n");

    Ok(())
}
