//! Adam Smith bot gRPC server.

use uslib::common::*;

use crate::discord::Discord;

use blockz::prelude::*;

use proto::moodle_events_server::MoodleEvents;
use proto::moodle_events_server::MoodleEventsServer;
use proto::NotifyRequest;
use proto::NotifyResponse;

use serde::Deserialize;

use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;

/// Configuration for the gRPC server.
#[derive(Configuration, Debug, Deserialize)]
pub struct GrpcServerConfig {
    port: u16,
}

#[derive(Debug, Singleton)]
pub struct GrpcServer {
    inner: Server,
    mevents: MoodleEventsService,
}

impl GrpcServer {
    /// Initialize the AsBotClient.
    pub async fn init() -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "grpc server: init\n");

        let inner = Server::builder();
        let mevents = MoodleEventsService;

        slog::trace!(uslib::LOGGER, "grpc server: init: setting up singleton\n");
        let grpc_server = Self {
            inner,
            mevents,
        };
        if let Err(e) = Self::init_singleton(grpc_server) {
            anyhow::bail!("grpc server: init: {}\n", e);
        };
        slog::trace!(uslib::LOGGER, "grpc server: init: singleton ok\n");

        slog::trace!(uslib::LOGGER, "grpc server: init: ok\n");
        Ok(())
    }

    pub async fn start(&mut self, config: GrpcServerConfig) -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "grpc server: start\n");

        let router = self
            .inner
            .add_service(MoodleEventsServer::new(self.mevents.clone()));
        let port = config.port;
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
