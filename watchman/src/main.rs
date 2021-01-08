#[macro_use]
extern crate anyhow;

mod watcher;

use actix::prelude::*;

use actix_rt::signal;

use watcher::WatcherActor;

#[actix_rt::main]
async fn main() {
    let rules = std::env::var("USSCRAPER_RULES").unwrap();
    let rules_vec: Vec<watcher::ScraperRule> = rules
        .split(",")
        .map(|s| watcher::ScraperRule::new(s.to_string()))
        .collect();

    let addr = WatcherActor::new(rules_vec).start();

    signal::ctrl_c().await.unwrap();
}
