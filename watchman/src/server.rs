//! Grpc server.

use uslib::common::*;

use std::convert::TryInto;
use std::net::SocketAddr;
use std::sync::Arc;

use blockz::prelude::*;

// use proto::scrape_newsletter_service_server::ScrapeNewsletterService;
// use proto::scrape_newsletter_service_server::ScrapeNewsletterServiceServer;
use proto::scrape_store_service_server::ScrapeStoreService;
use proto::scrape_store_service_server::ScrapeStoreServiceServer;
use proto::GetScrapeDataRequest;
use proto::GetScrapeDataResponse;
use proto::GetScrapeSumsRequest;
use proto::GetScrapeSumsResponse;
// use proto::SubscribeRequest;
// use proto::SubscribeResponse;

use serde::Deserialize;

use tokio::sync::Notify;

use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;

use types::Url;

/// Configuration for the gRPC server.
#[derive(Configuration, Debug, Deserialize)]
pub struct ScrapeStoreServerConfig {
    address: Url,
}

// /// Configuration for the gRPC server.
// #[derive(Configuration, Debug, Deserialize)]
// pub struct ScrapeNewsletterServerConfig {
//     address: Url,
// }

/// Scrape store gRPC server.
#[derive(Debug, Singleton)]
pub struct ScrapeStoreGrpcServer {
    inner: Server,
    shutdown: Arc<Notify>,
}

#[derive(Debug, Clone)]
pub struct ScrapeStoreServiceImpl;

// /// Scrape newsletter gRPC server.
// #[derive(Debug, Singleton)]
// pub struct ScrapeNewsletterGrpcServer {
//     inner: Server,
//     shutdown: Arc<Notify>,
// }

#[derive(Debug, Clone)]
pub struct ScrapeNewsletterServiceImpl;

impl ScrapeStoreGrpcServer {
    /// Initialize the AsBotClient.
    pub async fn init() -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "scrape store grpc server: init\n");

        let inner = Server::builder();

        slog::trace!(
            uslib::LOGGER,
            "scrape store grpc server: init: setting up singleton\n"
        );
        let grpc_server = Self {
            inner,
            shutdown: Arc::new(Notify::new()),
        };
        if let Err(e) = Self::init_singleton(grpc_server) {
            anyhow::bail!("scrape store grpc server: init: {}\n", e);
        };
        slog::trace!(
            uslib::LOGGER,
            "scrape store grpc server: init: singleton ok\n"
        );

        slog::trace!(uslib::LOGGER, "scrape store grpc server: init: ok\n");
        Ok(())
    }

    pub async fn start(&mut self, config: ScrapeStoreServerConfig) -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "scrape store grpc server: start\n");

        let router = self
            .inner
            .add_service(ScrapeStoreServiceServer::new(ScrapeStoreServiceImpl));
        let sockaddr: SocketAddr = (&config.address).try_into()?;
        let shutdown = self.shutdown.clone();
        tokio::spawn(async move {
            slog::trace!(
                uslib::LOGGER,
                "scrape store grpc server: start: begin serve\n"
            );
            router
                .serve_with_shutdown(sockaddr, shutdown.notified())
                .await
        });

        slog::trace!(uslib::LOGGER, "scrape store grpc server: start ok\n");
        Ok(())
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> {
        slog::trace!(uslib::LOGGER, "scrape store grpc server: stop\n");
        self.shutdown.notify_one();
        slog::trace!(uslib::LOGGER, "scrape store grpc server: stop ok\n");
        Ok(())
    }
}

// impl ScrapeNewsletterGrpcServer {
//     /// Initialize the AsBotClient.
//     pub async fn init() -> anyhow::Result<()> {
//         slog::trace!(uslib::LOGGER, "scrape newsletter grpc server: init\n");

//         let inner = Server::builder();

//         slog::trace!(
//             uslib::LOGGER,
//             "scrape newsletter grpc server: init: setting up singleton\n"
//         );
//         let grpc_server = Self {
//             inner,
//             shutdown: Arc::new(Notify::new()),
//         };
//         if let Err(e) = Self::init_singleton(grpc_server) {
//             anyhow::bail!("scrape newsletter grpc server: init: {}\n", e);
//         };
//         slog::trace!(
//             uslib::LOGGER,
//             "scrape newsletter grpc server: init: singleton ok\n"
//         );

//         slog::trace!(uslib::LOGGER, "scrape newsletter grpc server: init: ok\n");
//         Ok(())
//     }

//     pub async fn start(&mut self, config: ScrapeNewsletterServerConfig) -> anyhow::Result<()> {
//         slog::trace!(uslib::LOGGER, "scrape newsletter grpc server: start\n");

//         let router = self.inner.add_service(ScrapeNewsletterServiceServer::new(
//             ScrapeNewsletterServiceImpl,
//         ));
//         let sockaddr: SocketAddr = (&config.address).try_into()?;
//         let shutdown = self.shutdown.clone();
//         tokio::spawn(async move {
//             slog::trace!(
//                 uslib::LOGGER,
//                 "scrape newsletter grpc server: start: begin serve\n"
//             );
//             router
//                 .serve_with_shutdown(sockaddr, shutdown.notified())
//                 .await
//         });

//         slog::trace!(uslib::LOGGER, "scrape newsletter grpc server: start ok\n");
//         Ok(())
//     }

//     pub async fn stop(&mut self) -> anyhow::Result<()> {
//         slog::trace!(uslib::LOGGER, "scrape newsletter grpc server: stop\n");
//         self.shutdown.notify_one();
//         slog::trace!(uslib::LOGGER, "scrape newsletter grpc server: stop ok\n");
//         Ok(())
//     }
// }

#[async_trait::async_trait]
impl ScrapeStoreService for ScrapeStoreServiceImpl {
    async fn get_scrape_data(
        &self,
        request: Request<GetScrapeDataRequest>,
    ) -> Result<Response<GetScrapeDataResponse>, Status> {
        todo!();
    }

    async fn get_scrape_sums(
        &self,
        request: Request<GetScrapeSumsRequest>,
    ) -> Result<Response<GetScrapeSumsResponse>, Status> {
        todo!();
    }
}

// #[async_trait::async_trait]
// impl ScrapeNewsletterService for ScrapeNewsletterServiceImpl {
//     type SubscribeStream = SubscribeResponse;
//     async fn subscribe(
//         &self,
//         request: Request<SubscribeRequest>,
//     ) -> Result<Response<Self::SubscribeStream>, Status> {
//         todo!();
//     }
// }
