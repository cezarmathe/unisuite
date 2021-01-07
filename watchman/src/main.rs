mod watcher;

use actix::prelude::*;

use actix_rt::signal;

use watcher::WatcherActor;

#[actix_rt::main]
async fn main() {
    WatcherActor::new(vec![]).start();

    signal::ctrl_c().await.unwrap();
}
