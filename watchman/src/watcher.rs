//! File watcher.

use std::convert::TryFrom;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use actix::prelude::*;

use hotwatch::{Event, Hotwatch};

#[derive(Debug)]
pub struct ScraperRule(String);

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

/// An event raised by the watcher.
/// We only really care if the file changed, at this point.
pub struct WatcherEvent<'s> {
    rule_name: &'s str,
    cg_detected: std::time::Instant,
}

impl<'s> WatcherEvent<'s> {
    /// Create a new WatcherEvent.
    pub fn new(rule_name: &'s str) -> Self {
        Self {
            rule_name,
            cg_detected: std::time::Instant::now(),
        }
    }
}

pub struct WatcherActor {
    rules: Vec<ScraperRule>,
    hw: Hotwatch,
}

impl WatcherActor {
    pub fn new(rules: Vec<ScraperRule>) -> Self {
        Self {
            rules,
            hw: Hotwatch::new().expect("hotwatch failed to initialize!"),
        }
    }
}

impl Actor for WatcherActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // create the event handler lambda
        // this will log the event and send a message to the asbot client actor
        let event_handler = |event: Event| {
            let mut rule: Option<ScraperRule> = None;
            match event {
                Event::Create(path) => rule = Some(ScraperRule::try_from(path.as_path()).unwrap()),
                Event::Write(path) => rule = Some(ScraperRule::try_from(path.as_path()).unwrap()),
                Event::Error(err, path) => {
                    println!(
                        "Event::Error, path: {}, err: {}",
                        path.unwrap_or_default().display(),
                        err
                    );
                }
                _ => {
                    println!("event: {:?}", event);
                }
            }
            if let Some(val) = rule {
                let watcher_event = WatcherEvent::new(val);
                println!("rule updated: {:?}", watcher_event);
            }
        };

        // watch all rules
        for rule in self.rules.as_slice() {
            println!("Watching rule {:?}", rule);
            let path: &PathBuf = &rule.into();
            self.hw
                .watch(path, event_handler)
                .expect("failed to watch file!");
        }
    }
}
