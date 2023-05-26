use std::pin::Pin;

mod communication;

use communication::{
    forwarder_server::{Forwarder, ForwarderServer},
    Invocation, InvocationOverride, MessagePack, OverrideStatus,
};
use tonic::{
    codegen::futures_core::Stream, transport::Server, Request, Response, Status, Streaming,
};

mod subprocess;

pub struct ForwarderService {}

#[tonic::async_trait]
impl Forwarder for ForwarderService {
    type ForwardStream = Pin<Box<dyn Stream<Item = Result<MessagePack, Status>> + Send>>;

    async fn forward(
        &self,
        request: Request<Invocation>,
    ) -> Result<Response<Streaming<MessagePack>>, Status> {
        let invocation = request.into_inner();
        let module = match subprocess::new_subprocess(&invocation.module, &vec![]).await {
            Ok(value) => value,
            Err(err) => return Err(tonic::Status::new(tonic::Code::FailedPrecondition, err.to_string())),
        };
    }

    async fn r#override(
        &self,
        _request: Request<InvocationOverride>,
    ) -> Result<Response<OverrideStatus>, Status> {
        Ok(Response::new(OverrideStatus { status: 0 }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:62020".parse().unwrap();
    let forwarder_service = ForwarderService {};

    Server::builder()
        .add_service(ForwarderServer::new(forwarder_service))
        .serve(address)
        .await?;
    Ok(())
}
