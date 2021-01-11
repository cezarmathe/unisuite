//! File watcher.

use crate::config::ScraperRule;

use std::convert::TryFrom;
use std::path::PathBuf;

use actix::prelude::*;

use hotwatch::{Event, Hotwatch};

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
