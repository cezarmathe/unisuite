#[macro_use]
extern crate anyhow;

mod config;
mod watcher;

use actix::prelude::*;

use actix_rt::signal;

use watcher::WatcherActor;

#[actix_rt::main]
async fn main() {
    let rules = std::env::var("USSCRAPER_RULES").unwrap();
    let rules_vec: Vec<config::ScraperRule> = rules
        .split(",")
        .map(|s| config::ScraperRule::new(s.to_string()))
        .collect();

    let addr = WatcherActor::new(rules_vec).start();

    signal::ctrl_c().await.unwrap();
}
