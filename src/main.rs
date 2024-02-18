mod subprocess;

use norgopolis_protos::client_communication::{
    forwarder_server::{Forwarder, ForwarderServer},
    Invocation, InvocationOverride, MessagePack, OverrideStatus,
};
use norgopolis_protos::module_communication::invoker_client;
use std::{collections::HashMap, time::Duration};

use std::path::PathBuf;
use std::pin::Pin;
use tokio::{sync::Mutex, time::sleep};
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
    tx: tokio::sync::mpsc::UnboundedSender<()>,
}

impl ForwarderService {
    pub fn new(search_path: PathBuf, tx: tokio::sync::mpsc::UnboundedSender<()>) -> Self {
        ForwarderService {
            search_path,
            command_map: HashMap::new().into(),
            tx,
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

        let _ = self.tx.send(());

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

                    command_map
                        .insert(invocation.module, command.clone())
                        .unwrap_or(command)
                }
            }
        };

        // TODO: Negotiate capabilities with the module.

        let mut client = invoker_client::InvokerClient::new(module);

        client
            .invoke(norgopolis_protos::module_communication::Invocation {
                function_name: invocation.function_name,
                args: Some(norgopolis_protos::module_communication::MessagePack {
                    // TODO(vhyrro): Allow no arguments to be sent over the invocation
                    data: invocation
                        .args
                        .map(|val| val.data)
                        .unwrap_or(vec![b'{', b'}']),
                }),
            })
            .then(|future| async move {
                let mut stream = future?.into_inner();

                let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

                while let Some(message) = stream.message().await? {
                    if let Err(err) =
                        tx.send(Ok(norgopolis_protos::client_communication::MessagePack {
                            data: message.data,
                        }))
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

    // Keeps the instance of norgopolis alive.
    // On every succesful request from a client this will be filled,
    // preventing the application from shutting itself down due to a timeout.
    // TODO(vhyrro): Is there a better way of going around this, without the need for an unbounded
    // channel?
    let (tx, mut rx): (tokio::sync::mpsc::UnboundedSender<()>, _) =
        tokio::sync::mpsc::unbounded_channel();

    let forwarder_service = ForwarderService::new(data_dir, tx);

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(60 * 5)).await;

            if rx.recv().now_or_never().is_none() {
                // TODO: Exit graecfully, shut down all other subprocesses
                // (this means they must be stored in some sort of table,
                // or their PIDs copied somewhere).
                std::process::exit(0);
            }

            // Drain the remaining messages.
            while rx.recv().now_or_never().is_some() {};
        }
    });

    println!("ready");

    Server::builder()
        .add_service(ForwarderServer::new(forwarder_service))
        .serve(address)
        .await?;

    Ok(())
}
