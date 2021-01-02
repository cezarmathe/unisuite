//! File watcher.

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
                    // send a message
                },
                Event::Write(path) => {
                    // send a message
                },
                Event::Error(err, path) => {
                    // log the error
                }
                _ => {
                    // log the event name
                },
            }
        };

        // watch all rules
        for rule in self.rules.as_slice() {
            // fixme 02/01/2021: proper path derived from rule
            self.hw.watch(rule, event_handler).expect("failed to watch file!");
        }
    }
}
