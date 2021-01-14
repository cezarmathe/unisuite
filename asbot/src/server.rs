//! Adam Smith bot gRPC server.

use uslib::parking_lot::Mutex;

/// gRPC server.
pub static SERVER: uslib::OnceCell<Mutex<GrpcServer>> = uslib::OnceCell::new();

/// Configuration for the gRPC server.
#[derive(Debug)]
struct GrpcServerConfig {}

impl GrpcServerConfig {
    /// Load the grpc server configuration.
    pub async fn load() -> uslib::Result<Self> {
        uslib::debug!(uslib::LOGGER, "grpc server config: load\n");
        let config = Self {};
        uslib::trace!(uslib::LOGGER, "grpc server config: load: {:?}\n", config);
        Ok(config)
    }
}

#[derive(Debug)]
pub struct GrpcServer {}

impl GrpcServer {
    /// Initialize the AsBotClient.
    pub async fn init() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "grpc server: init\n");
        let config = GrpcServerConfig::load().await?;

        uslib::trace!(uslib::LOGGER, "grpc server: init: setting up singleton\n");
        if SERVER
            .set(Mutex::new(GrpcServer {}))
            .is_err()
        {
            uslib::bail!("grpc server: init: already initialized\n");
        };
        uslib::trace!(uslib::LOGGER, "grpc server: init: singleton ok\n");

        uslib::trace!(uslib::LOGGER, "grpc server: init: ok\n");
        Ok(())
    }

    pub async fn start() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "grpc server: start\n");
        let server = SERVER.get().unwrap().lock();
        uslib::trace!(uslib::LOGGER, "grpc server: start ok\n");
        Ok(())
    }

    pub async fn stop() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "grpc server: stop\n");
        let server = SERVER.get().unwrap().lock();
        uslib::trace!(uslib::LOGGER, "grpc server: stop ok\n");
        Ok(())
    }
}
