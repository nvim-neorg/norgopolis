mod stdio_service;
pub mod module_communication;
pub mod invoker_service;

use invoker_service::InvokerService;
use invoker_service::Service;
use tokio_stream::wrappers::ReceiverStream;
use module_communication::invoker_server::InvokerServer;
use stdio_service::StdioService;
use tonic::transport::Server;

pub use tonic::async_trait;
pub use tonic::{Status, Code};

pub struct Module {}

impl Module {
    pub async fn start(service: impl Service + Sync + Send + 'static) -> Result<(), anyhow::Error> {
        // TODO: Make configurable
        let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
        let stdio_service = StdioService { stdin, stdout };

        // TODO: Do this in a better way
        // `once_stream` doesn't work :/
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<StdioService, anyhow::Error>>(1);
        tx.send(Ok(stdio_service)).await?;

        Ok(Server::builder()
            .add_service(InvokerServer::new(InvokerService::new(service)))
            .serve_with_incoming(ReceiverStream::new(rx))
            .await?)
    }
}
