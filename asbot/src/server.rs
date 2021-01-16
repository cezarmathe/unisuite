//! Adam Smith bot gRPC server.

use crate::discord::DISCORD;

use std::cell::RefCell;
use std::str::FromStr;

use uslib::proto::moodle_events_server::MoodleEvents;
use uslib::proto::moodle_events_server::MoodleEventsServer;
use uslib::proto::NotifyRequest;
use uslib::proto::NotifyResponse;
use uslib::tokio;
use uslib::tonic::transport::Server;
use uslib::tonic::Request;
use uslib::tonic::Response;
use uslib::tonic::Status;

use tokio::sync::Mutex;

/// gRPC server.
pub static SERVER: uslib::OnceCell<Mutex<GrpcServer>> = uslib::OnceCell::new();

/// Configuration for the gRPC server.
#[derive(Debug)]
struct GrpcServerConfig {
    port: u16,
}

impl GrpcServerConfig {
    /// Load the grpc server configuration.
    pub async fn load() -> uslib::Result<Self> {
        uslib::debug!(uslib::LOGGER, "grpc server config: load\n");
        let port: u16;
        match std::env::var("ASBOT_GRPC_PORT") {
            Ok(value) => port = u16::from_str(value.as_str())?,
            Err(e) => uslib::bail!(
                "asbot client config: load: ASBOT_GRPC_PORT not found: {}\n",
                e
            ),
        }
        let config = Self { port };
        uslib::trace!(uslib::LOGGER, "grpc server config: load: {:?}\n", config);
        Ok(config)
    }
}

#[derive(Debug)]
pub struct GrpcServer {
    inner: RefCell<Server>,
    config: GrpcServerConfig,
    mevents: MoodleEventsService,
}

impl GrpcServer {
    /// Initialize the AsBotClient.
    pub async fn init() -> uslib::Result<()> {
        uslib::trace!(uslib::LOGGER, "grpc server: init\n");
        let config = GrpcServerConfig::load().await?;

        let inner = Server::builder();
        let mevents = MoodleEventsService;

        uslib::trace!(uslib::LOGGER, "grpc server: init: setting up singleton\n");
        if SERVER
            .set(Mutex::new(GrpcServer {
                inner: RefCell::new(inner),
                config,
                mevents,
            }))
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
        let server = SERVER.get().unwrap().lock().await;

        let mut inner = server.inner.borrow_mut();
        let router = inner.add_service(MoodleEventsServer::new(server.mevents.clone()));
        let port = server.config.port;
        tokio::spawn(async move {
            uslib::trace!(uslib::LOGGER, "grpc server: start: begin serve\n");
            router
                .serve(format!("0.0.0.0:{}", port).parse().unwrap())
                .await
        });

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

#[derive(Clone, Debug)]
struct MoodleEventsService;

#[uslib::async_trait]
impl MoodleEvents for MoodleEventsService {
    async fn notify(
        &self,
        request: Request<NotifyRequest>,
    ) -> Result<Response<NotifyResponse>, Status> {
        uslib::trace!(
            uslib::LOGGER,
            "grpc server: moodle events: notify: {:?}\n",
            request
        );
        let discord = DISCORD.get().unwrap().lock().await;
        if let Err(e) = discord
            .execute_moodle_webhook(format!("Change detected: {}", request.get_ref().rule))
            .await
        {
            uslib::warn!(uslib::LOGGER, "err {}", e);
        }

        Ok(Response::new(NotifyResponse {}))
    }
}
