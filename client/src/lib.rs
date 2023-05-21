use futures::FutureExt;
use std::future::Future;

use communication::{forwarder_client::ForwarderClient, Invocation, MessagePack};
use serde::de::DeserializeOwned;
use tonic::{transport::Channel, Request, Response, Status};

mod communication {
    tonic::include_proto!("communication");
}

pub struct ConnectionHandle(ForwarderClient<Channel>);

impl ConnectionHandle {
    pub fn invoke_raw<'a>(
        &'a mut self,
        module: String,
        function_name: String,
        args: Option<MessagePack>,
    ) -> impl Future<Output = Result<Response<MessagePack>, Status>> + 'a {
        self.0.forward(Request::new(Invocation {
            module,
            function_name,
            args,
        }))
    }

    pub fn invoke<'a, TargetStruct>(
        &'a mut self,
        module: String,
        function_name: String,
        args: Option<MessagePack>,
    ) -> impl Future<Output = Result<TargetStruct, rmp_serde::decode::Error>> + 'a
    where
        TargetStruct: DeserializeOwned,
    {
        self.invoke_raw(module, function_name, args)
            .map(|response| {
                rmp_serde::from_slice::<TargetStruct>(
                    response.unwrap().into_inner().data.as_slice(),
                )
            })
    }
}

pub async fn connect(ip: String, port: String) -> anyhow::Result<ConnectionHandle> {
    // TODO: Spin up the server if it doesn't already exist
    // NOTE: Perhaps make server spinup a feature flag?
    Ok(ConnectionHandle(
        ForwarderClient::connect(ip + ":" + &port).await?,
    ))
}
