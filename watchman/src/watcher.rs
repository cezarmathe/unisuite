//! File watcher.

use std::path::PathBuf;

use actix::prelude::*;

use hotwatch::{Hotwatch, Event};

pub struct WatcherActor {
    rules: Vec<String>,
    hw: Hotwatch,
}

impl WatcherActor {
    pub fn new(rules: Vec<String>) -> Self {
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
            match event {
                Event::Create(path) => {
                    println!("Event::Create, path: {}", path.display());
                },
                Event::Write(path) => {
                    println!("Event::Write, path: {}", path.display());
                },
                Event::Error(err, path) => {
                    println!("Event::Error, path: {}, err: {}", path.unwrap_or_default().display(), err);
                }
                _ => {
                    println!("event: {:?}", event);
                },
            }
        };

        // watch all rules
        for rule_name in self.rules.as_slice() {
            let rule_data_path = {
                // 80 = 20(/var/usscraper/data/) + 50(rule name length) + 10(/data.json)
                let mut rule_data_path = PathBuf::with_capacity(80);
                rule_data_path.push("/var/usscraper/data");
                rule_data_path.push(rule_name);
                rule_data_path.push("data.json");
                rule_data_path
            };
            self.hw.watch(rule_data_path, event_handler).expect("failed to watch file!");
        }
    }
}
