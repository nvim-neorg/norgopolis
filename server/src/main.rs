mod client_communication;
mod module_communication;
mod subprocess;

use client_communication::{
    forwarder_server::{Forwarder, ForwarderServer},
    Invocation, InvocationOverride, MessagePack, OverrideStatus,
};
use module_communication::invoker_client;

use std::pin::Pin;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{codegen::futures_core::Stream, transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct ForwarderService {}

#[tonic::async_trait]
impl Forwarder for ForwarderService {
    type ForwardStream = Pin<Box<dyn Stream<Item = Result<MessagePack, Status>> + Send>>;

    async fn forward(
        &self,
        request: Request<Invocation>,
    ) -> Result<Response<Self::ForwardStream>, Status> {
        let invocation = request.into_inner();

        // TODO: Don't spawn a new command if it's already running.
        let module = match subprocess::new_subprocess(invocation.module, vec![]).await {
            Ok(value) => value,
            Err(err) => {
                return Err(tonic::Status::new(
                    tonic::Code::FailedPrecondition,
                    err.to_string(),
                ))
            }
        };

        // TODO: Negotiate capabilities with the module.

        let mut client = invoker_client::InvokerClient::new(module);

        let response = client
            .invoke(module_communication::Invocation {
                function_name: invocation.function_name,
                args: Some(module_communication::MessagePack {
                    data: invocation.args.unwrap().data,
                }),
            })
            .await?;

        let mut stream = response.into_inner();

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(message) = stream.message().await.unwrap() {
                tx.send(Ok(client_communication::MessagePack { data: message.data }))
                    .expect("Failed to forward response to client!");
            }
        });

        Ok(Response::new(Box::pin(UnboundedReceiverStream::new(rx))))
    }

    // TODO: Implement
    async fn r#override(
        &self,
        _request: Request<InvocationOverride>,
    ) -> Result<Response<OverrideStatus>, Status> {
        Ok(Response::new(OverrideStatus { status: 0 }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Add clap, make this changeable
    let address = "127.0.0.1:62020".parse().unwrap();
    let forwarder_service = ForwarderService::default();

    Server::builder()
        .add_service(ForwarderServer::new(forwarder_service))
        .serve(address)
        .await?;

    Ok(())
}
