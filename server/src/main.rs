use communication::{
    forwarder_server::{Forwarder, ForwarderServer},
    Invocation, InvocationOverride, MessagePack, OverrideStatus,
};
use tonic::{transport::Server, Request, Response, Status};

mod communication {
    tonic::include_proto!("communication");
}

pub struct ForwarderService {}

#[tonic::async_trait]
impl Forwarder for ForwarderService {
    async fn forward(&self, request: Request<Invocation>) -> Result<Response<MessagePack>, Status> {
        let invocation = request.into_inner();
        Ok(Response::new(MessagePack {
            data: rmp_serde::to_vec(&("hello world!".to_string(),)).unwrap(),
        }))
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
