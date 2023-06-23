mod client_communication;
mod module_communication;
mod subprocess;

use client_communication::{
    forwarder_server::{Forwarder, ForwarderServer},
    Invocation, InvocationOverride, MessagePack, OverrideStatus,
};
use module_communication::invoker_client;
use std::collections::HashMap;

use std::path::PathBuf;
use std::pin::Pin;
use tokio::sync::Mutex;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{
    codegen::futures_core::Stream,
    transport::{Channel, Server},
    Code, Request, Response, Status,
};

use futures::FutureExt;

pub struct ForwarderService {
    command_map: Mutex<HashMap<String, Channel>>,
    search_path: PathBuf,
}

impl ForwarderService {
    pub fn new(search_path: PathBuf) -> Self {
        ForwarderService {
            search_path,
            command_map: HashMap::new().into(),
        }
    }
}

#[tonic::async_trait]
impl Forwarder for ForwarderService {
    type ForwardStream = Pin<Box<dyn Stream<Item = Result<MessagePack, Status>> + Send>>;

    async fn forward(
        &self,
        request: Request<Invocation>,
    ) -> Result<Response<Self::ForwardStream>, Status> {
        let invocation = request.into_inner();

        // TODO: Don't spawn a new command if it's already running.
        let module = {
            let command_map = &mut self.command_map.lock().await;

            match command_map.get(&invocation.module) {
                Some(channel) => channel.to_owned(),
                None => {
                    let command = match subprocess::new_subprocess(
                        invocation.module.clone(),
                        vec![],
                        self.search_path.clone(),
                    )
                    .await
                    {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(tonic::Status::new(
                                tonic::Code::FailedPrecondition,
                                err.to_string(),
                            ))
                        }
                    };

                    // TODO: Can we prevent this clone?
                    command_map.insert(invocation.module, command.clone());

                    command
                }
            }
        };

        // TODO: Negotiate capabilities with the module.

        let mut client = invoker_client::InvokerClient::new(module);

        client
            .invoke(module_communication::Invocation {
                function_name: invocation.function_name,
                args: Some(module_communication::MessagePack {
                    data: invocation.args.unwrap().data,
                }),
            })
            .then(|future| async move {
                let mut stream = future?.into_inner();

                let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

                while let Some(message) = stream.message().await? {
                    if let Err(err) =
                        tx.send(Ok(client_communication::MessagePack { data: message.data }))
                    {
                        return Err(Status::new(Code::Cancelled, err.to_string()));
                    }
                }

                Ok(Response::new(
                    Box::pin(UnboundedReceiverStream::new(rx)) as Self::ForwardStream
                ))
            })
            .await
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
    let data_dir = directories::ProjectDirs::from("org", "neorg", "norgopolis").expect("Could not grab known data directories, are you running on a non-unix and non-windows system?").data_dir().join("modules");

    let _ = std::fs::create_dir_all(&data_dir);

    let forwarder_service = ForwarderService::new(data_dir);

    Server::builder()
        .add_service(ForwarderServer::new(forwarder_service))
        .serve(address)
        .await?;

    Ok(())
}
