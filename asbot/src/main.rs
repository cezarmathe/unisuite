//! Adam Smith bot.

mod discord;
mod server;

use uslib::common::*;

use discord::Discord;

use blockz::prelude::*;

use server::GrpcServer;

#[tokio::main]
async fn main() {
    slog::info!(uslib::LOGGER, "main: hello world\n");

    // initialization
    slog::debug!(uslib::LOGGER, "main: initializing grpc server\n");
    if let Err(e) = GrpcServer::init().await {
        slog::crit!(uslib::LOGGER, "main: initializing grpc server: {}\n", e);
        return;
    }
    slog::debug!(uslib::LOGGER, "main: initializing discord\n");
    if let Err(e) = Discord::init().await {
        slog::crit!(uslib::LOGGER, "main: initializing discord: {}\n", e);
        return;
    }

    // start
    slog::debug!(uslib::LOGGER, "main: starting grpc server\n");
    if let Err(e) = GrpcServer::use_mut_singleton(GrpcServer::start).await {
        slog::crit!(uslib::LOGGER, "main: starting grpc server: {}\n", e);
        return;
    }
    slog::debug!(uslib::LOGGER, "main: starting discord\n");
    if let Err(e) = Discord::use_mut_singleton(Discord::start).await {
        slog::crit!(uslib::LOGGER, "main: starting discord: {}\n", e);
        return;
    }

    // wait for termination

    slog::debug!(uslib::LOGGER, "main: waiting for termination signal\n");
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .unwrap()
        .recv()
        .await
        .unwrap();
    slog::debug!(
        uslib::LOGGER,
        "main: received termination signal, proceeding with graceful shutdown\n"
    );

    // graceful shutdown
    slog::debug!(uslib::LOGGER, "main: stopping grpc server\n");
    if let Err(e) = server::GrpcServer::use_mut_singleton(server::GrpcServer::stop).await {
        slog::crit!(uslib::LOGGER, "main: stopping grpc server: {}\n", e);
        return;
    }
    slog::debug!(uslib::LOGGER, "main: stopping discord\n");
    if let Err(e) = Discord::use_mut_singleton(Discord::stop).await {
        slog::crit!(uslib::LOGGER, "main: stopping discord: {}\n", e);
        return;
    }

    // done

    slog::info!(uslib::LOGGER, "goodbye\n");
}
