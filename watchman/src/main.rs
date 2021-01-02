mod watcher;

use actix::prelude::*;

use watcher::WatcherActor;

#[actix_rt::main]
async fn main() {
    WatcherActor::new(vec![]).start();
}
