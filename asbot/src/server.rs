//! Adam Smith bot gRPC server.

use std::cell::RefCell;

use uslib::parking_lot::Mutex;
use uslib::proto::moodle_events_server::MoodleEvents;
use uslib::proto::moodle_events_server::MoodleEventsServer;
use uslib::proto::NotifyRequest;
use uslib::proto::NotifyResponse;
use uslib::tonic::transport::Server;
use uslib::tonic::Request;
use uslib::tonic::Response;
use uslib::tonic::Status;

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
pub struct GrpcServer {
    inner: RefCell<Server>,
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
        let server = SERVER.get().unwrap().lock();

        let mut inner = server.inner.borrow_mut();
        let router = inner.add_service(MoodleEventsServer::new(server.mevents.clone()));
        tokio::spawn(async {
            uslib::trace!(uslib::LOGGER, "grpc server: start: begin serve\n");
            router.serve("0.0.0.0:5555".parse().unwrap()).await
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
        uslib::trace!(uslib::LOGGER, "grpc server: moodle events: notify\n");
        Ok(Response::new(NotifyResponse {}))
    }
}
