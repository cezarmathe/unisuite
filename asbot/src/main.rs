//! Adam Smith bot.

#[tokio::main]
async fn main() {
    uslib::info!(uslib::LOGGER, "main: hello world\n");

    // initialization

    // start

    // wait for termination

    uslib::debug!(uslib::LOGGER, "main: waiting for termination signal\n");
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .unwrap()
        .recv()
        .await
        .unwrap();
    uslib::debug!(uslib::LOGGER, "main: received termination signal, proceeding with graceful shutdown\n");

    // graceful shutdown

    // done

    uslib::info!(uslib::LOGGER, "goodbye\n");
}
