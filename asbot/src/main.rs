//! Adam Smith bot.

#[macro_use]
extern crate uslib;

mod discord;
mod server;

use discord::Discord;

use server::GrpcServer;

use uslib::blockz::singleton::Singleton;
use uslib::tokio;

#[tokio::main]
async fn main() {
    uslib::info!(uslib::LOGGER, "main: hello world\n");

    // initialization
    uslib::debug!(uslib::LOGGER, "main: initializing grpc server\n");
    if let Err(e) = GrpcServer::init().await {
        uslib::crit!(uslib::LOGGER, "main: initializing grpc server: {}\n", e);
        return;
    }
    uslib::debug!(uslib::LOGGER, "main: initializing discord\n");
    if let Err(e) = Discord::init().await {
        uslib::crit!(uslib::LOGGER, "main: initializing discord: {}\n", e);
        return;
    }

    // start
    uslib::debug!(uslib::LOGGER, "main: starting grpc server\n");
    if let Err(e) = GrpcServer::use_mut_singleton(GrpcServer::start).await {
        uslib::crit!(uslib::LOGGER, "main: starting grpc server: {}\n", e);
        return;
    }
    uslib::debug!(uslib::LOGGER, "main: starting discord\n");
    if let Err(e) = Discord::use_mut_singleton(Discord::start).await {
        uslib::crit!(uslib::LOGGER, "main: starting discord: {}\n", e);
        return;
    }

    // wait for termination

    uslib::debug!(uslib::LOGGER, "main: waiting for termination signal\n");
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .unwrap()
        .recv()
        .await
        .unwrap();
    uslib::debug!(
        uslib::LOGGER,
        "main: received termination signal, proceeding with graceful shutdown\n"
    );

    // graceful shutdown
    uslib::debug!(uslib::LOGGER, "main: stopping grpc server\n");
    if let Err(e) = server::GrpcServer::use_mut_singleton(server::GrpcServer::stop).await {
        uslib::crit!(uslib::LOGGER, "main: stopping grpc server: {}\n", e);
        return;
    }
    uslib::debug!(uslib::LOGGER, "main: stopping discord\n");
    if let Err(e) = Discord::use_mut_singleton(Discord::stop).await {
        uslib::crit!(uslib::LOGGER, "main: stopping discord: {}\n", e);
        return;
    }

    // done

    uslib::info!(uslib::LOGGER, "goodbye\n");
}
