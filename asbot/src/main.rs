//! Adam Smith bot.

mod discord;
mod server;

#[tokio::main]
async fn main() {
    uslib::info!(uslib::LOGGER, "main: hello world\n");

    // initialization
    uslib::debug!(uslib::LOGGER, "main: initializing grpc server\n");
    if let Err(e) = server::GrpcServer::init().await {
        uslib::crit!(uslib::LOGGER, "main: initializing grpc server: {}\n", e);
        return;
    }
    uslib::debug!(uslib::LOGGER, "main: initializing discord bot\n");
    if let Err(e) = discord::DiscordBot::init().await {
        uslib::crit!(uslib::LOGGER, "main: initializing discord bot: {}\n", e);
        return;
    }

    // start
    uslib::debug!(uslib::LOGGER, "main: starting grpc server\n");
    if let Err(e) = server::GrpcServer::start().await {
        uslib::crit!(uslib::LOGGER, "main: starting grpc server: {}\n", e);
        return;
    }
    uslib::debug!(uslib::LOGGER, "main: starting discord bot\n");
    if let Err(e) = discord::DiscordBot::start().await {
        uslib::crit!(uslib::LOGGER, "main: starting discord bot: {}\n", e);
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
    if let Err(e) = server::GrpcServer::stop().await {
        uslib::crit!(uslib::LOGGER, "main: stopping grpc server: {}\n", e);
        return;
    }
    uslib::debug!(uslib::LOGGER, "main: stopping discord bot\n");
    if let Err(e) = discord::DiscordBot::stop().await {
        uslib::crit!(uslib::LOGGER, "main: stopping discord bot: {}\n", e);
        return;
    }

    // done

    uslib::info!(uslib::LOGGER, "goodbye\n");
}
