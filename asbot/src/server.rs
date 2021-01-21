//! Adam Smith bot gRPC server.

use uslib::common::*;

use crate::discord::Discord;

use std::str::FromStr;

use blockz::prelude::*;

use proto::moodle_events_server::MoodleEvents;
use proto::moodle_events_server::MoodleEventsServer;
use proto::NotifyRequest;
use proto::NotifyResponse;

use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;

/// Configuration for the gRPC server.
#[derive(Debug)]
struct GrpcServerConfig {
    port: u16,
}

impl GrpcServerConfig {
    /// Load the grpc server configuration.
    pub async fn load() -> anyhow::Result<Self> {
        slog::debug!(uslib::LOGGER, "grpc server config: load\n");
        let port: u16;
        match std::env::var("ASBOT_GRPC_PORT") {
            Ok(value) => port = u16::from_str(value.as_str())?,
            Err(e) => anyhow::bail!(
                "asbot client config: load: ASBOT_GRPC_PORT not found: {}\n",
                e
            ),
        }
        let config = Self { port };
        slog::trace!(uslib::LOGGER, "grpc server config: load: {:?}\n", config);
        Ok(config)
    }
}

#[derive(Debug, Singleton)]
pub struct GrpcServer {
    inner: Server,
    config: GrpcServerConfig,
    mevents: MoodleEventsService,
}

impl GrpcServer {
    /// Initialize the AsBotClient.
    pub async fn init() -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "grpc server: init\n");
        let config = GrpcServerConfig::load().await?;

        let inner = Server::builder();
        let mevents = MoodleEventsService;

        slog::trace!(uslib::LOGGER, "grpc server: init: setting up singleton\n");
        let grpc_server = Self {
            inner,
            config,
            mevents,
        };
        if let Err(e) = Self::init_singleton(grpc_server) {
            anyhow::bail!("grpc server: init: {}\n", e);
        };
        slog::trace!(uslib::LOGGER, "grpc server: init: singleton ok\n");

        slog::trace!(uslib::LOGGER, "grpc server: init: ok\n");
        Ok(())
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "grpc server: start\n");

        let router = self
            .inner
            .add_service(MoodleEventsServer::new(self.mevents.clone()));
        let port = self.config.port;
        tokio::spawn(async move {
            slog::trace!(uslib::LOGGER, "grpc server: start: begin serve\n");
            router
                .serve(format!("0.0.0.0:{}", port).parse().unwrap())
                .await
        });

        slog::trace!(uslib::LOGGER, "grpc server: start ok\n");
        Ok(())
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "grpc server: stop\n");
        slog::trace!(uslib::LOGGER, "grpc server: stop ok\n");
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct MoodleEventsService;

#[async_trait::async_trait]
impl MoodleEvents for MoodleEventsService {
    async fn notify(
        &self,
        request: Request<NotifyRequest>,
    ) -> Result<Response<NotifyResponse>, Status> {
        slog::trace!(
            uslib::LOGGER,
            "grpc server: moodle events: notify: {:?}\n",
            request
        );
        if let Err(e) = Discord::use_singleton_with_arg(
            Discord::execute_moodle_webhook,
            format!("Change detected: {}", request.get_ref().rule),
        )
        .await
        {
            slog::warn!(uslib::LOGGER, "grpc server: moodle events: notify: {}", e);
        }

        Ok(Response::new(NotifyResponse {}))
    }
}
